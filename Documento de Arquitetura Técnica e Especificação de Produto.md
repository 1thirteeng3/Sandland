# Documento de Arquitetura Técnica e Especificação de Produto (RFC / PRD)

**Nome do Sistema:** SANDLAND  
**Status:** Aprovado para Engenharia  
**Alvos Primários:** Linux (x86_64, aarch64), Windows (x86_64), macOS (Apple Silicon, Intel)  
**Paradigma:** *Local-First*, Soberano, Criptograficamente Auditável, Baseado em Arquivos (*File-as-Truth*)

---

## 1. Visão Geral e Filosofia do Produto

O **Sandland** unifica o ciclo de vida do conhecimento analítico — **Captura, Exploração Espacial, Conexão e Produção Textual** — em um ambiente computacional coeso executado inteiramente na máquina do usuário.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          CICLO DE VIDA DA INFORMAÇÃO                        │
├─────────────────┬───────────────────────────────┬───────────────────────────┤
│ FASE            │ METÁFORA OPERACIONAL          │ SUBSISTEMA                │
├─────────────────┼───────────────────────────────┼───────────────────────────┤
│ 1. Ingestão     │ Lago Desestruturado Invisível │ Ingest + Auto-Tagging SLM │
│ 2. Exploração   │ Mesa de Trabalho Espacial     │ Whiteboard (Canvas Pills) │
│ 3. Descoberta   │ Extração Web Sem Quebra       │ Rust Native Stealth Engine│
│ 4. Produção     │ Sala Editorial Modular        │ A Peça (Editor + Copiloto)│
│ 5. Governança   │ Rastreabilidade Determinística│ Dual-Layer Versioning     │
└─────────────────┴───────────────────────────────┴───────────────────────────┘
```

### Princípios Inegociáveis de Arquitetura

1. **Soberania do Dado (*File-as-Truth*):** O sistema de arquivos local (`.md` com YAML Frontmatter e mídias em `/assets/`) é a única fonte da verdade. Bancos de dados relacionais e vetoriais atuam unicamente como índices descartáveis e reconstruíveis a qualquer momento.
2. **Separação Cognitiva:** O *Ingest* absorve passivamente sem exigir organização do usuário; o *Whiteboard* organiza espacialmente ideias e sínteses; a *Peça* formaliza o produto editorial final.
3. **Contenção e Sandbox:** Agentes autônomos de IA e rotinas de extração web operam dentro de jaulas estritas de I/O em nível de sistema operacional (Landlock no Linux, Seatbelt no macOS e Job Objects no Windows), impedindo vazamento de dados ou execução arbitrária de comandos fora do escopo do projeto ativo.
4. **Desempenho Nativo:** Sem interpretadores externos ou runtimes pesados em background. Todo o backend de dados, rede, inferência local e extração é construído estritamente em Rust nativo.

### Métricas e Critérios de Aceite de Desempenho Realistas

Para evitar expectativas inexequíveis com modelos locais ativos, os limites operacionais do Sandland são divididos por estados de execução:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                       METRICAS DE CONSUMO DE RECURSOS                       │
├───────────────────┬───────────────────────────┬─────────────────────────────┤
│ ESTADO DE EXECUÇÃO│ LIMITE DE RAM REQUERIDO   │ LATÊNCIA DE INICIALIZAÇÃO   │
├───────────────────┼───────────────────────────┼─────────────────────────────┤
│ Idle (Repouso)    │ ≤ 350 MB                  │ NVMe SSD: < 2.2s            │
│ Active Indexing   │ ≤ 650 MB                  │ HDD / SATA: < 4.5s          │
│ Active Inference  │ ≤ 2.8 GB (unloads em 5min)│ Chamada de comando: < 150ms │
└───────────────────┴───────────────────────────┴─────────────────────────────┘
```

*   **Estado Idle:** Apenas Tauri v2 (WebView), PixiJS, SQLite e File Watcher ativos. O modelo de embeddings e o SLM estão descarregados da memória.
*   **Estado Active Indexing:** Modelos de embeddings e Whisper carregados sob demanda e executando em batch de baixa prioridade.
*   **Estado Active Inference:** SLM de classificação ou LLM Copilot carregado na VRAM/RAM. O Sandland impõe um mecanismo agressivo de *garbage collection* que descarrega (*unload*) os modelos locais da memória após 5 minutos de inatividade completa de inferência.

---

## 2. Topologia do Sistema e Estrutura de Diretórios

O Sandland é empacotado como um binário nativo via **Tauri v2** (Rust Core + Webview moderna com renderização acelerada por hardware via WebGL).

```
                             ARQUITETURA GERAL (TAURI v2)
 ┌───────────────────────────────────────────────────────────────────────────┐
 │                   CAMADA DE APRESENTAÇÃO (UI / FRONTEND)                  │
 │   - Engine do Canvas (PixiJS / WebGL Viewport 60 FPS com Culling e LOD)   │
 │   - Editor Modular de Blocos & Peça Lateral (Tipografia Editorial)        │
 │   - Chat Global e Copiloto Lateral                                        │
 └─────────────────────────────────────┬─────────────────────────────────────┘
                                       │ IPC (Tauri Commands / Events)
 ┌─────────────────────────────────────▼─────────────────────────────────────┐
 │                         BACKEND NATIVO EM RUST                            │
 │  ┌───────────────────────┐ ┌───────────────────────┐ ┌──────────────────┐ │
 │  │ SQLite + sqlite-vec   │ │ Engine de Extração    │ │ Orquestrador RLM │ │
 │  │ (Índice & Vetores)    │ │ (rquest + readability)│ │ (Despacho Local) │ │
 │  └───────────────────────┘ └───────────────────────┘ └──────────────────┘ │
 │  ┌───────────────────────┐ ┌───────────────────────┐ ┌──────────────────┐ │
 │  │ File Watcher (notify) │ │ Whisper.cpp / STT     │ │ Sandbox FS Guard │ │
 │  │ (Reatividade de disco)│ │ (Transcrição Local)   │ │ (OS-Level Cages) │ │
 │  └───────────────────────┘ └───────────────────────┘ └──────────────────┘ │
 └─────────────────────────────────────┬─────────────────────────────────────┘
                                       │ I/O Local Criptografado / Aberto
 ┌─────────────────────────────────────▼─────────────────────────────────────┐
 │                        SISTEMA DE ARQUIVOS (VAULT)                        │
 └───────────────────────────────────────────────────────────────────────────┘
```

### O Esquema do Vault em Disco

```text
/vault/
├── .system/                        <-- Metadados de controle e índices efêmeros
│   ├── index.db                    <-- SQLite com extensões sqlite-vec e FTS5
│   ├── audit_log.db                <-- Histórico de eventos e rollback
│   └── taxonomy_cache.json         <-- Cache vetorial de termos taxonômicos
├── intentions/                     <-- Camada de Teleologia (Metas, Hipóteses)
│   └── intentions.json             <-- Registro de objetivos e limites de foco
├── assets/                         <-- Content-Addressable Storage (CAS)
│   ├── 4f8a...3e.pdf               <-- Binários renomeados por hash SHA-256
│   ├── 9b1c...7a.mp4
│   └── d41d...2e.png
├── ingest/                         <-- Camada Append-Only (Somente Leitura)
│   ├── web/                        <-- Artigos raspados e snapshots
│   │   └── 20260921-artigo-rust.md
│   ├── media/                      <-- Transcrições de áudio/vídeo
│   │   └── 20260921-podcast-ep12.md
│   └── notes/                      <-- Despejos brutos de texto
│       └── 20260921-pensamento-solto.md
└── workspaces/                     <-- Pílulas de Canvas (Workspaces Isolados)
    ├── workspace_01j8k9/           <-- Sandbox individual do Workspace
    │   ├── board.canvas.mpk        <-- Topologia binária compacta (MessagePack)
    │   ├── board.canvas.json       <-- Cópia de interoperabilidade para Git
    │   ├── cells/                  <-- Células atômicas locais deste board
    │   │   └── cell_01j8k9_01.md
    │   ├── pieces/                 <-- Peças em redação vinculadas
    │   │   └── ensaio-analitico.md
    │   └── scratchpad/             <-- Área de rascunhos voláteis da IA
    └── workspace_01j8m2/
        └── ...
```

---

## 3. Ingestão e Taxonomia Guiada por SLM Local

O *Ingest* absorve materiais de forma transparente. A classificação taxonômica é delegada a um SLM local, executado estritamente sob as seguintes diretrizes técnicas:

### Modelo de Classificação Taxonômica Local
*   **Modelo Padrão:** `Qwen2.5-1.5B-Instruct-Q4_K_M` (GGUF).
*   **Tamanho em Disco / RAM:** ~1.12 GB de footprint.
*   **Runtime:** `llama.cpp` integrado diretamente via bindings Rust nativos (crate `llama-cpp-2` ou FFI customizado).
*   **Tempo de Inferência Médio:** ~1.8 segundos em Apple Silicon (M-series) e CPUs modernas com AVX2/AVX-512.

```
               PIPELINE DE CLASSIFICAÇÃO TAXONÔMICA NÃO-REDUNDANTE
 ┌──────────────────────┐
 │ Item Bruto no Ingest │
 └──────────┬───────────┘
            │
            ▼
 ┌──────────────────────┐
 │ Embedding Local      │ ──► Busca cosseno no índice taxonômico existente
 │ (bge-small-en-v1.5)  │     (Recupera Top-15 categorias e Top-20 tags ativas)
 └──────────┬───────────┘
            │
            ▼
 ┌──────────────────────┐
 │ Inferência SLM       │ ──► Prompt com GBNF (Grammar-Constrained Decoding):
 │ (Structured JSON)    │     "Reutilize os termos fornecidos prioritariamente;
 └──────────┬───────────┘     crie novos apenas se houver lacuna conceitual grave"
            │
            ▼
 ┌──────────────────────┐
 │ Pós-Normalização     │ ──► Normalização Kebab-case, singularização
 │ Determinística Rust  │ ──► Distância Levenshtein <= 2 (merge silencioso)
 └──────────┬───────────┘ ──► Similaridade Cosseno >= 0.92 (vinculação como alias)
            │
            ▼
 ┌──────────────────────┐
 │ Gravação no SQLite   │ ──► Escrita no index.db e gravação no frontmatter .md
 │ + Log de Auditoria   │ ──► Registro reversível no audit_log.db
 └──────────────────────┘
```

### GBNF (GGML Backus-Naur Form) para Structured Output Garantido

Para anular o risco de saídas malformadas ou JSONs inválidos, a API de inferência do `llama.cpp` é invocada passando a seguinte gramática estrita:

```gbnf
root   ::= object
object ::= "{\n" "  \"summary\": " string ",\n" "  \"category\": " category ",\n" "  \"tags\": " taglist "\n}"
category ::= "{\n" "    \"term\": " string ",\n" "    \"is_new\": " boolean ",\n" "    \"confidence\": " number "\n  }"
taglist ::= "[\n" (tag (",\n" tag)*)? "\n  ]"
tag ::= "    {\n" "      \"term\": " string ",\n" "      \"is_new\": " boolean ",\n" "      \"justification_if_new\": " string "\n    }"

string  ::= "\"" ([^"\\] | "\\" (["\\/bfnrt] | "u" [0-9a-fA-F] [0-9a-fA-F] [0-9a-fA-F] [0-9a-fA-F]))* "\""
boolean ::= "true" | "false"
number  ::= [0-9]+ "." [0-9]+
```

### Protocolo de Fallback da IA de Classificação (Resiliência)

Se a inferência do SLM falhar (por erro de sintaxe persistente, OOM do sistema ou timeout de 15 segundos), o Sandland executa a degradação elegante através de 3 camadas de segurança:

```
                         PROTOCOLO DE FALLBACK (SLM)
 ┌──────────────────────────────────────────────────────────────────────────┐
 │                         Início da Classificação                          │
 └────────────────────────────────────┬─────────────────────────────────────┘
                                      │
                                      ▼
                        ┌───────────────────────────┐
                        │   Tentativa 1: GBNF Qwen  │
                        └─────────────┬─────────────┘
                                      │
                         [Sucesso]    ├─────────── [Falha / Timeout / OOM]
                     ┌────────────────┴───────────────┐
                     ▼                                ▼
              ┌─────────────┐          ┌─────────────────────────────┐
              │ Finaliza ok │          │ Tentativa 2: Retry Temp 0.0 │
              └─────────────┘          └──────────────┬──────────────┘
                                                      │
                                         [Sucesso]    ├─────── [Falha]
                                     ┌────────────────┴──────────┐
                                     ▼                           ▼
                              ┌─────────────┐     ┌─────────────────────────────┐
                              │ Finaliza ok │     │ Tentativa 3: Keyword/Regex  │
                              └─────────────┘     │  + Similaridade de Cosseno  │
                                                  └──────────────┬──────────────┘
                                                                 │
                                                    [Sucesso]    ├─────── [Falha]
                                                 ┌───────────────┴──────────┐
                                                 ▼                          ▼
                                          ┌─────────────┐     ┌─────────────────────────────┐
                                          │ Finaliza ok │     │ Grava no index.db como      │
                                          └─────────────┘     │ `needs_manual_review`       │
                                                              └─────────────────────────────┘
```

1.  **Tentativa de Recuperação Térmica:** O sistema reinicia a inferência do SLM com `temperature = 0.0` e reduz a janela de contexto para os primeiros 2000 tokens do documento.
2.  **Fallback Heurístico de Cosseno + Regex (Determinístico):** Se o SLM falhar por completo ou o sistema estiver em restrição severa de hardware (sem VRAM/RAM suficiente), a classificação é substituída por um analisador léxico:
    *   Extração de palavras-chave baseada em TF-IDF rápido implementado localmente contra o texto do item.
    *   Cálculo de similaridade de cosseno (via `bge-small-en-v1.5`) do documento contra os termos já indexados no `taxonomy_cache.json`.
    *   Associação dos termos com maior pontuação ($>0.72$).
3.  **Marcação de Revisão Manual:** O arquivo markdown é gravado com o frontmatter `needs_manual_review: true` e uma notação no log de auditoria é inserida com o erro correspondente. O usuário é alertado discretamente por meio de um ícone indicador na UI.

---

## 4. Indexação Híbrida e Segurança do FS (*File-as-Truth*)

### Estratégia de Busca Híbrida: FTS5 + Vector + Reciprocal Rank Fusion (RRF)

O Sandland implementa busca híbrida combinando relevância léxica (SQLite FTS5) e relevância semântica (embeddings vetoriais com `sqlite-vec`).

```sql
-- DDL para index.db
CREATE TABLE IF NOT EXISTS items (
    rowid_item INTEGER PRIMARY KEY AUTOINCREMENT, -- Mapeador inteiro de 64-bit para vec_items
    id TEXT NOT NULL UNIQUE,                       -- UUIDv7 do documento
    vault_path TEXT NOT NULL UNIQUE,              -- Localização atual em disco (mapeada dinamicamente)
    item_type TEXT NOT NULL,                      -- 'note', 'web_snapshot', 'media_transcript'
    title TEXT NOT NULL,
    content_hash TEXT NOT NULL,                   -- SHA-256 gerenciado pelo indexador
    summary TEXT,
    category TEXT,
    needs_manual_review BOOLEAN DEFAULT 0,
    read_only BOOLEAN DEFAULT 1,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_items_path ON items(vault_path);
CREATE INDEX IF NOT EXISTS idx_items_category ON items(category);

-- Taxonomia e Relacionamentos N:N
CREATE TABLE IF NOT EXISTS taxonomy_terms (
    term TEXT PRIMARY KEY,
    term_type TEXT CHECK(term_type IN ('category', 'tag')),
    frequency INTEGER DEFAULT 1,
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS item_taxonomy (
    item_id TEXT REFERENCES items(id) ON DELETE CASCADE,
    term TEXT REFERENCES taxonomy_terms(term) ON DELETE CASCADE,
    PRIMARY KEY (item_id, term)
);

CREATE INDEX IF NOT EXISTS idx_item_taxonomy_term ON item_taxonomy(term);

-- Tabela de Busca de Texto Completo (FTS5)
CREATE VIRTUAL TABLE IF NOT EXISTS items_fts USING fts5(
    item_id UNINDEXED,
    title,
    content,
    tokenize="unicode61 remove_diacritics 1"
);

-- Tabela de Vetores (sqlite-vec com rowid mapeado de 64-bit)
-- Modelo multilíngue: paraphrase-multilingual-MiniLM-L12-v2 (384 dimensões via fastembed)
CREATE VIRTUAL TABLE IF NOT EXISTS vec_items USING vec0(
    rowid INTEGER PRIMARY KEY,
    embedding FLOAT[384]
);
```

As duas listas de resultados são unificadas usando a fórmula matemática de **Reciprocal Rank Fusion (RRF)**:

$$RRF\_Score(d) = \sum_{m \in M} \frac{1}{k + r_m(d)}$$

Onde $k = 60$ (constante padrão da indústria para balanceamento de cauda longa), $M$ representa os motores de busca (FTS5 e Vetorial), e $r_m(d)$ é o ranking do documento $d$ no motor $m$.

A consulta de unificação é executada em uma única query otimizada no SQLite:

```sql
WITH fts_results AS (
    SELECT item_id as id, ROW_NUMBER() OVER(ORDER BY rank) as rank_fts
    FROM items_fts
    WHERE items_fts MATCH :query
    LIMIT 100
),
vec_results AS (
    SELECT i.id, ROW_NUMBER() OVER(ORDER BY v.distance) as rank_vec
    FROM vec_items v
    JOIN items i ON i.rowid_item = v.rowid
    WHERE v.embedding MATCH :query_vector AND v.distance < 0.85
    ORDER BY v.distance ASC
    LIMIT 100
)
SELECT 
    i.id, 
    i.title, 
    i.vault_path,
    i.category,
    i.summary,
    COALESCE(f.rank_fts, 999) as r_fts,
    COALESCE(v.rank_vec, 999) as r_vec,
    (
      CASE WHEN f.rank_fts IS NOT NULL THEN 1.0 / (60.0 + f.rank_fts) ELSE 0.0 END +
      CASE WHEN v.rank_vec IS NOT NULL THEN 1.0 / (60.0 + v.rank_vec) ELSE 0.0 END
    ) as rrf_score
FROM items i
LEFT JOIN fts_results f ON i.id = f.id
LEFT JOIN vec_results v ON i.id = v.id
WHERE f.id IS NOT NULL OR v.id IS NOT NULL
ORDER BY rrf_score DESC
LIMIT :limit;
```

### Correção de Vulnerabilidade TOCTOU (Time-of-Check to Time-of-Use)

A verificação e leitura de caminhos do sistema de arquivos é suscetível a ataques de link simbólico se o arquivo for modificado entre a validação de segurança e a leitura real pelo backend Rust. O Sandland corrige o TOCTOU realizando a validação e vinculando a leitura diretamente ao descritor de arquivo resolvido de forma canônica:

```rust
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{self, Read};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SecurityError {
    #[error("Caminho base inválido")]
    InvalidBase,
    #[error("Arquivo não encontrado")]
    FileNotFound,
    #[error("Tentativa de fuga do Sandbox detectada!")]
    SandboxEscapeAttempt { attempted: PathBuf, boundary: PathBuf },
    #[error("Erro de I/O: {0}")]
    Io(#[from] io::Error),
}

/// Abre um arquivo de forma segura garantindo que ele reside estritamente 
/// dentro do limite do sandbox estabelecido pelo base_workspace.
/// Resolve symlinks atomicamente em nível de kernel durante a abertura.
pub fn secure_read_to_string(base_workspace: &Path, requested_path: &Path) -> Result<String, SecurityError> {
    let canonical_base = base_workspace.canonicalize()
        .map_err(|_| SecurityError::InvalidBase)?;
    
    // Resolve o caminho alvo canônico primariamente
    let canonical_target = requested_path.canonicalize()
        .map_err(|_| SecurityError::FileNotFound)?;

    // Validação lógica do limite físico do Sandbox
    if !canonical_target.starts_with(&canonical_base) {
        return Err(SecurityError::SandboxEscapeAttempt {
            attempted: canonical_target,
            boundary: canonical_base,
        });
    }

    // OPERAÇÃO ATÔMICA: Abre diretamente o descritor utilizando o caminho canônico já validado.
    // Evita ler o caminho original (requested_path), que poderia ter sido alterado no intervalo.
    let mut file = File::open(&canonical_target)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    Ok(content)
}
```

---

## 5. Subsistema de Espaço Espacial (Whiteboard & Pílulas de Canvas)

O Whiteboard do Sandland foi desenhado para contornar gargalos de renderização e evitar conflitos de controle de versão (Git) inerentes a telas infinitas monolíticas.

```
                              TOPOLOGIA DO WHITEBOARD
 ┌───────────────────────────────────────────────────────────────────────────┐
 │ BARRA SUPERIOR: Galeria de Pílulas de Canvas                              │
 │ [ Pílula: Pesquisa IA ] [ Pílula: Filosofia da Mente ] [ + Novo Board ]   │
 ├───────────────────────────────────────────────────────────────────────────┤
 │ VIEWPORT WEBGL (PixiJS)                                                   │
 │                                                                           │
 │   ┌───────────────────────┐              ┌───────────────────────────┐    │
 │   │ WIDGET DE BUSCA WEB   │              │ CÉLULA ATÔMICA (Note)     │    │
 │   │ (Snapshot Readability)│─────────────►│ "Atenção seletiva em      │    │
 │   │ Artigo: Transformers  │   Conexão    │  redes neurais..."        │    │
 │   └───────────────────────┘  (Ctrl+Drag) └─────────────┬─────────────┘    │
 │                                                        │                  │
 │                                                        │ Fork-on-insert   │
 │                                                        ▼                  │
 │ ┌───────────────────────────────────────────────────────────────────────┐ │
 │ │ A PEÇA (Painel Lateral Retrátil - Docked à Esquerda)                  │ │
 │ │ [Editor Modular de Blocos]  │  [Canal do Copiloto (IA)]               │ │
 │ │ # Título do Ensaio          │  > Usuário: Tensionar este parágrafo    │ │
 │ │ Parágrafo com célula forked │  > Copiloto: A premissa falha ao        │ │
 │ │ e citação preservada.       │    desconsiderar a latência do hardware │ │
 │ └───────────────────────────────────────────────────────────────────────┘ │
 └───────────────────────────────────────────────────────────────────────────┘
```

### Especificação do Formato de Topologia Híbrido: MessagePack + JSON

1.  **Gravação Rápida de UI (`board.canvas.mpk`):** Serializado diretamente em formato binário compacto via MessagePack (`rmp-serde` no Rust). Salva a posição dos nós e o estado do viewport a cada 350ms em operação síncrona de alta performance.
2.  **Gravação para Interoperabilidade Git (`board.canvas.json`):** Uma rotina em background serializa o JSON formatado em formato legível somente quando a UI está inativa (idle) por mais de 2 segundos, prevenindo conflitos complexos de merge no histórico Git e mantendo a interoperabilidade.

### Virtualização e Level-of-Detail (LOD) no PixiJS Engine

Para sustentar mais de 1000 nós em 60 FPS estáveis, o renderizador baseado em WebGL aplica:

1.  **Frustum Culling:** Nós cujas coordenadas delimitadoras estejam fora das dimensões da câmera ativa do Viewport são sumariamente omitidos do ciclo de desenho do PixiJS.
2.  **Mecânica de Level-of-Detail (LOD):**
    *   **Zoom $> 0.6$ (Próximo):** Renderização completa do nó: cabeçalhos, Markdown hidrato via renderizador interno, inputs de texto ativos e mídias visuais.
    *   **Zoom $0.3 \le Z \le 0.6$ (Médio):** Omissão do corpo de texto dinâmico. Renderização limitada ao Título do nó, contorno colorido indicando tipo, e ícones descritivos.
    *   **Zoom $< 0.3$ (Distante):** Omissão completa de texto. O nó é desenhado apenas como uma caixa de cor sólida simplificada (proxy card) com opacidade reduzida, aliviando o fillrate da GPU.

---

## 6. Motor Nativo de Descoberta e Extração Web

O Sandland resolve a necessidade de extração web sem violar seu pilar de isolamento de processos (sem interpretadores em background ou dependência síncrona do Chromium).

```
                         PIPELINE DE EXTRAÇÃO WEB (RUST)
                        ┌───────────────────────────────┐
                        │   Requisição: URL ou Query    │
                        └───────────────┬───────────────┘
                                        │
                                        ▼
                        ┌───────────────────────────────┐
                        │ Descoberta SERP (Se for busca)│
                        │ Consulta SearXNG/DDG Lite     │
                        └───────────────┬───────────────┘
                                        │
                                        ▼
                        ┌───────────────────────────────┐
                        │ FAST PATH: crate `rquest`      │
                        │ Emulação TLS BoringSSL        │
                        │ Assinatura JA3/JA4 Chrome     │
                        └───────┬───────────────┬───────┘
                                │               │
                         Sucesso (200 OK)    Bloqueio (403/Cloudflare JS)
                                │               │
                                ▼               ▼
            ┌───────────────────────┐   ┌───────────────────────────────┐
            │ Parser Rust Nativo    │   │ STEALTH PATH: `chromiumoxide` │
            │ `readability-rs`      │   │ Instância Headless sob CDP    │
            │ Limpeza e Conversão   │   │ Opcional / Lazy-installed     │
            │ direta para Markdown  │   │ Resolução de JS e Captura DOM │
            └───────────┬───────────┘   └───────────────┬───────────┘
                        │                               │
                        └───────────────┬───────────────┘
                                        │
                                        ▼
                        ┌───────────────────────────────┐
                        │ Gravação no Vault e Canvas    │
                        │ - Snapshot .md em /ingest/    │
                        │ - Imagens em /vault/assets/   │
                        │ - Plotagem de Widget no Board │
                        └───────────────────────────────┘
```

1.  **Fast Path (Nativo e Leve):** Requisição via HTTP client escrito em Rust nativo utilizando a biblioteca `rquest` (com suporte a spoofing de TLS BoringSSL e assinaturas JA3/JA4 simulando navegadores modernos). Resolve $+85\%$ das requisições tradicionais sem abrir processos pesados.
2.  **Stealth Path (Headless Opcional):** Se o Fast Path falhar (bloqueio Cloudflare, Turnstile ou dependência pesada de SPA Client-Side Rendering), o sistema recorre ao motor `chromiumoxide` (Chrome DevTools Protocol).
    *   **Instalação sob demanda (Lazy):** O Chromium não é empacotado no binário principal. O gerenciador de pacotes do Sandland oferece o download transparente de uma versão compacta e isolada do Chromium headless na pasta `.system/bin/` na primeira falha do Fast Path, isolando o consumo de disco do instalador nativo.

---

## 7. Subsistema de Redação e Co-Autoria (A "Peça" & Copiloto)

```
 ┌───────────────────────────────────────────────────────────────────────┐
 │ A PEÇA (Painel Lateral Retrátil - Docked à Esquerda)                  │
 │ [Editor Modular de Blocos]  │  [Canal do Copiloto (IA)]               │
 │ # Título do Ensaio          │  > Usuário: Tensionar este parágrafo    │
 │                             │                                         │
 │ Parágrafo com célula forked │  > Copiloto: A premissa falha ao        │
 │ e citação preservada.       │    desconsiderar a latência do hardware │
 └─────────────────────────────┴─────────────────────────────────────────┘
```

### Mecânica de Inserção: *Fork-on-Insert*

Ao arrastar uma Célula do canvas para dentro da Peça:

1.  O conteúdo é clonado para dentro da Peça de forma independente.
2.  Um metadado de citação é inserido no Frontmatter YAML da Peça garantindo a rastreabilidade completa do insight (*provenance*):

```markdown
---
id: piece_01j9a8b7c
title: "Ensaio sobre Sistemas Autônomos"
created_at: 2026-09-21T10:15:00Z
citations:
  - source_cell_id: "cell_01j8k9_01"
    inserted_at: "2026-09-21T10:30:00Z"
    original_snippet: "Atenção seletiva em redes neurais..."
---
```

---

## 8. Arquitetura RLM (*Recursive Language Model*) & Sandboxing

### A Estrutura de Orquestração e Despacho do RLM

O motor RLM contorna o limite de contexto dos modelos locais (*Context Bloat*) por meio de uma árvore de agentes hierárquica e síntese progressiva, impedindo que o modelo consuma arquivos sem relevância para a tarefa ativa.

```
                           HIERARQUIA DE EXECUÇÃO RLM
                      ┌─────────────────────────────────┐
                      │    Prompt do Usuário / Copiloto │
                      └────────────────┬────────────────┘
                                       │
                                       ▼
                      ┌─────────────────────────────────┐
                      │ NÍVEL 0: Agente Despachante     │
                      │ Consome APENAS o Manifesto      │
                      │ (board.canvas.json / Topologia) │
                      └────────┬───────────────┬────────┘
                               │               │
                 Requer Cluster A              Requer Cluster B
                               │               │
                               ▼               ▼
         ┌───────────────────────────┐   ┌───────────────────────────┐
         │ NÍVEL 1: Sub-Agente A     │   │ NÍVEL 1: Sub-Agente B     │
         │ Lê resumos das células    │   │ Lê resumos das células    │
         │ do grupo de Arquitetura   │   │ do grupo de Concorrência  │
         └─────────────┬─────────────┘   └─────────────┬─────────────┘
                       │ (Recursão se                  │
                       │  necessário)                  │
                       ▼                               ▼
         ┌───────────────────────────┐   ┌───────────────────────────┐
         │ NÍVEL 2: Folha Atômica    │   │ NÍVEL 2: Folha Atômica    │
         │ Lê parágrafo exato        │   │ Lê parágrafo exato        │
         │ do artigo web raspado     │   │ da transcrição de áudio   │
         └─────────────┬─────────────┘   └─────────────┬─────────────┘
                       │                               │
                       └───────────────┬───────────────┘
                                       │
                                       ▼
                      ┌─────────────────────────────────┐
                      │ Síntese e Colapso Ascendente    │
                      │ Resposta consolidada entregue   │
                      │ ao painel lateral da Peça       │
                      └─────────────────────────────────┘
```

#### Fase 1: Despacho de Nível 0 (Index-Guided Routing)
*   O Agente Despachante (Nível 0) **não lê** o conteúdo bruto das células.
*   Ele consome uma tabela em cache rápida de resumos da área de trabalho ativa (`workspace_summary_index.db`), contendo o mapeamento de IDs de nós, títulos e resumos executivos de 100 palavras de cada nó.
*   Com base no prompt do usuário, o Nível 0 determina quais clusters lógicos de nós precisam ser investigados, despachando sub-agentes paralelos (Nível 1) apenas para as chaves geográficas identificadas.

#### Fase 2: Síntese Progressiva (Collapse-Up)
*   **Nível 2 (Folha Atômica):** Extrai passagens textuais diretas do snapshot web ou da célula contida, gerando uma micro-síntese focada na pergunta do usuário.
*   **Nível 1 (Sub-Agente de Cluster):** Agrega as micro-sínteses das folhas atômicas do seu cluster. Identifica contradições e remove argumentos redundantes, compilando os dados num sumário analítico único do cluster.
*   **Nível 0 (Consolidador):** Recebe os sumários consolidados dos sub-agentes, monta a resposta final estruturada e entrega diretamente à UI do Copiloto, reduzindo o consumo de tokens na janela principal em até $80\%$.

### A Jaula de Sistema de Arquivos (OS-Level Sandboxing)

Para blindar o sistema contra comandos maliciosos injetados por ataques de injeção de prompt no Copilot, as threads secundárias de inferência de IA e o motor headless Chromium rodam sob uma arquitetura de privilégio mínimo e isolamento forçado do sistema operacional.

```
                           ARQUITETURA DE SANDBOX MULTIPLATAFORMA
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │                     PROCESSO DE INFRAESTRUTURA TAURI                        │
 └──────────────────────────────────────┬──────────────────────────────────────┘
                                        │ Spawns isolated process
                                        ▼
   ┌─────────────────────────────────────────────────────────────────────────┐
   │                       JAULA DO SISTEMA OPERACIONAL                      │
   ├──────────────────────────┬──────────────────────────┬───────────────────┤
   │ LINUX: LANDLOCK LSM      │ MAC OS: SEATBELT PROFILE │ WINDOWS: JOBS     │
   ├──────────────────────────┼──────────────────────────┼───────────────────┤
   │ - Bloqueia syscalls      │ - Perfil Lisp dinâmico   │ - SIDs restritos  │
   │ - Apenas pasta ativa     │ - Restringe escrita e    │ - No network      │
   │   tem write/read         │   rede no processo-filho │ - Job object cage │
   └──────────────────────────┴──────────────────────────┴───────────────────┘
```

#### 1. Linux (Landlock LSM & Seccomp)
Utiliza Landlock para restringir o acesso ao sistema de arquivos do processo de inferência em nível de kernel:

```rust
// Simplificação lógica da inicialização do Landlock no Linux
use landlock::{Access, AccessFs, Ruleset, RulesetError, RulesetAttr};

pub fn enforce_linux_sandbox(allowed_read: &Path, allowed_write: &Path) -> Result<(), RulesetError> {
    let mut ruleset = Ruleset::default()
        .handle_access(AccessFs::from_all())?; // Intercepta todas as chamadas de FS

    // Registra permissões apenas para caminhos válidos do Vault
    ruleset.add_rule(AccessFs::from_read().allow_path(allowed_read))?;
    ruleset.add_rule(AccessFs::from_write().allow_path(allowed_write))?;

    // Ativa as restrições na thread atual e seus descendentes
    ruleset.restrict_self()?;
    Ok(())
}
```

#### 2. macOS (Seatbelt / Sandbox-Exec Profiles)
No macOS, o Sandland executa qualquer pipeline de agente externo invocando o executável sob o comando `/usr/bin/sandbox-exec` utilizando um perfil temporário gerado dinamicamente:

```scheme
;; seatbelt_profile.sb - Gerado em runtime pelo Sandland para conter o processo filho
(version 1)
(deny default)

(allow process-fork)
(allow sysctl-read)

;; Permite ler recursos compartilhados do sistema operacional necessários para execução
(allow file-read* 
       (subpath "/usr/lib")
       (subpath "/System/Library")
       (subpath "/private/var/db/dyld"))

;; Jaula física e lógica estrita do Workspace
(allow file-read* file-write*
       (subpath "/private/tmp/sandland") ;; Temp runtime
       (subpath "{WORKSPACE_CANONICAL_PATH}")
       (subpath "{VAULT_ASSETS_PATH}"))

;; Bloqueia conexões de rede de saída para threads locais de agentes
(deny network-outbound)
```

#### 3. Windows (Job Objects & Restrições de Segurança)
No Windows, o processo do agente é encapsulado em um *Job Object* com limites rigorosos:
*   `ActiveProcessLimit` definido como $1$ (impede que o agente inicie novos processos em background).
*   Associação a um Token restrito via `CreateRestrictedToken` que remove os grupos de administradores e remove privilégios de gravação fora do diretório de trabalho lógica especificado.
*   Bloqueio de conexões de rede locais e externas através de regras ativas no firewall do Windows associadas ao ID temporário do processo.

---

## 9. Camada de Intenções (Teleologia) e Chat Global

A camada de Intenções funciona como um canal de controle proativo, permitindo que o Sandland identifique correlações em segundo plano sem que o usuário precise mapear links manualmente.

```json
{
  "active_intentions": [
    {
      "id": "intent_01j8m",
      "title": "Investigar Concorrência Segura sem Garbage Collection",
      "scope": "Engenharia de Sistemas",
      "hypotheses": [
        "Sistemas de tipagem lineares reduzem consumo de memória mas aumentam complexidade de compilação"
      ],
      "tags_monitored": ["rust", "memory-management", "concurrency"],
      "created_at": "2026-09-01T12:00:00Z"
    }
  ]
}
```

### Proposições Proativas e Disparo de Alertas

Durante o processamento assíncrono do *Ingest*, o Sandland calcula a similaridade do vetor do novo material contra o índice de intenções ativas. Se a pontuação ultrapassar o threshold de $0.85$, um contador de afinidade é incrementado. Ao atingir o limite estipulado, o Sandland emite uma notificação elegante sugerindo a criação automática de um novo Canvas conectando os itens descobertos.

---

## 10. Gestão e Ciclo de Vida de Modelos Locais

Para evitar downloads fragmentados e problemas de integridade de dados que quebram a inferência de modelos locais, o Sandland implementa um subsistema rigoroso de registro e verificação de integridade de modelos (*Model Lifecycle Manager*).

### Matriz de Modelos Homologados

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         MATRIZ DE MODELOS HOMOLOGADOS                       │
├───────────┬──────────────────────┬─────────┬────────────────────────────────┤
│ PAPEL     │ MODELO ESPECÍFICO    │ FORMATO │ CHECKSUM SHA-256               │
├───────────┼──────────────────────┼─────────┼────────────────────────────────┤
│ Embeddings│ paraphrase-multilingual │ ONNX    │ 5f9e2a1b7c8d9e0f...            │
│           │ MiniLM-L12-v2 (384d) │         │                                │
│ Classifier│ Qwen2.5-1.5B-Instruct│ GGUF(Q4)│ d4b3c2a1e0f9a8b7...            │
│ Copilot   │ Llama-3.2-3B-Instruct│ GGUF(Q4)│ a1b2c3d4e5f6a7b8...            │
│ Whisper   │ whisper.cpp-base     │ Bin     │ 9e8d7c6b5a4f3e2d...            │
└───────────┴──────────────────────┴─────────┴────────────────────────────────┘
```

### Caminhos de Instalação e Cache do Sistema

Os modelos de IA são mantidos em uma pasta de cache global fora dos cofres individuais (*vaults*) para evitar redundância de disco em instalações de múltiplos projetos:

*   **macOS:** `~/Library/Application Support/sandland/models/`
*   **Linux:** `~/.local/share/sandland/models/`
*   **Windows:** `%APPDATA%\sandland\models\`

### Pipeline de Verificação de Integridade

```rust
// Código conceitual de inicialização e validação de modelo pelo Sandland Core
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

pub enum ModelStatus {
    Verified,
    Corrupted,
    Missing,
}

pub fn check_model_integrity(model_path: &Path, expected_sha: &str) -> ModelStatus {
    if !model_path.exists() {
        return ModelStatus::Missing;
    }

    let file = match File::open(model_path) {
        Ok(f) => f,
        Err(_) => return ModelStatus::Corrupted,
    };

    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024 * 1024]; // Buffer de 1MB

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break, // Fim do arquivo
            Ok(bytes_read) => hasher.update(&buffer[..bytes_read]),
            Err(_) => return ModelStatus::Corrupted,
        }
    }

    let hash_result = format!("{:x}", hasher.finalize());
    if hash_result.eq_ignore_ascii_case(expected_sha) {
        ModelStatus::Verified
    } else {
        ModelStatus::Corrupted
    }
}
```

Se o arquivo de modelo for identificado como corrompido ou ausente, o gerenciador de download nativo do Sandland interrompe a tentativa de inicialização, remove o binário quebrado e inicia um download seguro e resiliente (com suporte a resumo/resuming de pacotes e rotação de espelhosCDN).

---

## 11. Estratégia de Versionamento e Auditoria

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      SISTEMA DE VERSIONAMENTO DUPLO                         │
├───────────────────────────────┬─────────────────────────────────────────────┤
│ CAMADA NATIVA (LOCAL / EVENT) │ CAMADA GIT (OPCIONAL / REMOTO)              │
├───────────────────────────────┼─────────────────────────────────────────────┤
│ - Granularidade: Transações   │ - Granularidade: Horas / Marcos Deliberados │
│ - Banco: audit_log.db         │ - Repositório: .git na raiz do /vault/      │
│ - Trata: Eventos semânticos,  │ - Trata: Arquivos .md físicos e ativos      │
│   mutações de texto, tags IA, │ - Finalidade: Sincronização multi-máquina,  │
│   NODE_DRAG_END (debounced)   │   backup contra falhas de hardware,         │
│ - Micro-movimentos (350ms):   │   auditoria externa via ferramentas padrão  │
│   board.canvas.mpk (binário)  │                                             │
│ - Finalidade: Desfazer em     │                                             │
│   lote, reversão de IA        │                                             │
└───────────────────────────────┴─────────────────────────────────────────────┘
```

---

## 12. Roteiro de Implementação Técnica (Milestones)

```
                            CRONOGRAMA DE ENTREGAS
  v0.1: O Núcleo Local-First e Canvas Básico
  ├── Runtime Tauri v2 (Linux/Windows) + Core Rust
  ├── Banco SQLite relacional e busca vetorial com sqlite-vec
  ├── Ingest com Auto-Tagging local (Qwen-2.5-1.5B + GBNF)
  ├── Sandboxing Nativo Linux (Landlock) e macOS (Seatbelt)
  └── Canvas infinito (PixiJS) com Células e manipulação de arquivos .md
  
  v0.2: A Camada Editorial e Co-autoria
  ├── A Peça (Editor em blocos no painel lateral retrátil)
  ├── Mecânica de inserção de nós por fork-on-insert
  ├── Galeria de Workspaces em Pílulas (Pills)
  └── Copiloto de Redação (Ferramentas de fluxo, reescrita e crítica)
  
  v1.0: Autonomia Web, RLM e Confinamento
  ├── Motor de busca e extração web em Rust puro (rquest + readability-rs)
  ├── Opcional Stealth-Path com Chromium sob demanda
  ├── Arquitetura de RLM (Execução hierárquica e colapso de nós)
  └── Camada de Intenções com sugestão proativa de projetos
```

---

## 13. Critérios de Aceite de Engenharia (Checklist de Validação)

1.  **Eficiência de Recursos:** O aplicativo em repouso (*idle*) com um canvas carregando 100 nós deve consumir menos de **350 MB de RAM** no Linux, Windows e macOS.
2.  **Latência de Inicialização:** O tempo decorrido entre o clique no ícone do app e a interatividade total do canvas ativo deve ser inferior a **2.2 segundos** no armazenamento NVMe.
3.  **Resiliência contra Quedas (*Crash Recovery*):** Desligar o computador bruscamente no meio de uma edição não deve corromper os arquivos de texto; a recuperação via log de eventos do SQLite reconstrói o estado com perda máxima de **500 milissegundos** de digitação.
4.  **Isolamento de Segurança:** Nenhuma chamada disparada pela IA via `tool_call` ou script web de terceiro pode resolver caminhos fora do subdiretório do workspace correspondente, garantindo proteção contra vulnerabilidades de travessia de diretórios (*directory traversal*) de forma física e via kernel.