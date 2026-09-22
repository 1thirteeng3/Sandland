# Arquitetura Técnica do Sandland

**Status:** Aprovado / Canônico  
**Versão:** 0.2.0-draft (Alinhado à v0.1 e pavimentando a v0.2)  
**Plataformas:** Windows (x86_64), macOS (Apple Silicon / Intel), Linux (x86_64, aarch64)  
**Paradigmas Centrais:** *Local-First*, Soberano, *File-as-Truth*, Separação Cognitiva de Motores

---

## 1. Visão Geral do Sistema

O **Sandland** é um ambiente computacional unificado para captura, exploração relacional e produção analítica de conhecimento. Ele rejeita a premissa de que toda informação deve ser reduzida a fragmentos textuais opacos (chunks) indexados em bancos vetoriais, adotando uma **arquitetura de dois motores cognitivos complementares**:

```
                                ARQUITETURA SANDLAND
 ┌─────────────────────────────────────────────────────────────────────────────────────────┐
 │                                   INTERFACE DO USUÁRIO                                  │
 │   ┌──────────────────────────────────────────────┐ ┌─────────────────────────────────┐  │
 │   │   Acervo & Ingestão (Gaveta Lateral)         │ │   Mesa de Pesquisa / Canvas     │  │
 │   │   - Busca Léxica & Semântica Híbrida         │ │   - Visualização Espacial       │  │
 │   │   - Drag-and-Drop & Seleção de Arquivos      │ │   - Agrupamento & Arestas       │  │
 │   │   - Captura de URLs Web                      │ │   - Renderização PixiJS/WebGL   │  │
 │   └──────────────────────┬───────────────────────┘ └────────────────┬────────────────┘  │
 └──────────────────────────┼──────────────────────────────────────────┼───────────────────┘
                            │ IPC Tauri v2                             │
 ┌──────────────────────────▼──────────────────────────────────────────▼───────────────────┐
 │                             BACKEND NATIVO (RUST CORE)                                  │
 │                                                                                         │
 │   ┌─────────────────────────────────────────────┐ ┌──────────────────────────────────┐  │
 │   │ MOTOR 1: ACERVO DE DOCUMENTOS BRUTOS        │ │ MOTOR 2: MESA / CANVAS (RLM)     │  │
 │   │ - Pipeline Híbrido: Vetorial + Lexical      │ │ - Vectorless RAG (PageIndex)     │  │
 │   │ - SQLite `fts5` (unicode61)                 │ │ - Topologia Semântica Estrutural │  │
 │   │ - `sqlite-vec` (FastEmbed / ONNX 384d)      │ │ - Geração em Runtime do          │  │
 │   │ - Reciprocal Rank Fusion (RRF k=60)         │ │   Skeleton Map (Árvore/Resumos)  │  │
 │   │ - File-as-Truth (`/ingest/*.md` + frontm.)  │ │ - Serialização `topology.json`   │  │
 │   └─────────────────────────────────────────────┘ └──────────────────────────────────┘  │
 │                                                                                         │
 │   ┌──────────────────────────────────────────────────────────────────────────────────┐  │
 │   │ CAMADA DE IA AGNOSTICA (Trait `ModelProvider` & BYOK)                            │  │
 │   │ - `LocalLlamaProvider`: Execução offline via pesos GGUF locais (CPU/GPU)         │  │
 │   │ - `CloudApiProvider`: OpenAI, Anthropic, OpenRouter, Together (Streaming/JSON)   │  │
 │   └──────────────────────────────────────────────────────────────────────────────────┘  │
 │                                                                                         │
 │   ┌──────────────────────────────────────────────────────────────────────────────────┐  │
 │   │ SANDBOX & PORTABILIDADE MULTIPLATAFORMA                                          │  │
 │   │ - Resolução Dinâmica de Diretórios via OS (`app.path().home_dir()`)              │  │
 │   │ - Proibição estrita de caminhos absolutos hardcoded                              │  │
 │   │ - VaultGuard: Confinamento em jaula com validação canônica de prefixos           │  │
 │   └──────────────────────────────────────────────────────────────────────────────────┘  │
 └──────────────────────────────────────────┬──────────────────────────────────────────────┘
                                            │ I/O Local Confinado
 ┌──────────────────────────────────────────▼──────────────────────────────────────────────┐
 │                                   SISTEMA DE ARQUIVOS (VAULT)                           │
 │   /vault/                                                                               │
 │   ├── .system/                     <-- Índices efêmeros descartáveis (`index.db`)       │
 │   ├── ingest/                      <-- Documentos brutos (File-as-Truth)                │
 │   └── workspaces/<board-id>/       <-- Mesas de trabalho (`topology.json`, rascunhos)  │
 └─────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Separação Estrutural de Motores de Conhecimento

### Motor 1: Acervo de Documentos Brutos (Ingestão & Recuperação Global)
- **Finalidade:** Absorção passiva e consulta em acervos extensos (notas longas, transcrições, PDFs e snapshots web).
- **Abordagem de Busca:** Pipeline híbrido combinando busca vetorial densa (`sqlite-vec` compilado estaticamente via C amalgamation) e busca lexical BM25 (`fts5` com tokenizador `unicode61`), unificadas via algoritmo *Reciprocal Rank Fusion* (RRF, com $k = 60$).
- **Embeddings:** Geração local via `fastembed` (modelo multilíngue `paraphrase-multilingual-MiniLM-L12-v2`, 384 dimensões), operando sob demanda em lotes de baixa prioridade.
- **Armazenamento:** Cada item ingerido persiste como um arquivo Markdown físico legível em `ingest/notes/`, `ingest/web/` ou `ingest/media/` com YAML Frontmatter padronizado.

### Motor 2: Mesa de Pesquisa / Canvas (Exploração RLM & Vectorless RAG)
- **Finalidade:** Raciocínio espacial, síntese, correlação visual e elaboração conceitual.
- **Paradigma:** **Vectorless RAG** baseado na teoria de *Relational Language Modeling* (RLM) e no padrão *PageIndex*.
- **Por que sem vetores no Canvas?** 
  - Fragmentar nós de um quadro em pedaços desconexos de 512 tokens e vetorizá-los destrói o contexto semântico fundamental proporcionado pela posição espacial, hierarquia de grupos, proximidade e arestas conceituais.
  - As relações explícitas estabelecidas pelo usuário (A conecta em B; C está agrupado com D) são determinísticas e estruturadas.
- **Mecanismo Operacional:**
  - A mesa inteira é serializada como um grafo de topologia (`topology.json`).
  - O orquestrador de IA sintetiza o estado do board em tempo de execução através de um **Skeleton Map** (mapa em árvore de seções, grupos, nós, arestas e resumos textuais), que é injetado diretamente na janela de contexto dos modelos de linguagem.

---

## 3. Portabilidade e Padrões Open Source

Para assegurar que o Sandland funcione sem atritos em qualquer distribuição Linux, macOS e Windows:

1. **Proibição Estrita de Caminhos Hardcoded:**
   - Nenhum caminho de sistema de arquivos pode ser codificado de forma estática no código-fonte Rust, JavaScript ou scripts de build (ex: banimento absoluto de referências diretas a `C:\Users\...` ou `/home/...`).
2. **Resolução Dinâmica de Diretórios:**
   - A raiz do cofre padrão e qualquer pasta temporária ou de cache DEVE ser resolvida em tempo de execução através das APIs canônicas do Tauri:
     ```rust
     let user_home = app.path().home_dir().map_err(|e| format!("Falha ao resolver pasta de usuário: {e}"))?;
     let vault_root = user_home.join("SandlandVault");
     ```
3. **Isolamento e Segurança (Sandbox Guard):**
   - Toda escrita e leitura de arquivos dentro do cofre é intermediada pela estrutura `VaultGuard`, que normaliza caminhos via `canonicalize()` e verifica se o prefixo de destino reside estritamente dentro da raiz do cofre ativo, rejeitando qualquer fuga (*directory traversal* ou symlink malicioso).
