# Quickstart & Validation Guide: Núcleo Local-First e Canvas Básico (v0.1)

**Feature**: `001-local-first-canvas`  
**Date**: 2026-09-21 (Revisão Técnica)  
**Status**: Ready  

Este guia detalha o roteiro para validar ponta a ponta os requisitos da v0.1: inicialização do runtime desktop Tauri v2 + Svelte 5, ingestão com máquina de estados, auto-tagging local com GBNF, geração desacoplada de embeddings via `fastembed` e navegação no canvas híbrido PixiJS v8 com overlay DOM.

---

## 1. Pré-requisitos do Ambiente

- **Rust**: Toolchain estável `1.80+` (`cargo`, `rustc`).
- **Node.js & Gerenciador de Pacotes**: `pnpm` (Node.js 20+).
- **Dependências de Sistema do Tauri v2**:
  - *Windows*: Visual Studio C++ Build Tools, WebView2 Runtime (nativo no Windows 11/10).
  - *Linux*: `libwebkit2gtk-4.1-dev`, `build-essential`, `curl`, `wget`, `file`, `libssl-dev`.
- **Modelos Locais**:
  - `Qwen2.5-1.5B-Instruct-Q4_K_M.gguf` para auto-tagging generativo.
  - `bge-small-en-v1.5` (ONNX) para embeddings semânticos rápidos via `fastembed`.
  *(Ambos são baixados automaticamente via gerenciador na primeira execução com barra de progresso no Ingest drawer).*

---

## 2. Configuração e Inicialização

1. **Instalar dependências do frontend (Svelte 5 + PixiJS v8)**:
   ```bash
   pnpm install
   ```

2. **Compilar e executar em modo de desenvolvimento**:
   ```bash
   pnpm tauri dev
   ```

3. **Verificação de Inicialização**:
   - O aplicativo deve abrir em menos de 2.2 segundos.
   - O diretório `/vault/` e os subdiretórios `/vault/.system/`, `/vault/ingest/notes/` e `/vault/workspaces/` devem ser criados automaticamente no primeiro boot.
   - O SQLite `index.db` aplica as migrações `refinery` criando `items`, `items_fts`, `vec_items` e `taxonomy_terms`.

---

## 3. Cenários de Validação Ponta a Ponta

### Cenário 1: Ingestão de Documento e Máquina de Estados

1. **Ação**: Soltar um arquivo de texto sobre concorrência em Rust (`teste-concorrencia.md`) na gaveta lateral de Ingest do app, ou salvá-lo diretamente na pasta do disco `/vault/ingest/notes/`.
2. **Resultado Esperado**:
   - O evento `ingest://state-changed` transiciona de `Pending` -> `Extracting` -> `Classifying`.
   - O arquivo é copiado para o cofre e registrado no `index.db` com hash SHA-256 e Frontmatter YAML padronizado em menos de 1s.
   - A geração de embeddings via `fastembed` conclui em < 15ms.
   - A inferência SLM via `llama.cpp` com amostragem GBNF conclui a classificação temática com transição para `Classified`.

### Cenário 2: Adição de Nó e Edição com Overlay DOM

1. **Ação**:
   - Clicar e arrastar o item recém-ingerido da gaveta de entrada para o centro do Canvas.
   - Dar duplo clique no cartão para editar o texto.
2. **Resultado Esperado**:
   - O PixiJS desenha o cartão com 60 FPS estáveis.
   - No duplo clique, o Svelte 5 projeta dinamicamente um micro-editor `<textarea>` posicionado exatamente sobre o nó.
   - Ao digitar e clicar fora (`blur`), o texto é sincronizado com o nó e salvo no arquivo MessagePack `board.canvas.mpk` em menos de 350ms.
   - Após 2 segundos sem mexer o mouse, o arquivo legível `board.canvas.json` é gravado com as novas coordenadas.

### Cenário 3: Validação do Level-of-Detail (LOD)

1. **Ação**: Utilizar a roda do mouse (*scroll wheel*) para afastar a câmera progressivamente até zoom inferior a $0.3$.
2. **Resultado Esperado**:
   - Zoom $> 0.6$: Cartão exibe título, resumo e tags formatadas.
   - Zoom $0.3 \le Z \le 0.6$: O corpo e resumo somem; apenas o título e o contorno colorido da categoria permanecem.
   - Zoom $< 0.3$: O cartão colapsa para uma caixa sólida simplificada sem texto (*proxy card*), mantendo a taxa em 60 FPS com dezenas de cartões na tela.

### Cenário 4: Resiliência contra Quedas (Crash Recovery)

1. **Ação**: Modificar posições no canvas e encerrar o processo abruptamente via gerenciador de tarefas (`kill -9` ou `Stop-Process`).
2. **Resultado Esperado**:
   - Ao reabrir com `pnpm tauri dev`, a cena carrega na mesma posição exata do último evento registrado no `audit_log.db` com tolerância inferior a 500ms de alteração.
