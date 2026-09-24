# Feature Specification: Peça Editorial e Proveniência Canônica (Fork-on-Insert Estendido)

**Feature Branch**: `005-editorial-piece-provenance`  
**Feature Directory**: `specs/005-editorial-piece-provenance`  
**Created**: 2026-09-24  
**Status**: Draft  
**Input**: Produção Editorial de Peças (`workspaces/<id>/pieces/*.md`) com Fork-on-Insert Estendido e Rastreabilidade de Proveniência a partir dos cartões da Mesa Espacial (Princípio II da Constituição).

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Criação e Redação de Peças Editoriais no Workspace (Priority: P1) 🎯 MVP

Como pesquisador ou autor analítico, quero criar e redigir documentos longos e estruturados denominados "Peças" dentro do workspace (`workspaces/<id>/pieces/<piece_id>.md`), com foco na escrita contínua, salvamento atômico durável e visualização organizada de seções.

**Why this priority**: É a fundação editorial do Sandland. O valor primário do acervo e da exploração espacial é culminar em uma peça de síntese autoral preservada soberanamente em arquivos abertos (Princípio I da Constituição).

**Independent Test**: Criar uma nova Peça "Relatório de Arquitetura", redigir parágrafos e títulos, fechar e reabrir o aplicativo para confirmar que o arquivo físico `workspaces/<id>/pieces/<piece_id>.md` foi persistido intacto com frontmatter canônico.

**Acceptance Scenarios**:
1. **Given** um workspace aberto, **When** o usuário clica em "Nova Peça" na barra de navegação editorial, **Then** o sistema gera `workspaces/<workspace_id>/pieces/<piece_id>.md` e abre o editor de texto focado.
2. **Given** uma Peça aberta com texto em digitação, **When** passam 400ms após a última tecla ou o usuário fecha a aba, **Then** as alterações são gravadas atomicamente no disco sem perda de dados.
3. **Given** múltiplas Peças no workspace, **When** o usuário alterna entre elas na barra lateral de Peças, **Then** o conteúdo de cada arquivo é carregado instantaneamente com seus títulos e metadados.

---

### User Story 2 - Fork-on-Insert da Mesa para a Peça com Citação Canônica (Priority: P2)

Como autor compilando evidências, quero transferir um cartão ou trecho da Mesa Espacial diretamente para a Peça Editorial (via arrastar e soltar ou comando de inserção), de modo que o texto seja clonado de forma totalmente independente e os metadados de proveniência sejam gravados canonicamente no frontmatter da Peça.

**Why this priority**: Concretiza o Princípio II da Constituição (*Separação Cognitiva e Fork-on-Insert Estendido*). Garante que a síntese na Peça tenha rastreabilidade estrita de onde veio cada ideia sem acoplamento frágil.

**Independent Test**: Arrastar um nó da Mesa com título "Arquitetura Local-First" para o editor da Peça, confirmar a inserção do texto como um novo bloco com indicador visual de citação e validar que o frontmatter YAML da Peça registrou a citação estruturada (`source_cell_id`, `source_revision`, `quote`, `inserted_at`).

**Acceptance Scenarios**:
1. **Given** a Mesa Espacial e uma Peça aberta lado a lado (modo split-view), **When** o usuário arrasta um nó da Mesa para o corpo da Peça, **Then** o conteúdo é copiado para a posição do cursor com um badge de proveniência.
2. **Given** a inserção de um nó via Fork-on-Insert, **When** o frontmatter da Peça é inspecionado no disco, **Then** ele contém a lista `citations` com `citation_id`, `source_cell_id`, `source_revision`, `source_title`, `quote` e `inserted_at`.
3. **Given** um nó já inserido na Peça, **When** o usuário edita ou altera a célula original na Mesa Espacial, **Then** o texto dentro da Peça permanece inalterado (imunidade a mutações retroativas automáticas).

---

### User Story 3 - Auditoria de Proveniência e Detecção de Divergência (Drift Detection) (Priority: P3)

Como revisor de conteúdo, quero clicar no badge de proveniência de um bloco na Peça para visualizar o cartão original na Mesa Espacial e ser alertado caso o cartão original tenha sido modificado posteriormente na Mesa.

**Why this priority**: Permite que o autor verifique a coerência das evidências ao longo do tempo. Se novas descobertas alterarem uma nota na Mesa, o autor deve ser conscientemente informado sem que sua Peça seja sobrescrita silenciosamente.

**Independent Test**: Modificar o texto de um nó na Mesa que já foi citado na Peça, abrir a Peça, verificar o indicador de status "Evidência Atualizada na Mesa" (Drift detectado) e visualizar a comparação lado a lado entre o texto citado e o texto atual do nó.

**Acceptance Scenarios**:
1. **Given** um bloco citado na Peça, **When** o usuário clica no badge de proveniência, **Then** o painel de contexto exibe os dados da citação e permite focar/centralizar a câmera da Mesa no nó de origem.
2. **Given** que o nó original teve sua revisão incrementada na Mesa (`current_revision > source_revision`), **When** o usuário inspeciona a citação na Peça, **Then** um indicador visual de divergência (*drift*) é apresentado.
3. **Given** a divergência detectada, **When** o usuário clica em "Atualizar Citação", **Then** o trecho na Peça é sincronizado com a versão mais recente e o frontmatter registra a nova `source_revision`.

---

### User Story 4 - Exportação Limpa e Compilação com Referências (Priority: P4)

Como autor pronto para publicar ou compartilhar, quero exportar a Peça Editorial para Markdown puro (com opção de apêndice bibliográfico gerado automaticamente a partir das citações) ou copiar para o clipboard.

**Why this priority**: Fecha o ciclo de vida do conhecimento no Sandland, permitindo que a pesquisa estruturada saia do ecossistema local para publicação externa limpa e auditada.

**Independent Test**: Acionar "Exportar Peça", selecionar a opção "Incluir Notas de Proveniência" e verificar a geração de um arquivo `.md` contendo o corpo do texto e notas de rodapé numeradas remetendo aos itens de origem.

**Acceptance Scenarios**:
1. **Given** uma Peça com 3 citações de cartões da Mesa, **When** o usuário escolhe exportar como Markdown compilado, **Then** o sistema gera o texto formatado incluindo uma seção de rodapé "## Referências & Proveniência".
2. **Given** a ação de copiar texto, **When** o usuário seleciona "Copiar Markdown Limpo", **Then** o texto copiado para a área de transferência remove os metadados internos de citação mantendo a formatação pura.

---

## Edge Cases

- **Nó de origem excluído da Mesa**: Se o nó de origem for deletado do canvas posteriormente, a Peça continua preservando o texto citado e os metadados de histórico (`source_cell_id`), exibindo status de "Origem arquivada/removida" sem corromper a Peça.
- **Inserção de trecho parcial vs nó inteiro**: O usuário deve poder selecionar apenas uma frase de um cartão e arrastá-la, gravando no metadado o trecho específico (`quote`).
- **Nomes de Peças duplicados**: A criação de peças com títulos repetidos gera identificadores únicos de arquivo (`artigo-analitico-1.md`, `artigo-analitico-2.md`) sem colisões.
- **Tamanho excessivo da Peça**: Documentos com mais de 50.000 palavras devem manter rolagem fluida e salvamento em segundo plano sem travar a interface gráfica.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O sistema DEVE persistir todas as peças em arquivos físicos individuais dentro do diretório `workspaces/<workspace_id>/pieces/<piece_id>.md` (Princípio I da Constituição).
- **FR-002**: O sistema DEVE suportar um editor de texto estruturado com atalhos de Markdown (títulos `#`, listas `-`, ênfases `**`, blocos de código e citações).
- **FR-003**: O sistema DEVE implementar a mecânica de Fork-on-Insert estendido, clonando o texto arrastado ou promovido para a Peça e gravando metadados de citação no frontmatter YAML da Peça (Princípio II da Constituição).
- **FR-004**: Cada citação registrada no frontmatter DEVE conter obrigatoriamente: `citation_id`, `source_cell_id`, `source_revision`, `quote`, `quote_hash` e `inserted_at`.
- **FR-005**: O sistema NUNCA DEVE modificar ou substituir automaticamente o texto citado na Peça quando o nó na Mesa sofrer edições posteriores.
- **FR-006**: O sistema DEVE fornecer detecção ativa de divergência (*drift detection*), comparando a `source_revision` registrada na citação com a revisão atual da célula correspondente na pasta `cells/*.md`.
- **FR-007**: O sistema DEVE disponibilizar uma visualização dividida (Split-View) que permita visualizar e interagir com a Mesa Espacial e a Peça Editorial lado a lado.
- **FR-008**: O sistema DEVE oferecer navegação bidirecional: a partir de um bloco de citação na Peça, o usuário pode clicar para centralizar e destacar a célula de origem no PixiJS da Mesa.
- **FR-009**: O sistema DEVE suportar exportação da Peça para Markdown com opção de compilação de referências bibliográficas ao final do documento.
- **FR-010**: As gravações em disco das Peças DEVEM ser atômicas com fsync antes do rename, garantindo a tolerância a quedas com janela máxima de 500ms (Princípio VII da Constituição).

---

### Key Entities

- **EditorialPiece**: Representa a unidade autoral longa (`workspaces/<id>/pieces/<piece_id>.md`).
  - Atributos: `id`, `workspace_id`, `title`, `slug`, `content`, `citations: Vec<CitationRecord>`, `word_count`, `created_at`, `updated_at`.
- **CitationRecord**: Registro estruturado de rastreabilidade de proveniência de um trecho citado.
  - Atributos: `citation_id`, `source_cell_id`, `source_revision`, `source_title`, `quote`, `quote_hash`, `inserted_at`.
- **CitationDrift**: Estado calculado de coerência entre a citação na Peça e o estado da célula na Mesa.
  - Valores: `Synchronized` (revisão coincide), `Diverged` (célula foi atualizada na Mesa), `Orphaned` (célula foi excluída da Mesa).

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: O salvamento automático da Peça ocorre com debounce de 400ms sem travamento na digitação do usuário.
- **SC-002**: O tempo para inserir um nó via Fork-on-Insert e registrar seus metadados de proveniência na Peça é inferior a 150 milissegundos.
- **SC-003**: 100% das inserções via Fork-on-Insert registram dados íntegros de proveniência no frontmatter YAML do arquivo `.md`.
- **SC-004**: A alternância ou visualização lado a lado (Split-View) entre a Mesa a 60 FPS e a Peça Editorial mantém consumo de memória abaixo do teto constitucional de 350 MB em repouso.
- **SC-005**: A detecção de divergência (*drift detection*) identifica com 100% de acurácia células modificadas na Mesa sem executar verificações de rede ou chamadas bloqueantes.

---

## Assumptions

- O usuário redige em Markdown padrão sem necessidade de dependências pesadas de processadores de texto binários (DOCX/PDF nativos).
- A Mesa Espacial (`004-spatial-canvas-modular`) já provê persistência de células em `cells/<id>.md` com revisão sequencial (`revision: u64`), servindo como base estável para citação.
- O modo Split-View redimensiona dinamicamente a viewport do PixiJS ajustando a matriz de projeção sem necessidade de reinicializar a aplicação gráfica.
- Exportação inicial foca em formato Markdown limpo e Markdown com notas de rodapé padronizadas.
