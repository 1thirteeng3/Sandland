# Research & Technical Decisions: Núcleo Local-First e Canvas Básico (v0.1)

**Feature**: `001-local-first-canvas`  
**Date**: 2026-09-21 (Revisão Técnica Canônica Pós-Análise de Integridade e Modelagem)  
**Status**: Completed & Approved  

---

## 1. Decisões Bloqueantes (B1 a B4)

### B1. Framework Frontend & State Management
- **Decisão**: **Svelte 5 (Runes `$state`, `$derived`, `$effect`) + TypeScript + Vite**.
- **Racional**:
  - Zero overhead de Virtual DOM e menor ocupação de RAM na WebView do Tauri v2.
  - Sincronização direta e reativa com o ciclo de vida do PixiJS (60 FPS) e projeção de overlays DOM.
  - Classes reativas nativas do Svelte 5 (`CanvasStore`, `IngestStore`) sem dependências pesadas de terceiros.

### B2. Runtime Assíncrono do Backend
- **Decisão**: **`tokio` (multi-threaded com feature `full`)**.
- **Racional**: Separação de threads de I/O de alta frequência (file watcher, sqlite) e threads de computação de tensores (SLM e embeddings) via `tokio::task::spawn_blocking`.

### B3. Estratégia de Download e Ciclo de Vida de Modelos (v0.1)
- **Decisão**: **Download sob demanda com barra de progresso no primeiro boot (Resilient Chunked Downloader)**.
- **Racional**:
  - Evita empacotar mais de 1.5 GB no instalador nativo.
  - Armazena pesos em `%APPDATA%\sandland\models\` (Windows) ou `~/.local/share/sandland/models/` (Linux).
  - Emite eventos assíncronos `model://download-progress` para a interface.
  - Verificação obrigatória de integridade SHA-256 antes da inicialização.

### B4. Contratos IPC Canônicos e Completos
- Documentados de forma exaustiva no artefato [`contracts/tauri-ipc.md`](contracts/tauri-ipc.md).

---

## 2. Solução de Arquitetura do Canvas: Renderização Híbrida WebGL + Projeção DOM

1. **Camada Gráfica Passiva (PixiJS v8 / WebGL)**:
   - Renderiza nós, contornos, cores de categoria e conexões com *Frustum Culling* e Level-of-Detail (LOD).
   - Suporta ancoragem direcional (`from_side`, `to_side`: `left | right | top | bottom`) para curvas bézier limpas sem cruzar cartões.
   - **Zero Duplicação de Dados**: O `CanvasNode` armazena apenas identificador, coordenadas espaciais e o ponteiro `item_id`. Título, resumo e tags são hidratados sob demanda a partir do cache do SQLite, evitando invalidação de cache.
2. **Camada de Edição Interativa (DOM Overlay em Svelte 5)**:
   - Ao dar duplo clique em um nó, o Svelte projeta um micro-editor Markdown (`<textarea>`) posicionado exatamente sobre as coordenadas projetadas da viewport.
   - Ao perder o foco (`blur`), o texto é sincronizado e o PixiJS retoma a renderização rasterizada da textura.

---

## 3. Motor de Embeddings Multilíngue Desacoplado: `fastembed`

- **Decisão**: O SLM generativo (`llama.cpp`) **NÃO** gera embeddings. A geração de vetores de 384 dimensões é realizada pela crate **`fastembed`** utilizando o modelo **`paraphrase-multilingual-MiniLM-L12-v2`** (ONNX Runtime local).
- **Racional**:
  - Modelo multilíngue nativo com suporte de alta fidelidade para Português e Inglês, eliminando a degradação semântica que ocorreria com o `bge-small-en-v1.5`.
  - Execução em CPU em < 15ms por documento com < 40 MB de memória RAM.
  - Operação 100% desacoplada do SLM generativo, permitindo busca vetorial contínua sem manter o modelo de 1.5B carregado na VRAM.

---

## 4. Conformidade Estrita com `sqlite-vec` (Tabela `vec0`) e Relações N:N

1. **Compatibilidade com `vec0`**:
   - A extensão `sqlite-vec` exige chaves primárias inteiras (`rowid` de 64 bits).
   - A tabela `items` possui a coluna `rowid_item INTEGER PRIMARY KEY AUTOINCREMENT`. A tabela virtual `vec_items` é vinculada diretamente a esse `rowid`, e as consultas unificadas usam `JOIN items i ON i.rowid_item = v.rowid`.
2. **Relacionamento N:N de Taxonomia**:
   - Criada a tabela de junção `item_taxonomy(item_id, term)` com índices compostos para permitir buscas e filtros relacionais instantâneos por tags sem operações custosas de `LIKE '%tag%'`.
3. **Sincronização com FTS5**:
   - A tabela `items_fts` é mantida em sincronia através de transações atômicas no worker em Rust (`items`, `items_fts`, `vec_items`, `item_taxonomy`).

---

## 5. Eliminação de Anti-Patterns no Paradigma *File-as-Truth*

1. **Eliminação de Metadados Extrínsecos do Frontmatter**:
   - `vault_path`: Removido do YAML Frontmatter. O caminho físico é um detalhe do sistema de arquivos; o identificador canônico é o `id`. O SQLite mapeia `id -> vault_path` dinamicamente a partir dos eventos do `notify`.
   - `content_hash`: Removido do YAML Frontmatter para eliminar o paradoxo de autorreferência. O hash SHA-256 do arquivo é gerenciado e persistido estritamente no `index.db`.
2. **Prevenção de Sobrecarga de I/O no `audit_log.db`**:
   - O evento `NODE_MOVED` **nunca** é registrado a 60 FPS durante o arraste.
   - Mudanças instantâneas de posição são persistidas unicamente em `board.canvas.mpk` (MessagePack a cada 350ms).
   - O `audit_log.db` recebe o evento `NODE_DRAG_END` apenas no término do movimento (debounced), reservando o banco de auditoria para transações semânticas reais.

---

## 6. Prompt de Sistema do SLM & Gramática GBNF

### 6.1. Prompt do Taxonomista
```text
<|im_start|>system
Você é o Assistente Taxonômico do Sandland, responsável por catalogar documentos analíticos em um cofre local.
Sua missão é ler o documento fornecido e gerar uma classificação concisa em formato JSON estruturado.

Diretrizes Obrigatórias:
1. Resumo ("summary"): Elabore um resumo conciso do documento em exatamente uma frase (máximo de 25 palavras).
2. Categoria ("category"): Atribua uma categoria temática ampla em formato kebab-case e singular (ex: engenharia-de-sistemas, neurociencia, filosofia-politica).
3. Tags ("tags"): Identifique de 2 a 5 tags específicas em kebab-case e no singular (ex: memoria, concorrencia, rust).
4. Reutilização de Vocabulário: Você receberá uma lista de tags já existentes no cofre. Se o conceito corresponder a um termo existente, REUTILIZE-O obrigatoriamente para evitar duplicações semânticas. Crie novos termos apenas se houver uma lacuna conceitual clara.
<|im_end|>
<|im_start|>user
[VOCABULÁRIO EXISTENTE NO COFRE]:
["rust", "concorrencia", "design-systems", "sistemas-distribuidos", "aprendizado-de-maquina"]

[DOCUMENTO A CLASSIFICAR]:
{TEXTO_DO_DOCUMENTO}
<|im_end|>
<|im_start|>assistant
```

### 6.2. Gramática GBNF
```gbnf
root   ::= object
object ::= "{\n" "  \"summary\": " string ",\n" "  \"category\": " string ",\n" "  \"tags\": " taglist "\n}"
taglist ::= "[\n" (string (",\n" string)*)? "\n  ]"
string  ::= "\"" ([^"\\] | "\\" (["\\/bfnrt] | "u" [0-9a-fA-F] [0-9a-fA-F] [0-9a-fA-F] [0-9a-fA-F]))* "\""
```
