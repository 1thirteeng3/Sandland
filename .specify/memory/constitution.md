<!--
Sync Impact Report:
- Version change: 1.0.1 -> 2.0.0
- List of modified principles:
  - I. Soberania do Dado (File-as-Truth & Local-First) -> I. Soberania do Dado e Precedência Canônica (File-as-Truth & Local-First)
  - II. Separação Cognitiva e Modularidade Editorial (Fork-on-Insert) -> II. Separação Cognitiva, Edição Modular e Fork-on-Insert Estendido
  - III. Contenção Rigorosa, Sandboxing OS-Level, I/O Seguro e Portabilidade (NON-NEGOTIABLE) -> III. Contenção Rigorosa, Sandboxing por Perfil e I/O Anti-TOCTOU (NON-NEGOTIABLE)
  - IV. Desempenho Nativo e Eficiência Estrita de Recursos -> IV. Núcleo Nativo, Extensões Locais Controladas e Eficiência de Recursos
  - V. Arquitetura RLM, Vectorless RAG para a Mesa e Integridade de IA -> V. Arquitetura de Contexto Vectorless para a Mesa e Busca Híbrida Desacoplada
  - VI. Resiliência Operacional, Auditoria e Versionamento Duplo -> VI. Decisões Tipadas de Taxonomia (Jev-First & BYOK) e Integridade de IA
  - Added: VII. Resiliência Operacional, Journaling Atômico e Recuperação contra Quedas
- Added sections:
  - Classes de Dados e Precedência de Persistência (em Technical Constraints & Security Standards)
  - Licenciamento Open Source e Governança de Terceiros (em Technical Constraints & Security Standards)
  - Modelo de Execução Solo com Agentes de IA (em Development Workflow & Quality Gates)
- Removed sections: None
- Follow-up TODOs: None
-->

# Sandland Constitution

## Core Principles

### I. Soberania do Dado e Precedência Canônica (File-as-Truth & Local-First)
O sistema de arquivos local é a única e definitiva fonte da verdade (*single source of truth*). O dado do usuário pertence exclusivamente a ele e DEVE ser persistido em formatos abertos e legíveis sem mediação de serviços externos.
- **Classes de Dados**: O sistema DEVE segregar estritamente seus dados em quatro classes:
  1. *Canônica* (impossível de apagar sem perda autoral irreversível): células e peças em Markdown (`.md`) com frontmatter YAML, topologia declarativa da Mesa em `board.canvas.json`, intenções em `intentions.json`, taxonomia aprovada em `taxonomy/terms.json`, e binários/mídias preservados no Content-Addressable Storage (`assets/<hash>.<ext>`) identificados por hash SHA-256.
  2. *Histórico Canônico*: registro contínuo e imutável de eventos e snapshots em `.history/events/*.jsonl` e `.history/objects/<hash>`.
  3. *Derivada* (descartável e reconstruível a frio a partir da fonte canônica): índices relacionais e FTS em SQLite (`index.db`), vetores e grafos derivados (Qdrant ou HelixDB), mapas de navegação do Context Engine, previews renderizados e caches binários de alta frequência (`board.canvas.mpk`).
  4. *Efêmera*: rascunhos voláteis de scratchpad, temporários de conversão e logs não autorais.
- O Sandland DEVE operar 100% autônomo e offline; nenhuma funcionalidade essencial de leitura, edição, navegação ou organização pode depender de conectividade de rede ou nuvem obrigatória.

### II. Separação Cognitiva, Edição Modular e Fork-on-Insert Estendido
As fases do fluxo de conhecimento (Ingestão, Exploração Espacial na Mesa e Produção Editorial na Peça) DEVEM manter fronteiras funcionais e estados desacoplados.
- **Ingestão Passiva e Não Bloqueante**: O módulo de Ingest opera em modo somente-leitura e append-only; o arquivo original capturado é gravado intacto no CAS antes de qualquer processamento ou OCR, garantindo que enriquecimentos falhos nunca corrompam a fonte primária.
- **Fronteira Gráfica e Editorial da Mesa**: A cena espacial da Mesa é gerida por PixiJS (câmera, seleção, conexões, frustum culling e LOD). O motor editorial BlockSuite DEVE ser instanciado sob demanda via overlay DOM sincronizado com a viewport estritamente na(s) célula(s) em edição ativa (máximo de 1 a 2 instâncias ativas concorrentes). Nós passivos utilizam previews estáticos, sendo proibido instanciar centenas de editores Web Components simultâneos.
- **Mecânica de Fork-on-Insert Estendido**: Ao transferir qualquer nó ou trecho da Mesa para o documento da Peça, o conteúdo DEVE ser clonado de forma totalmente independente. Metadados canônicos de citação DEVEM ser gravados no frontmatter da Peça (`source_cell_id`, `source_revision`, `source_block_id`/seletor, `source_asset_hash`, `quote`, `quote_hash`, `piece_block_id` e `inserted_at`). Edições subsequentes na célula do canvas NUNCA modificam a Peça retroativamente sem confirmação explícita do usuário.

### III. Contenção Rigorosa, Sandboxing por Perfil e I/O Anti-TOCTOU (NON-NEGOTIABLE)
Toda execução automatizada, worker poliglota, subagente de IA e rotina de extração web DEVE operar sob jaulas de segurança nativas do sistema operacional com privilégio mínimo e isolamento estrito de processos.
- **Sandboxing por Plataforma**: Utilizar Landlock LSM e seccomp no Linux; Job Objects restritos, tokens com privilégios reduzidos e AppContainer no Windows; helpers isolados e App Sandbox no macOS. As quotas de recursos e limites de processos (`ActiveProcessLimit`) DEVEM ser parametrizados por papel funcional (browser multiprocesso, workers de conversão, runtime de inferência).
- **Eliminação Real de Vulnerabilidades TOCTOU**: É expressamente proibida a sequência ingênua de verificação `canonicalize -> starts_with -> open`. O I/O DEVE operar por descritores/handles com resolução restrita ao workspace (ex.: `openat2` com `RESOLVE_BENEATH` no Linux, reparse-point safe handles no Windows, e validação no `ToolBroker`/`VaultGuard`). Tentativas de escape de limites disparam rejeição sumária (`SecurityError::SandboxEscapeAttempt`).
- **Proibição de Caminhos Hardcoded**: É vedada a presença de qualquer caminho de arquivo absoluto hardcoded no código-fonte ou scripts de build; toda resolução de cofre e pastas temporárias DEVE ser dinâmica via APIs de sistema do Tauri v2 (`app.path()`).
- **Contenção de Rede e SSRF**: Workers de extração e parsing operam com saída de rede bloqueada por padrão (`deny network-outbound`). Provedores de busca web (SearxNG e Scrapling) DEVEM aplicar sanitização rigorosa de requisições e proteção ativa contra SSRF (bloqueio de localhost, redes privadas e serviços internos do Sandland). LLMs e agentes NUNCA têm permissão para executar comandos arbitrários de shell ou acessar credenciais do sistema.

### IV. Núcleo Nativo, Extensões Locais Controladas e Eficiência de Recursos
A arquitetura do Sandland adota o princípio de **Núcleo Nativo com Extensões Gerenciadas**:
- **Núcleo de Autoridade em Rust**: A autoridade de dados, o armazenamento canônico, o controle de acesso, o agendamento de tarefas e a interface Tauri v2 são estritamente implementados em Rust nativo compilado.
- **Workers Especializados Gerenciados**: Motores especializados externos (Docling para documentos e OCR, Scrapling para aquisição web, SearxNG para metabusca, PageIndex adaptado para contexto, Readability/Turndown para sanitização HTML e LocalAI para execução de inferência) operam como processos locais satélites, iniciados sob demanda, monitorados pelo `ResourceSupervisor`, com ciclo de vida estrito e desligamento imediato quando ociosos.
- **Experiência Zero-Config**: O usuário comum NÃO PODE ser forçado a instalar interpretadores externos (Python, Go, Docker); todos os executáveis e runtimes satélites são empacotados ou baixados de forma transparente, isolada e verificada por hash criptográfico seguro.
- **Metas Rígidas de Desempenho e Recursos**:
  - Consumo de RAM em repouso (*idle*) com até 100 nós no canvas NÃO PODE exceder 350 MB (somatório de Core, WebView, GPU e serviços gerenciados).
  - O tempo de inicialização a frio até a prontidão interativa da interface DEVE ser inferior a 2.2 segundos em discos NVMe.
  - A cena gráfica (PixiJS) DEVE sustentar 60 FPS com 1.000 nós no viewport com Frustum Culling e LOD ativados.
  - Modelos locais de inferência no LocalAI DEVEM ser sumariamente descarregados da VRAM/RAM após 5 minutos de ociosidade contínua.

### V. Arquitetura de Contexto Vectorless para a Mesa e Busca Híbrida Desacoplada
A cognição e recuperação de dados no Sandland são adaptadas à natureza estrutural de cada espaço:
- **Vectorless Context Engine para a Mesa**: A Mesa de Trabalho adota o paradigma de contexto estrutural (adaptação do PageIndex sobre o `WorkspaceSnapshot` tipado com grupos, células, blocos, arestas e resumos hierárquicos). Células e conexões NÃO PODEM ser fragmentadas em chunks cegos ou vetores densos descontextualizados; a exploração analítica ocorre por ferramentas tipadas (`get_workspace_overview`, `get_group`, `read_cell`, `read_relations`, `read_source`) sob controle rígido de tokens pelo `ToolBroker`.
- **Invalidação Incremental e Econômica**: Mutações puramente geométricas (mover nós) invalidam apenas a topologia; alterações textuais invalidam apenas o resumo da célula afetada e evidências vinculadas; alterações de conexões invalidam apenas mapas relacionais. É vedado recalcular resumos globais por mudanças pontuais de layout.
- **Busca Híbrida Desacoplada para o Acervo**: Documentos brutos do Ingest utilizam o contrato genérico `SearchProvider`, implementando avaliação comparativa entre Qdrant (vetorial/híbrido) e HelixDB (grafo/vetores embutido), combinados com busca lexical FTS5 via Reciprocal Rank Fusion (RRF).
- **Integridade Epistêmica**: Sínteses e peças produzidas por IA DEVEM ser indexadas com marcação explícita de proveniência (`generated_or_authored = generated`), sendo expressamente vedado utilizá-las como evidências externas independentes para validar novos raciocínios.

### VI. Decisões Tipadas de Taxonomia (Jev-First & BYOK) e Integridade de IA
A organização do conhecimento substitui palpites generativos soltos por decisões tipadas e auditáveis:
- **Contrato de Decisão Estruturada (`DecisionProvider`)**: Classificações taxonômicas utilizam primitivas determinísticas (Choice para seleção entre categorias prévias, Score para relevância e Noul para atribuição de tags), inspiradas na semântica do Jev, em vez de exigir que um LLM produza categorias, tags e resumos em texto livre no mesmo prompt.
- **Jev-First com Soberania Local**: Jev atua como provedor preferencial para decisões complexas quando expressamente autorizado via rede/API. Na ausência de credenciais, rede ou preferência por privacidade estrita, o sistema DEVE operar com taxonomia pendente ou fallback determinístico local explicitamente sinalizado, sendo expressamente proibido qualquer fallback silencioso para nuvem.
- **Rigor Taxonômico**: É terminantemente vedada a fusão silenciosa de tags ou termos por distância de Levenshtein ($\le 2$) ou similaridade vetorial sem revisão humana explícita. Termos possuem IDs estáveis independentes de formatação textual.
- **Isolamento de Inferência Generativa**: A camada `ModelGateway` abstrai provedores locais (LocalAI / `llama.cpp`) e remotos (BYOK - Bring Your Own Key). A validação de payloads estruturados DEVE utilizar GBNF ou JSON Schema no nível do motor, com sanitização semântica complementar pelo Sandland Core.

### VII. Resiliência Operacional, Journaling Atômico e Recuperação contra Quedas
O Sandland DEVE assegurar imunidade contra perda de dados decorrente de travamentos da aplicação ou interrupção repentina de energia.
- **Topologia Declarativa Soberana**: O arquivo `board.canvas.json` é a representação canônica da topologia do workspace. O arquivo `board.canvas.mpk` atua exclusivamente como cache binário de alta performance indexado pela revisão do JSON. Em caso de divergência de timestamps, a integridade canônica do JSON e do journal de eventos DEVE ter precedência irrestrita sobre o binário.
- **Protocolo de Persistência em Duas Fases**: Toda mutação autoral DEVE ser registrada de forma atômica no journal de eventos (`.history/events/`) antes da publicação do arquivo no disco via substituição atômica de arquivos no SO (`FlushFileBuffers` / `fsync` seguido de rename atômico).
- **Garantia de Queda de 500 ms**: A arquitetura DEVE garantir que nenhuma falha abrupta provoque perda de digitação superior a 500 milissegundos. No boot, o `VaultStore` executa rotina de recuperação idempotente, auditando a coerência entre o journal durável e os arquivos canônicos.

## Technical Constraints & Security Standards

1. **Stack Tecnológica Mandatória**:
   - Backend & Autoridade: Rust (edição estável mais recente), Tauri v2.
   - Frontend & Apresentação: Webview moderna com renderização acelerada por hardware via WebGL / PixiJS para a cena espacial da Mesa, e BlockSuite com overlay DOM para edição de texto estruturado.
   - Armazenamento Canônico: Sistema de arquivos local com Markdown, frontmatter YAML, JSON canônico e CAS SHA-256.
   - Motores de Recuperação & Índices: SQLite embarcado (FTS5 com `unicode61`), adaptador `SearchProvider` integrando Qdrant ou HelixDB para busca híbrida com fusão RRF ($k = 60$).
   - Workers Locais & Runtimes: Docling (parsing/OCR), Scrapling (extração web), SearxNG (metabusca JSON local), LocalAI (gestão de modelos locais), Readability/Turndown (normalização HTML em sandbox).
   - Abstração de IA & Decisão: Trait `ModelProvider` e gateway de inferência agnóstico (LocalAI / BYOK Cloud); trait `DecisionProvider` com suporte a Jev (API TypeSafe autorizada) e classificadores locais.

2. **Protocolo de Persistência, Transações e Classes de Dados**:
   - As gravações em disco DEVEM obedecer a uma ordem estrita: validar revisão esperada $\rightarrow$ preparar objeto temporário $\rightarrow$ registrar entrada durável no journal $\rightarrow$ aplicar substituição atômica via rename no SO $\rightarrow$ emitir evento assíncrono para os indexadores.
   - Nenhuma thread de renderização de interface gráfica pode executar I/O de disco bloqueante ou síncrono.
   - O SQLite atua unicamente como projeção acelerada de inventário e índices descartáveis; sua exclusão física acidental ou intencional nunca acarreta perda de notas, topologias ou intenções.

3. **Diretrizes de Segurança, Confinamento e Rede**:
   - **VaultGuard & ToolBroker**: Toda solicitação de leitura ou escrita por subagentes e ferramentas DEVE passar por mediação central com verificação de escopo, permissão e resolução de handles livres de TOCTOU.
   - **Políticas de Egress**: Bloqueio de rede padrão (`deny network-outbound`) para todos os módulos analíticos e de inferência. Requisições web do SearxNG e Scrapling DEVEM respeitar lista restrita de protocolos, timeouts rigorosos, limites de tamanho de payload e bloqueio integral de redes privadas (RFC 1918 / loopback / metadados cloud).
   - **Proteção de Segredos**: Chaves de API do usuário (BYOK, Jev) DEVEM ser armazenadas no chaveiro seguro nativo do sistema operacional (OS Keyring / Secret Service), sendo estritamente vedada sua persistência em arquivos Markdown, arquivos de configuração no Vault ou repositórios Git.

4. **Estrutura Canônica do Vault**:
   ```text
   vault/
   ├── vault.json                         # Identidade e versão do schema do Vault
   ├── assets/<hash>.<ext>                 # Arquivos binários imutáveis (CAS SHA-256)
   ├── ingest/                            # Fontes brutas preservadas (web, media, notes)
   ├── taxonomy/terms.json                # Vocabulário controlado, aliases e decisões
   ├── intentions/intentions.json         # Intenções e objetivos de pesquisa
   ├── workspaces/<id>/                   # Sandboxes isolados de trabalho
   │   ├── board.canvas.json              # Topologia canônica da Mesa
   │   ├── cells/*.md                     # Células atômicas promovidas
   │   ├── pieces/*.md                    # Peças editoriais produzidas
   │   └── scratchpad/                    # Rascunhos voláteis (não promovidos)
   ├── .history/                          # Histórico canônico aberto
   │   ├── events/*.jsonl                 # Log contínuo e durável de operações
   │   └── objects/<hash>                 # Snapshots e revisões versionadas
   └── .system/cache/                     # Índices e caches descartáveis (ignorado em sync)
       ├── index.db                       # Projeções relacionais e FTS
       ├── search/                        # Índices vetoriais/grafos (Qdrant/HelixDB)
       ├── context/                       # Caches do Context Engine
       └── boards/*.mpk                   # Caches binários de aceleração
   ```

5. **Licenciamento Open Source e Governança de Terceiros**:
   - O Sandland é um projeto de código aberto desenvolvido sem finalidade comercial. O código próprio do núcleo adota licença recíproca (proposta: AGPL-3.0-or-later).
   - O projeto DEVE respeitar integralmente as licenças dos motores incorporados ou integrados (MIT, Apache-2.0, BSD-3-Clause, MPL-2.0, AGPL-3.0).
   - Cada release DEVE manter atualizados o arquivo `THIRD_PARTY_NOTICES`, a pasta `LICENSES/` e um manifesto SBOM detalhando componentes, hashes e licenças de todos os runtimes e modelos distribuídos.

## Development Workflow & Quality Gates

1. **Test-First, Confinamento e Validação de Invariantes**:
   - Todo subsistema de persistência e acesso ao sistema de arquivos DEVE possuir suíte de testes unitários e de integração que comprove a contenção em sandbox, rejeição de symlinks maliciosos, controle de hard links e resistência a ataques TOCTOU.
   - Qualquer modificação de esquemas estruturais (`board.canvas.json`, frontmatter, schemas de eventos) DEVE conter testes de compatibilidade retroativa e migração idempotente.

2. **Quality Gates de Desempenho e Memória**:
   - Nenhum pull request ou release DEVE ser aprovado caso benchmarks automatizados indiquem violação do teto de memória em repouso ($\le 350\text{ MB}$), tempo de inicialização interativa ($\le 2.2\text{ s}$ em NVMe) ou taxa de quadros da cena gráfica ($< 60\text{ FPS}$ com 1.000 nós no canvas).
   - Modelos locais DEVEM ser auditados para comprovar o descarregamento efetivo de memória da GPU e da RAM após 5 minutos de ociosidade contínua.

3. **Modelo de Execução Solo com Agentes de IA**:
   - O desenvolvimento do Sandland adota a governança de uma pessoa responsável (autor/mantenedor) com auxílio de agentes autônomos operando sob papéis estritos:
     - *Mantenedor Humano*: Autoridade soberana final sobre produto, decisões arquiteturais (ADRs), segurança, licenças e aprovação de merges.
     - *Agente de Implementação*: Escopo restrito a uma única tarefa delimitada por sprint (conforme definido no Roadmap e Backlog), em branch isolada, sem autorização para alterar esquemas ou contratos globais sem aprovação prévia.
     - *Agente de Testes*: Elaboração de fixtures, testes negativos, cenários adversariais e validação de regressões.
     - *Agente Revisor*: Leitura adversarial de diffs, validação de conformidade com esta Constituição e checklist de critérios de aceite do sprint.
     - *CI/Harness*: Execução automatizada de testes, linters, `cargo audit`, verificação de licenças e benchmarks de performance.
   - Limite de Trabalho em Progresso (WIP): No máximo uma tarefa principal de implementação ativa por vez, garantindo rastreabilidade e integridade conceitual.

4. **Auditoria de Código e Dependências**:
   - Toda biblioteca Rust ou dependência externa DEVE ser submetida a varredura contra vulnerabilidades conhecidas (`cargo audit`).
   - É expressamente vedado o uso de bibliotecas abandonadas ou que apresentem riscos não auditados de execução arbitrária de código ou vazamento de descritores de arquivo.

## Governance

Esta constituição estabelece as diretrizes arquiteturais, operacionais e éticas soberanas do projeto Sandland e tem precedência sobre qualquer decisão pontual, solicitação de funcionalidade ou código ad-hoc.

1. **Procedimento de Emenda e Registro de ADRs**:
   - Qualquer modificação dos princípios inegociáveis ou das restrições técnicas DEVE ser formalizada através de Architectural Decision Record (ADR) numerada e documentada.
   - Propostas de emenda DEVEM apresentar justificativa técnica fundamentada, análise de impacto em segurança e desempenho, avaliação de compatibilidade de formatos e plano de migração para cofres existentes.
   - A ratificação de qualquer emenda requer aprovação explícita do mantenedor humano do projeto e validação pelos testes automatizados.

2. **Política de Versionamento Semântico**:
   - **MAJOR**: Remoção, afrouxamento ou redefinição incompatível de princípios de segurança, soberania do dado, arquitetura de persistência, modelo de execução ou sandboxing.
   - **MINOR**: Adição de novos princípios, novos componentes de subsistema, ou refinamento e expansão substancial de diretrizes técnicas e restrições.
   - **PATCH**: Correções redacionais, esclarecimentos conceituais, ajustes de tipografia ou correções de sintaxe sem impacto normativo.

3. **Revisão de Conformidade no Spec Kit**:
   - Todas as especificações técnicas (`spec.md`), planos de implementação (`plan.md`) e tarefas (`tasks.md`) gerados no fluxo de desenvolvimento com Spec Kit DEVEM ser explicitamente confrontados e validados contra os princípios e restrições desta constituição.
   - Qualquer proposta em especificação ou plano que viole ou contorne as regras aqui dispostas DEVE ser rejeitada ou adaptada antes do início da implementação.

**Version**: 2.0.0 | **Ratified**: 2026-09-21 | **Last Amended**: 2026-09-22
