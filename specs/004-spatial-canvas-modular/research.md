# Research & Technical Decisions: Mesa Espacial e Edição Modular

**Feature**: `004-spatial-canvas-modular`  
**Date**: 2026-09-24  
**Status**: Approved  

## Research Decisions

### Decision 1: Arquitetura de Overlay DOM Sob Demanda para Edição (Princípio II da Constituição)

- **Decision**: A cena gráfica espacial (PixiJS v8) gerencia câmera, pan, zoom, seleção, movimentação de caixas, conexões e renderização acelerada por hardware de todos os nós em repouso. O motor editorial rico é instanciado **estritamente sob demanda** em um elemento overlay DOM sincronizado com a viewport quando o usuário entra em modo de edição (duplo clique ou tecla Enter), limitando a concorrência a no máximo 1 a 2 instâncias ativas simultâneas. Ao concluir a edição (blur ou tecla Esc), o estado é confirmado, o editor é destruído e a textura do nó no PixiJS é atualizada.
- **Rationale**: A Constituição v2.0 (Princípio II - Separação Cognitiva e Edição Modular) proíbe categoricamente a instanciação de centenas de Web Components ou editores DOM simultâneos no canvas, pois isso causaria degradação severa de memória (> 500 MB) e queda de performance. O overlay dinâmico sob demanda garante consumo de memória em repouso inferior a 350 MB e 60 FPS sustentados.
- **Alternatives considered**:
  - *Renderizar editores HTML nativos em cada cartão*: Rejeitado por consumir centenas de megabytes de RAM e derrubar o framerate para < 20 FPS com 500 nós.
  - *Editor embutido inteiramente na GPU/Canvas*: Rejeitado pela impossibilidade de seleção nativa de texto do SO, ausência de suporte a IME (chinês/japonês/acentos), perda de acessibilidade e quebra de ergonomia de digitação.

---

### Decision 2: Geometria de Conexões e Arestas Relacionais no PixiJS (Drag & Connect)

- **Decision**: As arestas entre cartões são desenhadas como curvas de Bézier cúbica no layer inferior de gráficos do PixiJS (`GraphicsContainer`), ligando portas de ancoragem discretas (`Top`, `Bottom`, `Left`, `Right`) calculadas nos centros geométricos de cada borda dos nós. O usuário pode puxar interativamente uma linha com snap magnético a partir de qualquer porta até outro nó.
- **Rationale**: Curvas Bézier oferecem clareza visual analítica superior para diagramas de afinidade e grafos de evidências, contornando sobreposições com elegância. O cálculo em GPU no PixiJS garante que o reposicionamento de nós recalcula e redesenha dezenas de arestas com latência $\le 4$ ms, mantendo os 60 FPS contínuos.
- **Alternatives considered**:
  - *Linhas ortogonais retas (estilo circuito)*: Rejeitado nesta fase devido à complexidade desnecessária de roteamento que não agrega valor imediato à exploração analítica do usuário.
  - *Overlay SVG para arestas*: Rejeitado por perda de sincronia de renderização com a cena PixiJS durante operações rápidas de pan/zoom.

---

### Decision 3: Persistência Canônica Dupla: Topologia Declarativa e Células Markdown (Princípio I e VII)

- **Decision**: 
  1. A geometria da Mesa (viewport, lista de nós, coordenadas, dimensões, cores e lista de arestas) é salva no arquivo canônico `workspaces/<id>/board.canvas.json` com número de revisão sequencial (OCC).
  2. O conteúdo textual autoral de cada célula criada ou promovida na Mesa é persistido canonicamente como nota individual em `workspaces/<id>/cells/<cell_id>.md` com frontmatter YAML (`item_id`, `source_path`, `source_hash`, `revision`, `created_at`).
  3. Toda gravação utiliza o protocolo atômico em duas fases (`write temp` $\rightarrow$ `FlushFileBuffers/fsync` $\rightarrow$ `rename`).
- **Rationale**: Em total alinhamento com a Soberania do Dado (Princípio I), as notas de trabalho do usuário são sempre arquivos Markdown legíveis por qualquer editor externo. A separação entre topologia (`board.canvas.json`) e corpo da célula (`cells/*.md`) viabiliza modularidade e controle de versão atômico.
- **Alternatives considered**:
  - *Salvar todo o conteúdo de texto dentro do JSON de topologia*: Rejeitado porque violaria o princípio File-as-Truth de notas individuais desacopladas.
  - *Salvar alterações de células de volta na pasta de ingestão `ingest/notes/`*: Rejeitado pelo Princípio II (Mecânica de Fork-on-Insert Estendido).

---

### Decision 4: Frustum Culling Geométrico e Níveis de Detalhe (LOD) a 60 FPS (Princípio IV)

- **Decision**: O renderizador calcula a caixa delimitadora (`AABB`) da janela visível em coordenadas mundo a cada alteração de câmera.
  - **Frustum Culling**: Nós e arestas cujos limites não interceptam a viewport visível têm `visible = false` configurado no nó PixiJS, sendo completamente descartados das draw calls de GPU.
  - **Level of Detail (LOD)**:
    - *Zoom Normal / Próximo ($\ge 0.4x$)*: Renderização com título completo, snippets de texto legíveis e badges.
    - *Zoom Distante / Macro ($< 0.4x$)*: Oculta texto miúdo, renderizando apenas blocos geométricos sólidos com suas respectivas cores de identificação e conexões macro.
- **Rationale**: Assegura cumprimento estrito dos Quality Gates da Constituição (60 FPS com 1.000 nós no canvas e tempo de inicialização ultrarrápido).
- **Alternatives considered**:
  - *Desativar renderização de texto sempre que a câmera se mover*: Rejeitado por causar cintilação visual (*flickering*) desagradável durante o pan suave.
