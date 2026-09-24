# Feature Specification: Mesa Espacial e Edição Modular (Canvas & BlockSuite)

**Feature Branch**: `004-spatial-canvas-modular`  
**Created**: 2026-09-24  
**Status**: Draft  
**Input**: User description: "Sprint 03: Mesa Espacial Avançada e Edição Modular (Edição de nós via overlay DOM sob demanda, criação de arestas/conexões interativas, persistência canônica de topologia e células em workspaces/<id>/ e renderização a 60 FPS com frustum culling)"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Edição Modular sob Demanda no Canvas via Overlay DOM (Priority: P1) 🎯 MVP

Como pesquisador organizando ideias na Mesa Espacial, quero dar um duplo clique (ou teclar Enter) em qualquer cartão/célula do Canvas para editar seu conteúdo textual de forma rica, garantindo que o motor editorial só seja instanciado sob demanda exatamente sobre o nó selecionado, mantendo a aplicação leve e veloz.

**Why this priority**: É a consagração do Princípio II da Constituição (Separação Cognitiva e Edição Modular). Em vez de renderizar centenas de editores pesados simultaneamente, instanciam-se no máximo 1 a 2 editores ativos no overlay DOM sincronizado com a viewport do PixiJS.

**Independent Test**:
- Clicar duas vezes em um nó de nota na Mesa.
- O editor modular surge posicionado exatamente sobre o retângulo do cartão com foco automático no cursor de texto.
- Digitar texto com markdown/títulos e clicar fora do cartão (blur) ou pressionar Esc.
- O editor desmonta suavemente, atualizando o preview visual do nó no PixiJS e persistindo o conteúdo localmente.

**Acceptance Scenarios**:
1. **Given** um nó existente na Mesa, **When** o usuário efetua duplo-clique sobre o cartão, **Then** o sistema ativa o overlay DOM de edição posicionado nas coordenadas de tela do nó.
2. **Given** um nó em edição ativa, **When** o usuário clica fora da célula ou pressiona Esc, **Then** o conteúdo é confirmado, o editor sob demanda é destruído e o preview gráfico no PixiJS é atualizado.
3. **Given** 500 nós na viewport, **When** o usuário navega pela tela, **Then** nenhum nó passivo instancia componentes de editor DOM, mantendo o consumo de memória estritamente controlado.

---

### User Story 2 - Criação e Gestão de Conexões Relacionais entre Nós (Priority: P2)

Como analista conectando evidências, quero arrastar uma linha a partir das portas de conexão (alças de ancoragem) de um cartão até outro cartão na Mesa, criando uma aresta visual com direção e rótulo semântico opcional.

**Why this priority**: Permite estruturar grafos de conhecimento visual e relacionamentos conceituais entre evidências coletadas no Acervo.

**Independent Test**:
- Passar o mouse sobre um cartão para revelar as portas de ancoragem (`top`, `bottom`, `left`, `right`).
- Clicar e arrastar da porta direita do "Nó A" até o "Nó B".
- Uma curva suave (bézier) é desenhada em tempo real conectando os dois cartões.
- Ao soltar o mouse sobre o "Nó B", a aresta é confirmada e registrada na topologia do workspace.

**Acceptance Scenarios**:
1. **Given** dois cartões na Mesa, **When** o usuário arrasta uma linha de conexão de uma borda do nó de origem até o nó de destino, **Then** uma aresta relacional é criada e desenhada conectando ambos.
2. **Given** nós conectados por arestas, **When** qualquer um dos nós é arrastado e reposicionado na Mesa, **Then** as curvas das arestas acompanham fluidamente o movimento em tempo real a 60 FPS.
3. **Given** uma aresta selecionada, **When** o usuário pressiona Delete ou Backspace, **Then** a aresta é excluída sem afetar o conteúdo ou a existência dos nós conectados.

---

### User Story 3 - Persistência Canônica da Topologia e Células do Workspace (Priority: P3)

Como usuário, quero que todas as alterações na Mesa (posições, nós, arestas e textos) sejam salvas no formato canônico aberto do Sandland (`workspaces/<id>/board.canvas.json` e notas em `workspaces/<id>/cells/<cell_id>.md`), protegidas contra conflitos de revisão concorrente (OCC).

**Why this priority**: Respeita o Princípio I (File-as-Truth) e Princípio VII (Journaling Atômico), assegurando que o trabalho espacial seja legível fora do app e protegido contra perda de dados.

**Independent Test**:
- Mover um nó e criar uma nova nota diretamente na Mesa.
- Verificar que o arquivo `workspaces/default-workspace/board.canvas.json` é atualizado atomicamente com a revisão incrementada.
- Verificar que o texto da célula é persistido em `workspaces/default-workspace/cells/<id>.md` com frontmatter YAML contendo metadados de criação e revisão.

**Acceptance Scenarios**:
1. **Given** mutações na topologia da Mesa, **When** o autosave atômico é disparado, **Then** o arquivo `board.canvas.json` é atualizado no disco via substituição atômica em duas fases.
2. **Given** uma edição de conteúdo em um nó, **When** a edição é concluída, **Then** o arquivo Markdown da célula correspondente em `cells/<id>.md` é gravado com frontmatter YAML e sincronizado com o índice SQLite.
3. **Given** um conflito de revisão OCC detectado no salvamento, **When** a versão em disco for mais recente que a local, **Then** o sistema notifica o usuário e executa reconciliação idempotente sem perda de digitação recente.

---

### User Story 4 - Navegação Espacial com Frustum Culling e Níveis de Detalhe (LOD) (Priority: P4)

Como pesquisador manipulando grandes volumes de pesquisa, quero navegar (pan e zoom de 0.1x a 3.0x) por uma Mesa contendo até 1.000 nós e dezenas de arestas com taxa estável de 60 FPS, adaptando o nível de detalhe dos cartões conforme o nível de zoom.

**Why this priority**: Atende aos requisitos de performance nativa do Princípio IV (Cena gráfica a 60 FPS com 1.000 nós no viewport com Frustum Culling e LOD ativados).

**Independent Test**:
- Adicionar ou carregar um workspace com 500+ nós.
- Executar pan e zoom rápido através da roda do mouse ou atalhos de teclado.
- Verificar que apenas os nós visíveis na viewport são desenhados pela GPU (Frustum Culling).
- Verificar que em níveis de zoom muito reduzidos (< 0.4x), o texto detalhado é substituído por barras de densidade ou blocos abstratos (LOD), preservando os 60 FPS.

**Acceptance Scenarios**:
1. **Given** um grande workspace com centenas de nós, **When** o usuário realiza pan ou zoom na Mesa, **Then** a renderização gráfica sustenta 60 FPS contínuos sem congelamentos.
2. **Given** nós localizados fora da janela visível da tela, **When** o PixiJS executa o ciclo de renderização, **Then** esses nós são descartados pelo Frustum Culling sem alocação desnecessária de draw calls na GPU.
3. **Given** nível de zoom inferior a 0.35x, **When** os cartões são visualizados, **Then** o modo LOD simplificado é exibido, mantendo legibilidade macro da topologia.

---

### Edge Cases

- **Deleção de nó com arestas conectadas**: Quando um nó é removido, todas as arestas de entrada e saída vinculadas a ele são automaticamente limpas da topologia para evitar arestas órfãs.
- **Auto-conexão (loop)**: Tentativas de conectar um nó a ele mesmo devem ser rejeitadas ou ignoradas visualmente.
- **Nós colidindo ou empilhados**: Ao soltar nós na mesma coordenada, o sistema aplica um deslocamento suave (*offset scatter*) de 20px para evitar sobreposição total imperceptível.
- **Queda de energia durante edição em overlay**: O conteúdo em digitação no overlay DOM é enviado ao buffer de autosave a cada 400ms, respeitando a tolerância máxima constitucional de 500ms de perda em caso de encerramento abrupto.
- **Redimensionamento da janela do aplicativo com overlay ativo**: As coordenadas do overlay DOM acompanham o redimensionamento e transformações de câmera da viewport imediatamente.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O sistema DEVE fornecer interação de edição modular sob demanda, instanciando o editor de blocos via overlay DOM sobre a célula selecionada e destruindo a instância ao sair do modo de edição (máximo de 1 a 2 instâncias ativas simultâneas).
- **FR-002**: O sistema DEVE permitir a criação de arestas direcionadas ou não direcionadas entre nós do Canvas através de arrasto a partir de portas de ancoragem (`top`, `bottom`, `left`, `right`).
- **FR-003**: As arestas DEVEM ser renderizadas como curvas suaves (Bézier cúbica) no PixiJS e acompanhar dinamicamente a movimentação dos nós em tempo real.
- **FR-004**: O sistema DEVE persistir a topologia declarativa canônica da Mesa no arquivo `workspaces/<id>/board.canvas.json`, contendo viewport, array de nós e array de arestas com controle de versão OCC (`revision`).
- **FR-005**: Cada célula de nota criada ou promovida na Mesa DEVE ser persistida canonicamente como arquivo Markdown em `workspaces/<id>/cells/<cell_id>.md`, contendo frontmatter YAML com metadados de proveniência (`item_id`, `source_path`, `source_hash`, `revision`).
- **FR-006**: O sistema DEVE implementar Frustum Culling geométrico no PixiJS para descartar do pipeline de renderização nós e arestas que estejam totalmente fora da viewport visível.
- **FR-007**: O sistema DEVE implementar pelo menos dois Níveis de Detalhe (LOD): modo detalhado (zoom $\ge 0.4x$, com título, snippet e badges) e modo simplificado (zoom $< 0.4x$, com bloco colorido e densidade geométrica).
- **FR-008**: O sistema DEVE fornecer controles de viewport intuitivos: pan livre (arrastar com botão do meio ou espaço + arrasto), zoom contínuo centralizado no cursor (0.1x a 3.0x), reset de zoom e botão de recentralizar nós (*fit to view*).

### Key Entities

- **CanvasNode**: Representa um cartão visual na Mesa Espacial. Atributos: `id` (string), `item_id` (opcional, link de proveniência), `local_cell_path` (caminho para `cells/<id>.md`), `title` (string), `content` (string), `x` (f32), `y` (f32), `width` (f32), `height` (f32), `color_preset` (opcional), `node_type` (`note` | `asset` | `text`).
- **CanvasEdge**: Representa uma conexão relacional entre dois nós. Atributos: `id` (string), `source_node_id` (string), `target_node_id` (string), `from_side` (`Top` | `Bottom` | `Left` | `Right`), `to_side` (`Top` | `Bottom` | `Left` | `Right`), `label` (opcional), `directed` (booleano).
- **BoardTopology**: Estrutura declarativa canônica do workspace. Atributos: `workspace_id` (string), `viewport` (`x`, `y`, `zoom`), `nodes` (lista de `CanvasNode`), `edges` (lista de `CanvasEdge`), `revision` (inteiro), `updated_at` (timestamp Unix).

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A ativação e desmontagem do overlay DOM de edição sob demanda DEVE ocorrer em menos de 50 milissegundos, sem vazamento de elementos no DOM após o fechamento.
- **SC-002**: A renderização gráfica na viewport DEVE sustentar 60 FPS contínuos durante operações de pan e zoom com 1.000 nós no workspace com Frustum Culling e LOD ativados.
- **SC-003**: 100% das mutações de topologia e edição de células DEVEM ser gravadas atomicamente no disco sem corrupção de arquivos em caso de crash do processo.
- **SC-004**: O arraste simultâneo de nós conectados deve atualizar as arestas sem atraso visual perceptível (latência de re-layout de aresta $\le 4$ ms).

---

## Assumptions

- O usuário utiliza aceleração gráfica padrão de WebGL / WebGPU suportada pelo WebView do Tauri v2.
- A maioria das Mesas de Trabalho individuais possui entre 20 e 500 nós ativos.
- O salvamento automático com debounce de 400ms é suficiente para garantir persistência contínua sem sobrecarregar o subsistema de I/O em disco.
- Os dados do workspace são armazenados estritamente na pasta local do cofre (`<vault_root>/workspaces/<id>/`).
