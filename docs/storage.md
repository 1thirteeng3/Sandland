# Modelo de Armazenamento e Dados (Storage Architecture)

**Status:** Aprovado  
**Escopo:** Persistência Local-First, Coexistência de Motores, Topologia do Canvas e Ciclo de Vida de Células

---

## 1. Topologia Canônica do Vault em Disco

O Sandland adota o princípio da **Soberania do Dado (*File-as-Truth*)**, onde o sistema de arquivos local é a fonte primária e definitiva da verdade. O cofre do usuário organiza-se hierarquicamente da seguinte forma:

```text
SandlandVault/
├── .system/                        <-- Metadados de controle e índices efêmeros (descartáveis)
│   ├── index.db                    <-- SQLite com extensões sqlite-vec e FTS5 (Acervo)
│   ├── index.db-wal
│   └── index.db-shm
├── intentions/                     <-- Camada teleológica de pesquisa (Metas, Hipóteses)
│   └── intentions.json
├── assets/                         <-- Content-Addressable Storage (CAS)
│   ├── 4f8a...3e.pdf               <-- Arquivos binários renomeados por hash SHA-256
│   └── d41d...2e.png
├── ingest/                         <-- Documentos Brutos / Acervo (File-as-Truth)
│   ├── notes/                      <-- Notas e reflexões ingeridas (*.md)
│   ├── web/                        <-- Snapshots e clippings da web (*.md)
│   └── media/                      <-- Transcrições Whisper de áudios/vídeos (*.md)
└── workspaces/                     <-- Compartimentalização por Board / Mesa de Pesquisa
    ├── default-workspace/
    │   ├── topology.json           <-- Grafo vetorialless canônico (células, arestas, grupos)
    │   ├── board.canvas.json       <-- Compatibilidade visual e backup estruturado
    │   └── scratchpad/             <-- Rascunhos temporários do usuário
    └── deep-research-ai/
        ├── topology.json
        └── board.canvas.json
```

---

## 2. Dualidade de Motores de Conhecimento

| Dimensão | Motor 1: Acervo de Documentos Brutos | Motor 2: Mesa de Pesquisa / Canvas (RLM) |
| :--- | :--- | :--- |
| **Escopo de Dados** | Documentos longos, PDFs, clippings web, artigos | Células atômicas, notas de síntese, conexões conceituais |
| **Mecanismo de Indexação** | Vetorial denso (`sqlite-vec` / 384d) + Lexical (`fts5`) | **Vectorless RAG**: Topologia pura via `topology.json` |
| **Granularidade** | Documentos inteiros com busca em chunks / parágrafos | Estrutura de grafo: nós, grupos semânticos e arestas |
| **Persistência Primária** | Arquivos `.md` individuais com YAML Frontmatter | Arquivo unificado `topology.json` por workspace |
| **Consumo por Modelos** | RAG Tradicional (Similaridade de cosseno + BM25) | **Skeleton Map** (Árvore hierárquica serializada no prompt) |

---

## 3. O Paradigma Vectorless RAG no Canvas

### 3.1. Por que dispensar a vetorização na Mesa?
Em uma mesa visual de pesquisa, o significado e a relevância de uma ideia não dependem exclusivamente da proximidade vetorial de suas palavras, mas sim de:
1. **Posicionamento e Clusterização Espacial:** Itens próximos no canvas expressam afinidade temática decidida intencionalmente pelo usuário.
2. **Arestas Semânticas:** Linhas e setas entre células explicitam relações causais, oposições ou desdobramentos lógicos ("A refuta B", "C é evidência de D").
3. **Agrupamento Hierárquico:** Grupos e molduras delimitam tópicos e fases de raciocínio.

Converter essas células em fragmentos vetoriais cegos destrói o grafo cognitivo. O Sandland trata a mesa como uma **árvore relacional explícita**.

### 3.2. Estrutura do `topology.json`
```json
{
  "workspaceId": "default-workspace",
  "version": 1,
  "viewport": { "x": 0, "y": 0, "zoom": 1.0 },
  "nodes": [
    {
      "id": "cell-01",
      "title": "Hipótese de Segurança em Rust",
      "content": "O borrow checker garante ausência de data races em tempo de compilação.",
      "x": 120.0,
      "y": 240.0,
      "width": 280.0,
      "height": 160.0,
      "groupId": "group-systems",
      "sourceItemId": "019213a8-7b2c-7000-8000-000000000001",
      "updatedAt": 1726916400
    }
  ],
  "edges": [
    {
      "id": "edge-01",
      "sourceNodeId": "cell-01",
      "targetNodeId": "cell-02",
      "relationType": "supports",
      "directed": true
    }
  ],
  "groups": [
    {
      "id": "group-systems",
      "title": "Engenharia de Sistemas",
      "color": "#3b82f6",
      "bounds": { "x": 100.0, "y": 200.0, "width": 640.0, "height": 400.0 }
    }
  ]
}
```

---

## 4. Ciclo de Vida das Células: Prevenção de Poluição de Disco

Um problema comum em ferramentas de anotação é a criação desordenada de milhares de micro-arquivos `.md` vazios ou efêmeros para cada post-it ou ideia descartável criada no canvas.

O Sandland resolve isso através de um ciclo de vida em duas fases:

```
[ Usuário cria ideia na mesa ]
             │
             ▼
┌──────────────────────────────────────────────┐
│  Célula Estruturada (No-Disk Pollution)      │
│  - Vive unicamente no `topology.json`        │
│  - Edição instantânea via overlay DOM        │
│  - Não gera arquivos individuais no cofre    │
└──────────────────────┬───────────────────────┘
                       │
         [ Comando Explícito do Usuário ]
          "Promover Célula a Nota"
                       │
                       ▼
┌──────────────────────────────────────────────┐
│  Nota Física Canônica                        │
│  - Cria `ingest/notes/<slug>-<uuid>.md`       │
│  - Gera YAML Frontmatter canônico            │
│  - Indexa no SQLite `index.db` & FTS5        │
│  - Célula no canvas mantém link com o id     │
└──────────────────────────────────────────────┘
```

1. **Estado Nativo (Nó do Canvas):** As células nascem e vivem como entradas no array `nodes` do arquivo `topology.json`. Esse arquivo é salvo de forma atômica e debounced, garantindo que o cofre permaneça limpo e rastreável.
2. **Promoção a Nota Canônica:** Somente mediante a ação intencional do usuário (*"Promover a Nota"* ou atalho de menu de contexto), o backend Rust extrai o título e corpo da célula, serializa com YAML Frontmatter em `ingest/notes/` e indexa no `index.db` do Acervo.

---

## 5. Esquema Relacional do Acervo (`.system/index.db`)

O banco SQLite com `sqlite-vec` atua estritamente como índice de alta velocidade para o Acervo de documentos brutos:

- **Tabela `items`:** Registro primário de cada nota física ou captura web com título, hash SHA-256 e estado do ciclo de vida.
- **Tabela Virtual `items_fts` (FTS5):** Índice invertido com suporte a stemming e busca por prefixos para correspondência exata e termos técnicos.
- **Tabela Virtual `vec_items` (`vec0`):** Índice vetorial de 384 dimensões em ponto flutuante, populado sob demanda pelo motor de embeddings.
- **Tabela `taxonomy_tags`:** Mapeamento N:M de categorias e etiquetas aplicadas às notas.
