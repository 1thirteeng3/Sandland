# Quickstart & Cenários de Validação: Mesa Espacial e Edição Modular

**Feature**: `004-spatial-canvas-modular`  
**Date**: 2026-09-24  
**Status**: Ready for Validation  

Este guia detalha os cenários executáveis e testes ponta a ponta para validar a entrega da **Mesa Espacial e Edição Modular (Sprint 03)** no Sandland v2.0.

---

## 🛠️ Pré-requisitos e Execução

### Modo Desktop Nativo (Tauri v2)
```powershell
npm run tauri dev
```

### Modo Simulação Web (Navegador)
```powershell
npm run dev
# Acesse no navegador: http://localhost:1420
```

---

## 🧪 Cenários de Validação Ponta a Ponta

### Cenário 1: Edição de Célula no Canvas via Overlay Modular sob Demanda (MVP)
**Objetivo**: Validar que o editor só é montado sobre o nó selecionado e destruído ao sair, mantendo a cena leve (Princípio II).
1. Abra a aplicação e selecione ou crie um cofre local.
2. Na Mesa Espacial, clique no botão **"➕ Nota"** na barra de ferramentas superior para criar um novo cartão.
3. Dê **duplo-clique** sobre o novo cartão (ou selecione-o e pressione `Enter`).
4. **Verificação**:
   - Um container HTML de overlay surge exatamente sobre o retângulo do cartão com foco imediato no cursor.
   - Digite um novo título e corpo em Markdown: `# Hipótese 1\n\nTexto demonstrativo de pesquisa.`.
   - Clique em qualquer área vazia da Mesa (blur) ou pressione `Esc`.
   - O editor desmonta suavemente.
   - O cartão no PixiJS exibe o novo texto renderizado com tipografia nítida.

---

### Cenário 2: Criação Interativa de Arestas Relacionais (Drag & Connect)
**Objetivo**: Conectar dois cartões na Mesa através de alças de ancoragem com curvas Bézier.
1. Crie dois cartões na Mesa ("Cartão A" e "Cartão B").
2. Aproxime o cursor da borda direita do "Cartão A" até que o ponto/porta de ancoragem fique visível.
3. Clique e arraste o mouse a partir da alça do "Cartão A" em direção ao "Cartão B".
4. **Verificação**:
   - Uma curva Bézier elástica e suave acompanha a ponta do cursor em tempo real.
   - Ao soltar sobre a borda esquerda do "Cartão B", a aresta se fixa magneticamente.
   - Uma seta direcional indica o sentido da relação de A para B.
5. Mova o "Cartão A" pela tela: a curva Bézier acompanha a movimentação instantaneamente a 60 FPS.

---

### Cenário 3: Persistência Canônica no Disco (`board.canvas.json` e `cells/*.md`)
**Objetivo**: Comprovar que a topologia e o texto das células são persistidos como arquivos reais no cofre (Princípio I).
1. Realize alterações na Mesa (adicione notas, conecte arestas e mova posições).
2. Abra o explorador de arquivos e navegue até a pasta do cofre aberto:
   - `<seu_cofre>/workspaces/default-workspace/board.canvas.json`
   - `<seu_cofre>/workspaces/default-workspace/cells/`
3. **Verificação**:
   - `board.canvas.json` contém os nós (`nodes`), arestas (`edges`) e a revisão incrementada (`revision`).
   - Dentro de `cells/`, arquivos `.md` individuais contêm o frontmatter YAML (`id`, `workspace_id`, `node_id`, `created_at`) e o texto digitado.

---

### Cenário 4: Frustum Culling e Níveis de Detalhe (LOD) a 60 FPS
**Objetivo**: Validar que a renderização é acelerada e descarta nós invisíveis da GPU.
1. Crie ou carregue múltiplos nós na tela (use pan e zoom amplo).
2. Afaste a câmera com a roda do mouse até que o zoom fique abaixo de `0.35x`.
3. **Verificação**:
   - Os cartões transitam para o modo LOD simplificado (apenas blocos geométricos sólidos e rótulos macro).
   - Ao aproximar a câmera ($zoom \ge 0.4x$), os cartões voltam a renderizar títulos, trechos e detalhes com total nitidez.
   - As operações de pan e zoom não apresentam engasgos (*stuttering*), sustentando 60 FPS.
