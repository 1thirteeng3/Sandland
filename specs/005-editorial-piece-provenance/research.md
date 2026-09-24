# Research & Technical Decisions: Peça Editorial e Proveniência Canônica

**Feature**: `005-editorial-piece-provenance`  
**Date**: 2026-09-24  
**Status**: Approved  

---

## Technical Decisions

### Decision 1: Formato Canônico e Persistência Física da Peça Editorial

- **Decisão**: Cada Peça Editorial é armazenada exclusivamente como um arquivo Markdown individual em `workspaces/<workspace_id>/pieces/<piece_id>.md` com metadados estruturados em YAML Frontmatter.
- **Racional**:
  - Atende estritamente ao **Princípio I da Constituição (Soberania do Dado e File-as-Truth)**: o sistema de arquivos local é a única fonte da verdade; nenhum serviço proprietário é necessário para ler ou exportar a Peça.
  - O YAML frontmatter preserva o catálogo estruturado de citações (`citations`), enquanto o corpo do documento permanece texto legível puro compatível com qualquer ferramenta (Obsidian, VS Code, Git).
- **Alternativas Consideradas**:
  - *Armazenar peças no banco SQLite (`index.db`)*: Rejeitado categoricamente, pois violaria a Constituição que define o SQLite apenas como índice descartável/derivado.
  - *Arquivo monólito JSON*: Rejeitado por prejudicar a ergonomia de leitura humana e impossibilitar diffs limpos no controle de versão Git.

---

### Decision 2: Mecânica de Fork-on-Insert Estendido e Imunidade Retroativa

- **Decisão**: Ao transferir qualquer nó ou trecho da Mesa para a Peça, o conteúdo é duplicado de forma autônoma (Fork-on-Insert). O frontmatter da Peça registra uma entrada de citação estruturada:
  ```yaml
  ---
  id: "piece-artigo-1"
  workspace_id: "default-workspace"
  title: "Arquitetura e Soberania Local"
  slug: "artigo-1"
  word_count: 420
  created_at: 1727180000
  updated_at: 1727180400
  citations:
    - id: "cit-101"
      source_cell_id: "cell-welcome"
      source_revision: 1
      source_title: "Bem-vindo ao Sandland"
      source_asset_hash: null
      quote: "Dê um duplo clique nesta célula para editar..."
      quote_hash: "a1b2c3d4..."
      inserted_at: 1727180200
  ---
  ```
- **Racional**:
  - Implementa o **Princípio II da Constituição (Separação Cognitiva e Fork-on-Insert Estendido)**: a exploração na Mesa é livre e divergente, enquanto a Peça é convergente e autoral.
  - Mutações posteriores no cartão da Mesa NUNCA alteram silenciosamente a Peça. A citação retém o snapshot do texto no momento da inserção (`quote`).
- **Alternativas Consideradas**:
  - *Transclusão dinâmica / ponteiro reativo vivo*: Rejeitado expressamente pela Constituição; atualizações automáticas silenciosas destruíriam o raciocínio editorial fixado pelo autor.

---

### Decision 3: Detecção Determinística de Divergência (*Drift Detection*)

- **Decisão**: A integridade das citações é verificada através da comparação determinística de revisões entre o frontmatter da Peça (`source_revision`) e a nota física em `workspaces/<workspace_id>/cells/<source_cell_id>.md`:
  - Se `current_cell.revision == citation.source_revision`: **Sincronizado** (verde).
  - Se `current_cell.revision > citation.source_revision`: **Divergente** (âmbar - *Drift detectado*).
  - Se a célula física não existir: **Órfã / Histórica** (cinza - origem removida da Mesa, preservada na Peça).
- **Racional**:
  - Custo computacional insignificante ($O(1)$ por citação), sem depender de chamadas a LLMs ou processamento pesado.
  - O autor pode abrir uma visualização comparativa (Diff lado a lado) e decidir conscientemente se deseja atualizar o trecho na Peça ou manter a citação histórica.
- **Alternativas Consideradas**:
  - *Análise de embeddings semânticos para detectar divergência*: Rejeitado por adicionar complexidade desnecessária e latência em um problema perfeitamente resolvido por revisões sequenciais de OCC.

---

### Decision 4: Interface Unificada e Modo Split-View (Mesa + Peça)

- **Decisão**: A interface do workspace suporta três modos de visualização:
  1. *Canvas Only*: visualização espacial completa da Mesa a 60 FPS (PixiJS).
  2. *Piece Only*: editor editorial focado com tipografia refinada para escrita longa.
  3. *Split-View*: visualização dividida lado a lado, com redimensionamento fluido da viewport do PixiJS (`app.renderer.resize`) permitindo arrastar cartões diretamente da Mesa para o texto da Peça.
- **Racional**:
  - Permite fluxo de trabalho sem fricção cognitiva: pesquisar e organizar à esquerda, redigir à direita.
  - Mantém o teto de recursos estipulado no **Princípio IV da Constituição**: o editor editorial no DOM é leve e a renderização do PixiJS adapta seu culling à nova largura da viewport mantendo o consumo de memória em repouso abaixo de 350 MB.
- **Alternativas Consideradas**:
  - *Múltiplas janelas nativas do sistema operacional*: Rejeitado por introduzir complexidade de sincronização entre instâncias webview e gerenciamento de foco.
