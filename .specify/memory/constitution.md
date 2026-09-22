<!--
Sync Impact Report:
- Version change: Initial scaffold -> 1.0.0
- List of modified principles:
  - [PRINCIPLE_1_NAME] -> I. Soberania do Dado (File-as-Truth & Local-First)
  - [PRINCIPLE_2_NAME] -> II. Separação Cognitiva e Modularidade Editorial (Fork-on-Insert)
  - [PRINCIPLE_3_NAME] -> III. Contenção Rigorosa, Sandboxing OS-Level e I/O Seguro (NON-NEGOTIABLE)
  - [PRINCIPLE_4_NAME] -> IV. Desempenho Nativo e Eficiência Estrita de Recursos
  - [PRINCIPLE_5_NAME] -> V. Arquitetura RLM, Estruturação Determinística e Integridade de IA
  - Added: VI. Resiliência Operacional, Auditoria e Versionamento Duplo
- Added sections:
  - Technical Constraints & Security Standards (Technology stack, network & proxy policies, storage schemas)
  - Development Workflow & Quality Gates (CI/CD gates, crash recovery guarantees, test-first standards)
- Removed sections:
  - Generic template placeholder sections
- Follow-up TODOs: None
-->

# Sandland Constitution

## Core Principles

### I. Soberania do Dado (File-as-Truth & Local-First)
O sistema de arquivos local é a única e definitiva fonte da verdade (*single source of truth*). Todo o conteúdo do usuário DEVE ser persistido em arquivos Markdown legíveis (`.md`) com YAML Frontmatter padronizado, e arquivos binários/mídias DEVEM ser armazenados no Content-Addressable Storage (`/assets/`) com nomes derivados de seus hashes SHA-256. Bancos de dados relacionais e vetoriais (SQLite e `sqlite-vec`) atuam exclusivamente como índices derivados, descartáveis e reconstruíveis a qualquer momento a partir do sistema de arquivos. O sistema DEVE funcionar em modo 100% autônomo e offline, sem dependência mandatória de conexões de rede ou serviços de nuvem para suas operações centrais.

### II. Separação Cognitiva e Modularidade Editorial (Fork-on-Insert)
Cada etapa do ciclo de vida da informação (Ingestão, Exploração Espacial no Whiteboard, Extração Web e Produção Editorial na Peça) DEVE manter fronteiras lógicas estritas e isoladas. O subsistema de Ingestão DEVE operar de maneira passiva e somente-leitura (*append-only*), sem exigir organização a priori do usuário. A transição de insights entre o Whiteboard e a Peça DEVE obrigatoriamente seguir a mecânica de *Fork-on-Insert*: ao mover uma célula atômica para o editor editorial, o conteúdo DEVE ser clonado de forma independente, registrando metadados explícitos de citação (`source_cell_id`, `inserted_at`, `original_snippet`) no frontmatter para garantir rastreabilidade histórica e imutabilidade dos nós de origem.

### III. Contenção Rigorosa, Sandboxing OS-Level, I/O Seguro e Portabilidade (NON-NEGOTIABLE)
Qualquer rotina de execução autônoma, sub-agente de IA, thread de inferência ou motor headless de extração web DEVE operar sob jaulas de segurança nativas do sistema operacional com privilégio mínimo (Landlock LSM no Linux, Seatbelt Profiles no macOS e Job Objects restritos no Windows). O acesso ao sistema de arquivos DEVE ser rigidamente limitado ao subdiretório canônico do workspace e à pasta de assets correspondente. Tentativas de resolução fora dos limites estabelecidos DEVEM resultar em rejeição imediata (`SecurityError::SandboxEscapeAttempt`). Toda operação de I/O em arquivos DEVE neutralizar vulnerabilidades TOCTOU (*Time-of-Check to Time-of-Use*) resolvendo caminhos canônicos e abrindo diretamente descritores de arquivos atômicos antes de qualquer leitura ou escrita. **É expressamente proibida a presença de qualquer caminho de arquivo absoluto hardcoded** no código-fonte ou scripts de build; toda resolução de cofre e pastas temporárias DEVE ser dinâmica via chamadas de sistema operacional (`app.path().home_dir()`, `app.path().document_dir()`).

### IV. Desempenho Nativo e Eficiência Estrita de Recursos
O núcleo de dados, rede, orquestração de IA e manipulação de arquivos DEVE ser implementado estritamente em Rust nativo, integrado via Tauri v2, sem runtimes ou interpretadores pesados em segundo plano. O consumo de memória RAM do sistema em repouso (*idle*) com até 100 nós no canvas NÃO PODE exceder 350 MB. A latência de inicialização a frio até a prontidão interativa do canvas DEVE ser inferior a 2.2 segundos em discos NVMe. O motor gráfico do Whiteboard (PixiJS/WebGL) DEVE sustentar 60 FPS estáveis para mais de 1000 nós por meio de Frustum Culling e Level-of-Detail (LOD). Modelos locais de inferência DEVEM ser sumariamente descarregados (*unloaded*) da VRAM/RAM após 5 minutos de inatividade.

### V. Arquitetura RLM, Vectorless RAG para a Mesa e Integridade de IA
Para contornar restrições de contexto e preservar fidelidade cognitiva, a Mesa de Trabalho (Canvas) DEVE adotar o paradigma **Vectorless RAG (baseado no conceito PageIndex / RLM)**. As células, grupos visuais e arestas NÃO PODEM ser convertidas em vetores densos nem fragmentadas em chunks cegos; o contexto analítico é isolado por workspace/board e sintetizado em tempo de execução via **Skeleton Map** (mapa em árvore de seções, grupos, nós, arestas e resumos) derivado do `topology.json` e injetado diretamente na janela de contexto de modelos de linguagem. O uso de embeddings densos (`sqlite-vec` via `fastembed` 384d) e busca lexical BM25 (`fts5`) é restrito ao catálogo do Acervo de documentos brutos. A camada de inferência DEVE ser agnóstica a modelos através da trait `ModelProvider`, suportando execução local offline (`LocalLlamaProvider`) e provedores de nuvem via chaves do usuário (`CloudApiProvider` / BYOK). Classificações taxonômicas DEVEM utilizar amostragem guiada por gramática (GBNF) para garantir JSON estritamente tipado.

### VI. Resiliência Operacional, Auditoria e Versionamento Duplo
O Sandland DEVE implementar uma estratégia de versionamento em duas camadas: uma camada de eventos local de alta frequência (`audit_log.db`), capaz de garantir recuperação de estado após falhas bruscas de energia com perda máxima de 500 milissegundos de digitação, e uma camada Git opcional para colaboração, backup e sincronização multi-máquina. A persistência da topologia do canvas DEVE utilizar serialização binária MessagePack (`board.canvas.mpk`) a cada 350ms para garantir fluidez na interface, gerando o arquivo declarativo legível `board.canvas.json` de forma assíncrona após 2 segundos de inatividade do usuário para evitar conflitos de merge no Git. Células do canvas vivem como nós estruturados em `topology.json`, evitando a criação automática de micro-arquivos no disco até que o comando explícito "Promover Célula a Nota" seja acionado pelo usuário.

## Technical Constraints & Security Standards

1. **Stack Tecnológica Mandatória**:
   - Backend: Rust (última versão estável), Tauri v2.
   - Banco de Dados e Busca: SQLite embarcado com extensões `fts5` (tokenizador `unicode61`) e `sqlite-vec` para busca híbrida no Acervo unificada via Reciprocal Rank Fusion (RRF, com $k = 60$).
   - Mesa de Pesquisa: Paradigma Vectorless RAG estruturado sobre `topology.json` com geração em runtime do Skeleton Map.
   - Frontend / UI: Webview moderna com renderização acelerada por hardware via WebGL / PixiJS para o canvas, interface tipográfica modular para a Peça.
   - Abstração de IA (Model-Agnostic & BYOK): Trait `ModelProvider` com suporte a `LocalLlamaProvider` (offline via `llama.cpp` e GBNF) e `CloudApiProvider` (OpenAI, Anthropic, OpenRouter, Together AI com streaming SSE e structured output). Embeddings locais com `fastembed` (384d).

2. **Diretrizes de Rede e Extração Web**:
   - Fast Path: Cliente HTTP Rust nativo (`rquest`) com emulação TLS BoringSSL e assinaturas JA3/JA4.
   - Stealth Path: Instalação lazy sob demanda de Chromium headless isolado (`chromiumoxide`) em `.system/bin/`, acionado apenas mediante bloqueios intransponíveis.
   - Conexões de agentes autônomos locais DEVEM ter conexões de rede de saída bloqueadas por padrão (`deny network-outbound`) durante a análise de documentos e geração de sínteses.

3. **Estrutura Canônica do Vault**:
   - `.system/`: Índices SQLite (`index.db`, `audit_log.db`) e caches efêmeros (descartáveis).
   - `intentions/`: Arquivo de teleologia e objetivos (`intentions.json`).
   - `assets/`: Armazenamento imutável por hash SHA-256.
   - `ingest/`: Diretório somente-leitura append-only segregado em `web/`, `media/` e `notes/`.
   - `workspaces/`: Sandboxes individuais contendo `topology.json`, `board.canvas.mpk`, `board.canvas.json`, peças (`pieces/`) e rascunhos voláteis (`scratchpad/`).

## Development Workflow & Quality Gates

1. **Test-First e Verificação de Invariantes**:
   - Todas as operações no sistema de arquivos DEVEM possuir testes unitários e de integração validando o confinamento no sandbox e a rejeição de symlinks maliciosos.
   - Modificações em schemas de banco ou estruturas de serialização (MessagePack / JSON) DEVEM incluir testes de migração e validação de compatibilidade retroativa.

2. **Quality Gates de Desempenho e Memória**:
   - Nenhum pull request ou release DEVE ser aprovado caso os testes automatizados indiquem violação dos limites de RAM em repouso ($\le 350\text{ MB}$) ou tempo de boot ($\le 2.2\text{ s}$ em NVMe).
   - Benchmarks de renderização gráfica DEVEM atestar 60 FPS estáveis na movimentação da câmera com mais de 1000 nós no viewport sob os diferentes níveis de LOD.

3. **Auditoria de Código e Segurança**:
   - Toda integração de dependência externa ou crate Rust DEVE passar por auditoria de segurança (`cargo audit`), sendo expressamente vedado o uso de bibliotecas não mantidas ou com vulnerabilidades conhecidas de I/O e execução remota.

## Governance

Esta constituição estabelece as diretrizes arquiteturais e operacionais soberanas do projeto Sandland e tem precedência sobre qualquer decisão pontual, solicitação de funcionalidade ou código ad-hoc. 

1. **Procedimento de Emenda**:
   - Qualquer modificação dos princípios inegociáveis ou das restrições fundamentais DEVE ser formalizada através de atualização deste documento.
   - Propostas de emenda DEVEM apresentar justificativa técnica documentada, análise de impacto em desempenho e segurança, e plano de migração para cofres (*vaults*) legados.
   - A aprovação de emendas exige revisão de arquitetura e conformidade integral com os testes automatizados de segurança e desempenho.

2. **Política de Versionamento Semântico**:
   - **MAJOR**: Remoção, afrouxamento ou redefinição incompatível de princípios de segurança, local-first, integridade ou sandboxing.
   - **MINOR**: Adição de novos princípios, novos componentes de subsistema, ou refinamento e expansão de restrições técnicas.
   - **PATCH**: Correções redacionais, esclarecimentos conceituais, correções de sintaxe ou alinhamentos sem impacto comportamental.

3. **Revisão de Conformidade**:
   - Todas as especificações técnicas (`spec.md`), planos de implementação (`plan.md`) e tarefas (`tasks.md`) gerados pelo Spec Kit DEVEM ser explicitamente verificados contra os princípios e restrições desta constituição.

**Version**: 1.0.1 | **Ratified**: 2026-09-21 | **Last Amended**: 2026-09-21
