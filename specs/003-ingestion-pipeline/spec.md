# Feature Specification: Pipeline de Ingestão de Notas e Arquivos Físicos

**Feature Branch**: `003-ingestion-pipeline`  
**Created**: 2026-09-22  
**Status**: Draft  
**Input**: User description: "Sprint 02: Pipeline de Ingestão de Notas e Arquivos Físicos (cópia real para o cofre, banco SQLite index.db e cards no Canvas)"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Ingestão e Persistência Física de Notas Markdown (Priority: P1)

Como pesquisador ou escritor, quero arrastar ou selecionar arquivos `.md` e `.txt` no painel "Acervo & Ingestão", para que eles sejam gravados fisicamente na pasta de ingestão do meu cofre local e indexados para busca rápida no banco SQLite `index.db`.

**Why this priority**: É o cerne do princípio constitucional I (Soberania do Dado e File-as-Truth). Sem a persistência física das notas e sua indexação, o usuário não possui acervo durável no disco.

**Independent Test**:
- Arrastar um arquivo `pesquisa.md` para o dropzone do aplicativo desktop.
- Verificar que o arquivo é gravado fisicamente em `<vault_root>/ingest/notes/pesquisa.md`.
- Verificar que o item aparece imediatamente na lista de itens do Acervo com título, data e contagem de palavras calculadas.
- Verificar que uma entrada correspondente foi inserida no SQLite `index.db`.

**Acceptance Scenarios**:
1. **Given** um cofre local aberto, **When** o usuário solta um arquivo `.md` no Dropzone, **Then** o arquivo é salvo no disco em `ingest/notes/`, uma entrada é criada em `index.db` e o item surge com status "Classified" na lista do Acervo.
2. **Given** um arquivo com frontmatter YAML existente, **When** o arquivo é ingerido, **Then** as tags e título do frontmatter são extraídos e preservados.
3. **Given** uma tentativa de ingerir um arquivo com nome conflitante, **When** o arquivo é salvo, **Then** ele recebe um sufixo numérico sem sobrescrever a versão anterior.

---

### User Story 2 - Promoção de Itens Ingeridos para a Mesa Espacial / Canvas (Priority: P2)

Como usuário explorando minhas referências, quero arrastar um item do painel "Acervo & Ingestão" diretamente para a Mesa Espacial (ou clicar em "Adicionar à Mesa"), criando automaticamente um nó espacial conectado à fonte primária.

**Why this priority**: Conecta o Acervo ao Canvas interativo, viabilizando o fluxo de exploração espacial e respeitando o princípio II (Separação Cognitiva e Edição Modular).

**Independent Test**:
- Clicar ou arrastar um item do Acervo para o Canvas.
- Um novo nó retangular surge na coordenada solta contendo o título, resumo/conteúdo e link para a nota original.
- A topologia do Canvas (`workspaces/default-workspace/topology.json`) é atualizada e persistida no disco com controle de revisão OCC.

**Acceptance Scenarios**:
1. **Given** um item listado no Acervo, **When** o usuário clica no botão "+" ou o arrasta para o Canvas, **Then** uma nova célula de nota é instanciada na Mesa Espacial.
2. **Given** uma célula criada a partir de uma nota ingerida, **When** o usuário edita a célula na Mesa, **Then** a edição ocorre na célula do workspace sem alterar a fonte bruta original em `ingest/notes/` (Mecânica de Fork-on-Insert).

---

### User Story 3 - Ingestão de Imagens e Ativos Binários no CAS (Priority: P3)

Como usuário, quero adicionar imagens e diagramas (PNG, JPEG, SVG, WebP) ao cofre e ao Canvas, garantindo que o arquivo seja armazenado no Content-Addressable Storage (`assets/<sha256>.<ext>`) sem duplicatas físicas.

**Why this priority**: Garante suporte a mídias visuais com deduplicação criptográfica por hash SHA-256 e visualização rápida na Mesa.

**Independent Test**:
- Ingerir uma imagem `diagrama.png`.
- O hash SHA-256 é calculado.
- O arquivo é copiado para `<vault_root>/assets/<hash>.png`.
- A imagem pode ser referenciada por múltiplos nós no Canvas sem duplicar o arquivo em disco.

**Acceptance Scenarios**:
1. **Given** uma imagem enviada pelo usuário, **When** o sistema processa o arquivo, **Then** ele grava em `assets/<sha256>.<ext>` e retorna o descritor de ativo com tamanho em bytes e hash canônico.
2. **Given** o envio da mesma imagem duas vezes, **When** o segundo envio ocorre, **Then** o sistema reconhece o hash existente e reaproveita o arquivo sem duplicar espaço em disco.

---

### User Story 4 - Captura Rápida de Snapshot de URL (Priority: P4)

Como pesquisador, quero colar uma URL no campo de captura para que o conteúdo textual da página seja extraído, convertido em Markdown limpo e salvo no cofre como nota de leitura offline.

**Why this priority**: Permite arquivamento ágil de artigos da web com imunidade a alterações ou exclusões externas.

**Independent Test**:
- Inserir `https://exemplo.com/artigo` no campo de URL.
- O sistema obtém o HTML, sanitiza via parser, converte para Markdown e grava em `ingest/web/<timestamp>-slug.md`.

**Acceptance Scenarios**:
1. **Given** uma URL válida da internet, **When** o usuário submete a captura, **Then** o conteúdo textual é salvo em Markdown e o item surge no Acervo com a tag `#web-snapshot`.
2. **Given** uma URL que aponte para `localhost` ou rede privada (ex: `192.168.1.1`), **When** o sistema valida a URL, **Then** a requisição é rejeitada sumariamente para proteção contra SSRF (Princípio III da Constituição).

---

### Edge Cases

- **Queda de energia ou fechamento durante a cópia**: O protocolo de escrita atômica em duas fases garante que arquivos incompletos nunca sejam deixados como notas válidas.
- **Arquivos corrompidos ou com encoding inválido**: O ingestor tenta UTF-8 com fallback para perda segura de caracteres inválidos, notificando o usuário sem travar a aplicação.
- **Carregamento de notas muito grandes (> 10 MB)**: O sistema ingere o arquivo físico, mas trunca o preview da listagem para as primeiras 5.000 palavras, mantendo a performance da interface.
- **Navegação em Modo Web Simulado (Browser)**: No browser puro (`localhost:1420`), o pipeline de ingestão utiliza `localStorage` e mock local, informando o usuário via banner sem disparar exceções não tratadas.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O sistema DEVE gravar notas de texto e Markdown no diretório canônico `<vault_root>/ingest/notes/` sem alterar os bytes originais da fonte primária.
- **FR-002**: O sistema DEVE persistir os metadados de cada item ingerido na tabela `ingested_items` do SQLite (`index.db`), incluindo `id`, `source_type`, `canonical_path`, `title`, `summary`, `word_count`, `created_at` e `status`.
- **FR-003**: O sistema DEVE fornecer busca de texto completo (FTS5) sobre o título e conteúdo das notas indexadas no `index.db`.
- **FR-004**: O sistema DEVE armazenar imagens e arquivos binários no Content-Addressable Storage (`assets/<sha256>.<ext>`), indexando o hash para evitar redundância em disco.
- **FR-005**: Ao transferir ou promover um item do Acervo para a Mesa Espacial (Canvas), o sistema DEVE clonar o conteúdo de forma desacoplada (Fork-on-Insert) com citação de proveniência (`source_path`, `source_hash`).
- **FR-006**: O sistema DEVE validar URLs fornecidas pelo usuário contra SSRF, bloqueando endereços de loopback (`127.0.0.1`, `localhost`) e faixas de rede privada (RFC 1918) antes da extração web.
- **FR-007**: O sistema DEVE expor comandos IPC tipados no Tauri v2 (`ingest_file`, `ingest_file_content`, `ingest_url`, `list_ingested_items`, `promote_to_cell`) com tratamento seguro de erros.

### Key Entities

- **IngestedItem**: Representa um documento ou referência capturada. Atributos: `id` (UUID), `source_type` (`file` | `url` | `text`), `source_path` (caminho relativo ou URL original), `canonical_uri` (URI dentro do cofre), `title` (string), `summary` (string), `tags` (lista de termos), `word_count` (inteiro), `ingested_at` (timestamp ISO), `status` (`Ingested` | `Classified` | `Promoted`).
- **AssetRecord**: Representa um arquivo binário imutável no CAS. Atributos: `sha256_hash` (hex 64 chars), `extension` (string), `byte_size` (inteiro), `mime_type` (string), `created_at` (timestamp ISO).
- **WorkspaceCellReference**: Vínculo de proveniência inserido no nó do Canvas ao promover um item do Acervo. Atributos: `cell_id` (string), `source_item_id` (string), `source_path` (string), `source_hash` (string), `promoted_at` (timestamp ISO).

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: O tempo total de ingestão, cópia física para o disco e indexação no SQLite para um arquivo Markdown de até 1 MB DEVE ser inferior a 80 milissegundos.
- **SC-002**: A adição de um item do Acervo como novo nó na Mesa Espacial DEVE ocorrer em menos de 16 milissegundos (60 FPS contínuos).
- **SC-003**: 100% dos arquivos ingeridos DEVEM ser reconstruíveis a frio no `index.db` após a exclusão acidental do arquivo de banco de dados (`index.db`), sem perda de texto ou ativos.
- **SC-004**: Deduplicação de imagens idênticas DEVE alcançar 100% de reaproveitamento de armazenamento CAS sem duplicação de arquivos no disco.

---

## Assumptions

- O usuário possui permissão de escrita no diretório do cofre selecionado.
- Os arquivos Markdown de notas padrão variam entre 1 KB e 5 MB.
- A extração web básica opera offline com fallback gracioso quando a rede não estiver disponível.
- A autoridade canônica dos dados reside nos arquivos do cofre (`ingest/` e `assets/`), sendo o SQLite estritamente derivado e recriável.
