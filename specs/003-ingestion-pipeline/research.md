# Research & Technical Decisions: Pipeline de Ingestão e Processamento de Documentos

**Feature**: `003-ingestion-pipeline`  
**Date**: 2026-09-22

## Research Decisions

### Decision 1: Estrutura Física de Ingestão e Imutabilidade Canônica (Princípio I)
- **Decision**: Toda nota de texto ou Markdown ingerida é persistida fisicamente em `<vault_root>/ingest/notes/<filename>.md`. Arquivos binários/mídias são gravados no Content-Addressable Storage (CAS) em `<vault_root>/assets/<sha256>.<ext>`.
- **Rationale**: Em conformidade com a Constituição v2.0 (Soberania do Dado e File-as-Truth), o sistema de arquivos local é a fonte primária da verdade. O arquivo original nunca é adulterado pelo pipeline de enriquecimento.
- **Alternatives considered**:
  - Salvar o texto apenas em banco de dados SQLite: Rejeitado porque violaria o princípio File-as-Truth e prenderia o conteúdo do usuário em formato proprietário.
  - Misturar notas brutas com células atômicas em `workspaces/`: Rejeitado para preservar o princípio II (Separação Cognitiva e Fork-on-Insert).

---

### Decision 2: Indexação em SQLite `index.db` e FTS5
- **Decision**: Utilizar tabela `ingested_items` e tabela virtual FTS5 (`ingested_items_fts`) no SQLite derivado (`<vault_root>/.system/index.db`).
- **Rationale**: Permite buscas instantâneas (< 10ms) e filtragem por tags/título sem ler centenas de arquivos `.md` do disco a cada digitação no campo de busca.
- **Alternatives considered**:
  - Scan recursivo em disco a cada pesquisa: Rejeitado por degradar a responsividade com dezenas de milhares de arquivos.
  - Usar banco de dados externo ou servidor dedicado: Rejeitado pela exigência de zero-config e operação 100% offline.

---

### Decision 3: Mecânica de Promoção para o Canvas (Fork-on-Insert Estendido)
- **Decision**: Ao promover uma nota do Acervo para a Mesa Espacial (Canvas), o Sandland cria um nó do tipo `text` ou `card` contendo o texto inicial e metadados de proveniência (`source_path`, `source_hash`).
- **Rationale**: Garante independência editorial. O usuário pode reescrever ou encurtar a nota na Mesa sem alterar a cópia física original armazenada em `ingest/notes/`.
- **Alternatives considered**:
  - Two-way binding síncrono (editar no canvas altera a nota original): Rejeitado expressamente pela Constituição (Princípio II - Mecânica de Fork-on-Insert Estendido).

---

### Decision 4: Proteção Ativa contra SSRF em Captura de URLs (Princípio III)
- **Decision**: O manipulador de URLs valida o host antes de qualquer conexão HTTP, bloqueando `localhost`, `127.0.0.1`, `::1`, faixas privadas IPv4 (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16) e endereços link-local.
- **Rationale**: Previne ataques de SSRF (Server-Side Request Forgery) conforme mandatório no Princípio III da Constituição.
- **Alternatives considered**:
  - Fazer requisição cega com cliente HTTP padrão: Rejeitado por vulnerabilidade de segurança.
