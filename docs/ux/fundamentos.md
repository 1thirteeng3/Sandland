# Fundamentos de UX — Sandland

**Fase:** 1 de 5 (Fundamentos) · **Status:** Aprovado para derivação da Fase 2 (IA)
**Data:** 2026-09-22 · **Arquivo:** `docs/ux/fundamentos.md`
**Precedência:** Este documento interpreta a Constituição e o PRD para fins de interface.
Em conflito com código existente, vale a Constituição; em conflito com este documento,
vale decisão explícita posterior do responsável pelo produto, registrada como emenda aqui.

---

## 0. Decisões vinculantes já tomadas (não reabrir sem emenda)

| # | Decisão | Origem |
|---|---------|--------|
| D-01 | O produto tem **3 ecrãs de topo**: **Captura**, **Mesa**, **Chat**. O shell nunca muda de número | Produto, 2026-09-22 |
| D-02 | **Captura é sempre separada e não expõe IA como recurso interativo** | Produto, 2026-09-22 |
| D-03 | **Mesa acessa IA via command-palette first** (⌘K / Ctrl+K) | Produto, 2026-09-22 |
| D-04 | **Chat é conversa normal com a base de conhecimento, com IA** | Produto, 2026-09-22 |
| D-05 | **Peça é um nó/widget que abre e se expande dentro da Mesa** (não é ecrã de topo) | Produto, 2026-09-22 |
| D-06 | **Descoberta vive num widget de busca externa na Mesa**: resultado é plotado no board; com um gesto, o item pode ser enviado à Captura | Produto, 2026-09-22 |
| D-07 | **Governança/Auditoria vive dentro de Configurações**, acessível de qualquer ecrã | Produto, 2026-09-22 |
| D-08 | **Só Dark.** Não existe tema claro; "light" não será implementado | Produto, 2026-09-22 |
| D-09 | Nesta rodada: **só spec + tokens**. Nenhum componente Svelte novo será implementado | Produto, 2026-09-22 |

> Leitura crítica de D-02 (registrada aqui para não gerar ambiguidade na Fase 2):
> a Captura **opera um pipeline de IA invisível** — auto-tagging local via SLM + GBNF
> (SPEC FR-003/FR-004, tasks T025–T033, todas concluídas). D-02 proíbe **IA como recurso
> interativo** na Captura (sem caixa de prompt, sem seletor de modelo, sem chat, sem botões
> "gerar"). Classificação aparece como **estado de pipeline** (badges de estado), nunca como
> "a IA". O comando IPC `trigger_classification` existe, mas deve ser exposto como ação de
> sistema ("Reprocessar"), não como affordance de IA. Ver P2 e §4.

---

## 1. Fontes e método

### 1.1. Fontes canônicas consultadas (todas, sem exceção)

| Fonte | Versão/Data | Papel neste documento |
|-------|-------------|----------------------|
| Constituição (`/.specify/memory/constitution.md`) | v1.0.1, ratificada 2026-09-21 | 6 princípios I–VI + estrutura do vault + gates. Precedência máxima |
| PRD (`Documento de Arquitetura Técnica e Especificação de Produto.md`) | 805 linhas | Filosofia, pipeline taxonômico (§3), busca híbrida (§4), whiteboard (§5), extração web (§6), Peça + fork-on-insert (§7), RLM (§8), Intenções + Chat (§9), modelos (§10), versionamento (§11), roadmap v0.1/v0.2/v1.0 (§12), aceites (§13) |
| `docs/architecture.md` | v0.2.0-draft | Diagrama de 2 painéis (Acervo-gaveta + Mesa) — **declarado defasado para navegação** (ver §9, C-01) |
| `docs/storage.md` | Aprovado | Layout do vault, dualidade de motores, ciclo de vida de células, `topology.json` |
| `docs/ai-pipeline.md` | Aprovado | Trait `ModelProvider`, Skeleton Map, painel de BYOK no frontend (§4) |
| ADR-0001 (Vectorless RAG / RLM) | Aceito 2026-09-21 | Nada na Mesa é vetorizado; Skeleton Map 800–2000 tokens; `read_cell` sob demanda |
| ADR-0002 (`ModelProvider` + BYOK) | Aceito 2026-09-21 | Local vs nuvem por escolha do usuário; **exige UI** de teste de chave, cota e download de modelos (Consequências) |
| ADR-0003 (ciclo de vida de células) | Aceito 2026-09-21 | Célula vive em `topology.json`; promoção a nota é gesto explícito; células não promovidas **não aparecem na busca do Acervo** |
| ADR-0004 (portabilidade de paths) | Aceito 2026-09-21 | Sem impacto direto de UI, exceto: nunca exibir paths absolutos como identidade (ver P3) |
| `specs/001-local-first-canvas/spec.md` | Draft 2026-09-21 | 4 user stories (P1/P1/P2/P3), FR-001–FR-013, 5 entidades, SC-001–SC-006, edge cases, assumptions |
| `specs/001-local-first-canvas/plan.md` | Revisão pós-crítica | Metas de performance (60 FPS, boot <2.2s, idle ≤350MB, SLM unload 5min) |
| `specs/001-local-first-canvas/data-model.md` | Ready & Approved | Frontmatter canônico, state machine, DDL, topologia, Skeleton Map, anti-poluição |
| `specs/001-local-first-canvas/research.md` | Completed | B1–B4, canvas híbrido WebGL+DOM, fastembed desacoplado, anti-patterns, prompt GBNF |
| `specs/001-local-first-canvas/contracts/tauri-ipc.md` | Ready & Exhaustive | 12 comandos + 3 eventos + `SandlandError` (10 variantes) — base da §7 |
| `specs/001-local-first-canvas/tasks.md` | Ready (tudo `[X]`) | Prova de que v0.1 está implementada; T044 (minimap) **sem correspondente no código** — ver §8 G-12 |
| Código `src/` (20 arquivos lidos integralmente) | Estado em 2026-09-22 | Shell, tokens, 7 componentes, 2 stores — auditados na §8 |

### 1.2. Método

1. Extração dos invariantes de cada fonte (o que é **obrigação**, não preferência).
2. Derivação de princípios de UX por tradução 1:1 de cada invariante constitucional,
   no formato: título curto → enunciado → porquê → aplicação → contra-exemplo → trade-off,
   com **ordem de precedência explícita para conflitos** (skill `design-principles`).
3. Mapeamento funcional-emocional-social dos jobs (skill `jobs-to-be-done`), restrito ao
   que a spec e o PRD afirmam sobre o usuário — sem inventar persona.
4. Inventário fechado de superfícies e objetos: tudo que aparece em qualquer fonte, nada além.
5. Rastreabilidade FR/SC/IPC → superfície (tabela §7): cada requisito termina num ecrã.
6. Auditoria adversarial do código atual contra os princípios (§8) e registro de
   contradições inter-fontes + questões abertas numeradas (§9).

---

## 2. Personas e Jobs-to-be-Done

A spec nomeia o usuário como **"pesquisador ou escritor analítico"** (US-1),
**"usuário que acumula dezenas de fontes"** (US-2) e **"analista visual"** (US-3);
o PRD fala em **pesquisadores com exigência de confidencialidade estrita** (ADR-0002)
e profissionais que querem modelos de fronteira. Daí derivam 3 perfis — não são
arquétipos inventados, são os 3 modos de uso que as fontes exigem:

- **A Pesquisadora** — acumula fontes, precisa de soberania total (100% offline).
  Vive na Captura e no Chat com escopo Acervo.
- **O Escritor Analítico** — transformasei em tese; precisa de proveniência
  (citação célula→peça) e copiloto de tensionamento. Vive na Mesa (Peça expandida).
- **O Analista Visual** — pensa espacialmente; precisa de 60 FPS, LOD e arestas
  tipadas. Vive na Mesa (canvas + palette).

### 2.1. Job principal

> **Quando** acumulo material heterogêneo (notas, PDFs, web, ideias soltas),
> **quero** capturar sem atrito, organizar espacialmente e transformar em texto com
> proveniência — sem entregar meus dados a nenhuma nuvem —,
> **para** produzir análise original rastreável até a fonte.

- Funcional: ciclo Captura → Exploração → Conexão → Produção sem perda e sem retrabalho.
- Emocional: **controle soberano** (nada acontece aos meus dados sem meu gesto) e
  **continuidade de fluxo** (nunca interrompido por burocracia de catalogação — US-1).
- Social: autoria verificável (cada afirmação da Peça aponta para a célula-fonte).
- Sucesso: SC-001–SC-006 (ingestão <1s; tagging <5s; 80% reaproveitamento de tags;
  60 FPS/100 nós; 100% offline; crash-recovery <1s).

### 2.2. Jobs por fase (cada um ancora num ecrã — D-01)

| Fase (PRD) | Job | Ecrã dono | Exigência de UX (fonte) |
|---|---|---|---|
| Ingestão (§3) | Capturar sem classificar; revisar só o que falhou | Captura | Zero formulário obrigatório (US-1); fila de `NeedsManualReview` discreta (US-2, cenário 3) |
| Exploração (§5) | Ver tudo de uma vez; aproximar para ler (LOD) | Mesa | 3 níveis de detalhe por zoom (FR-012); pan/zoom sem engasgo (SC-004) |
| Conexão (§5/§8) | Declarar relações (apoia/refuta/evidência) que vetor nenhum infere | Mesa | Arestas direcionadas com âncora (US-3, cenário 2); posição e grupo são semântica (ADR-0001) |
| Descoberta (§6) | Trazer a web para dentro sem sair do raciocínio | Mesa (widget) | Fast Path invisível; Stealth Path só sob bloqueio, com download lazy e consentimento (PRD §6) |
| Produção (§7) | Escrever a Peça sem quebrar a origem; tensionar com copiloto | Mesa (Peça expandida) | Fork-on-insert com `source_cell_id + inserted_at + original_snippet` (CONST II) |
| Governança (§11) | Desfazer, auditar, versionar, recuperar de queda | Config + transversal | Perda <500ms; undo em lote; Git opcional (CONST VI) |
| Teleologia (§9) | Declarar intenções; ser avisado quando o acervo converge | Chat (config) + notificações | Threshold 0.85 de similaridade; sugestão de canvas, nunca ação automática |

---

## 3. Princípios de UX (8, priorizados)

Ordem de precedência em conflito: **P1 > P2 > … > P8**. Exemplo resolvido:
se "IA invocada" (P4) conflitar com "tudo se desfaz" (P8), P4 vence —
a ação de IA exige invocação explícita **e**, uma vez invocada, submete-se ao undo.

### P1 — Soberania antes da magia
**Enunciado:** Nada sai da máquina sem um gesto explícito do usuário; o estado
local/nuvem de cada operação é sempre visível.
**Porquê:** CONST I (offline total, File-as-Truth) + CONST V/ADR-0002 (BYOK:
nuvem só com chave do usuário).
**Aplicação:** badge persistente de provedor ativo (Local/Nuvem) na Mesa e no Chat;
classificação da Captura é **sempre local** (SPEC FR-003) mesmo com nuvem configurada;
toda chamada cloud exige consentimento por ação ou sessão, com indicador de streaming.
**Contra-exemplo:** fallback automático silencioso para nuvem quando o SLM local falha.
**Trade-off:** mais atrito (configurar chave, esperar modelo local) em troca de confiança
absoluta. A fricção é documentada, nunca escondida.

### P2 — Cada fase no seu ecrã
**Enunciado:** Captura, Mesa e Chat nunca misturam affordances; a Captura não expõe
IA interativa (D-02).
**Porquê:** CONST II (separação cognitiva) + D-01–D-04.
**Aplicação:** sem prompt, sem seletor de modelo, sem "gerar" na Captura; IA da Mesa só
via palette (D-03); conversa só no Chat (D-04). A travessia entre ecrãs é sempre um
gesto de transporte (arrastar, promover, enviar), nunca um modo híbrido.
**Contra-exemplo:** botão "resumir com IA" dentro do cartão da Captura.
**Trade-off:** features de IA menos "descobríveis" na Captura; compensa-se com
visibilidade de estados (P3) e com a palette global na Mesa.

### P3 — Arquivo é verdade, índice é sombra
**Enunciado:** A UI apresenta arquivos como realidade e derivados como derivados;
estados de índice (hidratando, reindexando, degradado) são comunicados, nunca ocultos.
**Porquê:** CONST I + research §5 (frontmatter sem metadados extrínsecos; hash só no `index.db`).
**Aplicação:** itens da Captura exibem origem em disco e estado do pipeline; busca indica
quando o índice está frio; nunca exibir `vault_path` absoluto como identidade
(ADR-0004: paths são detalhe de FS, o `id` UUIDv7 é a âncora).
**Contra-exemplo:** mostrar snippet do FTS como se fosse o documento.
**Trade-off:** mais estados vazios/transitórios para desenhar (Fase 3); menos "mágica".

### P4 — IA invocada, jamais intrusiva
**Enunciado:** IA aparece em 3 modos e só: **pipeline invisível** (Captura),
**invocada por palette** (Mesa), **conversacional** (Chat). Nada gera, reescreve ou
reorganiza sem invocação — exceto o alerta quieto de Intenções (threshold 0.85, PRD §9),
que sugere e nunca executa.
**Porquê:** D-02–D-04 + PRD §9 (proposições proativas como notificação elegante).
**Aplicação:** palette da Mesa com ações tipadas (sintetizar board, ler célula,
promover, tensionar parágrafo); Chat com escopo explícito; Intenções como configuração
do Chat, não como agente autônomo visível.
**Contra-exemplo:** reescrita automática de tags ao abrir a Captura.
**Trade-off:** menos momentos "uau"; mais previsibilidade. Para este produto, previsibilidade vence.

### P5 — Promover é gesto, rascunho é ar
**Enunciado:** Tudo nasce volátil; virar arquivo exige gesto explícito
("Promover Célula a Nota", fork para a Peça); toda promoção carrega proveniência visível.
**Porquê:** ADR-0003 (anti-poluição) + CONST II (fork-on-insert com citação) + CONST VI
(células vivem em `topology.json`).
**Aplicação:** células não promovidas sinalizam "só existe neste board" (ADR-0003:
não aparecem na busca do Acervo); Peça exibe chips de proveniência
(`source_cell_id`, trecho original); onboarding ensina o modelo de dois níveis.
**Contra-exemplo:** criar `.md` por post-it automaticamente.
**Trade-off:** curva de aprendizado do modelo volátil-vs-arquivo; custo pago uma vez no
onboarding, economizado para sempre em cofre limpo.

### P6 — Espaço significa
**Enunciado:** Posição, agrupamento e aresta são semântica declarada pelo usuário;
toda representação (LOD, minimapa, Skeleton Map, exportação) deve preservá-los.
**Porquê:** ADR-0001 (vetorizar destrói o grafo cognitivo) + storage §3 (3 motivos:
posição, arestas, hierarquia).
**Aplicação:** sem auto-layout que reordene; sobreposição resolve-se com smart offset
(spec, edge case 4); zoom-out agrega sem reordenar; grupos têm título e cor persistidos.
**Contra-exemplo:** "organizar automaticamente" que re-fluxa o board.
**Trade-off:** abre-se mão de arrumação mágica; ganha-se confiança de que a Mesa
é o pensamento, não uma visualização descartável.

### P7 — Densidade segue o zoom
**Enunciado:** Todo surface entrega 3 densidades progressivas; detalhe total só no foco.
**Porquê:** FR-012 + US-4 (LOD >0.6 completo / 0.3–0.6 título+contorno / <0.3 proxy sólido).
**Aplicação:** generalizar o LOD para listas da Captura (linha → cartão → expandido) e
para o Chat (citação curta → trecho → fonte integral); thresholds numéricos do canvas
são lei (0.6 / 0.3), não sugestão.
**Contra-exemplo:** cartão sempre com corpo integral em qualquer zoom.
**Trade-off:** cada surface custa 3 desenhos (Fase 3); performance e legibilidade pagam a conta.

### P8 — Tudo se desfaz
**Enunciado:** Toda mutação — humana ou de IA — é reversível e anuncia sua
reversibilidade; quedas são narradas, não escondidas.
**Porquê:** CONST VI + PRD §11 (undo em lote, reversão de IA, recovery <500ms).
**Aplicação:** Ctrl+Z na Mesa e na Peça; ações de IA ((y) da palette, respostas do Chat
aplicadas) geram entradas de undo; pós-crash, aviso "recuperado até HH:MM:SS, perda <1s"
(SC-006); Config expõe auditoria (D-07).
**Contra-exemplo:** autosave silencioso sem histórico acessível.
**Trade-off:** pilha de undo e UI de histórico custam engenharia; sem eles, P4 é promessa vazia.

---

## 4. O modelo de 3 ecrãs (fixo)

```
┌──────────┐   gesto de transporte    ┌──────────┐   invocação     ┌──────────┐
│ CAPTURA  │ ── arrastar p/ a Mesa ──► │   MESA   │ ◄── palette ──► │   CHAT   │
│ sem IA   │ ◄── promover/enviar ──── │ +palette │                 │ com IA   │
│ interat. │      (volta como nota)   │ +widgets │   escopo:       │ escopo:  │
└──────────┘                          │ +Peça    │   board atual   │ Acervo / │
     ▲                                └──────────┘                 │ Mesa /   │
     │ Config (D-07: vault, modelos/BYOK, auditoria) — acessível de todos      │
     └─────────────────────────────────────────────────────────────────────────┘
```

### 4.1. O que cada ecrã é — e o que jamais será (escopo negativo)

**Captura.** É: porta de entrada passiva (drop, pasta vigiada, URL), lista com estados
de pipeline, fila de revisão, edição de tags/categoria (FR-007), envio à Mesa.
Jamais: prompt, modelo, geração, resumo automático visível como feature.
O resumo de 1 frase do SLM (SPEC US-2) aparece como **metadado do cartão**, rotulado
como estado ("classificado"), não como "resposta da IA".

**Mesa.** É: canvas infinito (pan/zoom/LOD), células/grupos/arestas, palette de
comando como **única** porta de IA, widgets (busca web, Peça), pílulas de workspaces
(roadmap v0.2, PRD §5). Jamais: conversa livre (isso é o Chat), catalogação
(isso é a Captura).

**Chat.** É: thread conversacional sobre a base, com **seletor de escopo**
(Acervo global / Mesa ativa / Intenção) — derivado da dualidade de motores (§6.1) e da
camada de Intenções (PRD §9). Citações clicáveis com proveniência; aplicar-no-board
como gesto explícito. Jamais: edição de arquivos, ingestão, configuração de modelos.

**Configurações.** É: vault (criar/abrir/trocar — IPC `create_vault`/`open_vault`),
modelos e BYOK (exigência explícita do ADR-0002: testar chave, ver cota, baixar GGUF
com progresso + SHA-256), auditoria/governança (D-07: trilha de eventos, Git opcional),
sobre. Jamais: conteúdo do usuário.

### 4.2. Modos de IA por ecrã (tabela normativa)

| Ecrã | Modo | Motor permitido | Nuvem (BYOK)? | Regra |
|------|------|-----------------|---------------|-------|
| Captura | Pipeline invisível | SLM local + GBNF + fallback léxico | **Nunca** (FR-003: 100% local) | Estados visíveis; "Reprocessar" é ação de sistema |
| Mesa (palette) | Invocada, escopo = board | Skeleton Map + `read_cell` (ADR-0001) | Sim, com consentimento e badge | Cada ação declara escopo e é desfazível (P8) |
| Mesa (widgets) | Embutida | Extração web fast/stealth (PRD §6) | N/A (web pública, não dados do vault) | Stealth/Chromium só sob bloqueio + download lazy + aviso |
| Chat | Conversacional, escopo selecionável | Híbrido Acervo (RRF k=60) e/ou Skeleton da Mesa | Sim, com consentimento e badge | Citação obrigatória; aplicar exige gesto |
| Config | Nenhuma (gestão) | — | Cadastro de chaves, teste de conectividade | Chaves só em `.system/api_keys.json` ou keyring do SO (ai-pipeline §4) |

### 4.3. Regras de travessia (gestos entre ecrãs)

1. Captura → Mesa: arrastar cartão (cria nó **vinculado** via `item_id` — ver §8 G-03).
2. Mesa → Captura: "Promover Célula a Nota" (cria `ingest/notes/<slug>-<uuid>.md`, ADR-0003)
   e "Enviar resultado web à Captura" (D-06).
3. Mesa ⇄ Peça: arrastar célula para a Peça = fork com citação (CONST II); Peça exibe
   chips de proveniência clicáveis que focalizam a célula-fonte.
4. Qualquer ecrã → Chat: "Perguntar sobre isto" leva contexto + escopo pré-selecionado.
5. Chat → Mesa: "Aplicar no board" plota resposta/citações como nós (gesto explícito).
6. Qualquer ecrã → Config: atalho único e persistente (engrenagem); auditoria mora lá (D-07).

---

## 5. Inventário de superfícies

Legenda de tipo: **E** = ecrã · **O** = overlay · **W** = widget de board · **T** = transitória.
Fase: `v0.1` (construir agora), `v0.2` (Peça/copiloto/pílulas), `v1.0` (web plena/RLM/intenções).

### 5.1. Ecrãs e transitórias

| ID | Superfície | Tipo | Fase | IA | Objetos primários |
|----|-----------|------|------|----|-------------------|
| S-00 | Onboarding / Primeiro boot | T | v0.1 | Nenhuma | Vault, Modelo (opt-in) |
| S-01 | Captura | E | v0.1 | Pipeline invisível | Item, Termo, Fila de revisão |
| S-02 | Mesa | E | v0.1 | Palette | Workspace, Célula, Aresta, Grupo |
| S-03 | Chat | E | v1.0* | Conversacional | Thread, Citação, Intenção |
| S-04 | Configurações | E | v0.1† | Nenhuma | Vault, Provedor/Chave, Auditoria |

\* O Chat ancora na camada de Intenções (roadmap v1.0, PRD §9/§12); nada impede um
Chat v0.2 com escopo Acervo sobre o RRF já implementado (T046). Decisão de
antecipação fica para a matriz da Fase 2.
† Config mínima v0.1 (vault + modelos, exigidos por T028.1/B3); auditoria plena com o roadmap.

### 5.2. Overlays e widgets da Mesa

| ID | Elemento | Tipo | Fase | Notas normativas |
|----|----------|------|------|------------------|
| M-01 | Command palette (⌘K/Ctrl+K) | O | v0.2‡ | **Única** porta de IA da Mesa (D-03); ações tipadas com escopo declarado |
| M-02 | Editor de célula (overlay DOM) | O | v0.1 | Duplo clique; Esc **descarta** (corrigir §8 G-06); Ctrl+Enter salva |
| M-03 | Controles de câmera | W | v0.1 | Zoom/pan/reset + presets + minimapa (T044 prevê; **não implementado** — §8 G-12) |
| M-04 | Pílulas de workspaces | W | v0.2 | Barra superior do PRD §5; trocar `workspaceId` sem perder viewport |
| M-05 | Widget de busca web | W | v1.0§ | Query/URL → snapshot → plotar → [enviar à Captura] (D-06) |
| M-06 | Peça (colapsada → expandida) | W | v0.2 | Nó especial; expande **dentro** da Mesa (D-05); editor de blocos + canal do copiloto (PRD §7) |
| M-07 | Menus de contexto (nó/aresta/fundo) | O | v0.1 | Casa de "Promover", "Duplicar", "Fixar", "Excluir" — hoje inexistentes (§8 G-09) |
| M-08 | Rascunho rápido (scratchpad) | O/W | a definir | `scratchpad/` existe no vault (storage §1) sem surface — ver §9 Q-04 |

‡ Palette pode nascer v0.1 como paleta **sem IA** (navegação/ações) e ganhar IA na v0.2 —
compatível com D-03 e útil desde já. Recomendação registrada para a Fase 2.
§ `ingest_url` v0.1 (simplificado) permite um M-05 mínimo (URL→snapshot→plotar) antes da
SERP plena (SearXNG/DDG, v1.0).

### 5.3. Sistemas transversais (todos os ecrãs)

| ID | Sistema | Exigência (fonte) |
|----|---------|-------------------|
| X-01 | Notificações (toasts + central) | download de modelos (evento `model://download-progress`), alertas de Intenção (threshold 0.85), falhas de classificação, crash-recovery (SC-006) |
| X-02 | Busca com escopo | Captura: híbrida RRF sobre o Acervo; Mesa: palette (nós do board + ações); Chat: escopo selecionável. Células não promovidas **só** são acháveis no board (ADR-0003) |
| X-03 | Undo / histórico | Ctrl+Z Mesa + Peça; reversão de IA (PRD §11); trilha em `audit_log.db` (debounced, nunca a 60 FPS) |
| X-04 | Troca de vault + onboarding | `create_vault`/`open_vault` exigem fluxo de primeiro boot (S-00): criar → opt-in de modelos → estados vazios guiados |
| X-05 | Estados vazios/erros | Cada surface tem vazio guiado; erros do `SandlandError` (10 variantes) viram mensagens PT amigáveis, nunca dumps |
| X-06 | Seleção com destino | Toda seleção abre um destino (detalhe/ação); seleção sem destino é bug de IA (ver §8 G-04) |

---

## 6. Inventário de objetos (linguagem ubíqua da interface)

Nome canônico em PT é lei de UI: o usuário nunca vê `topology.json`, `vec_items` ou `rowid`.

| Objeto (PT) | O que é | Ciclo de vida | Ecrã-dono | Relações |
|---|---|---|---|---|
| Cofre (Vault) | Raiz de arquivos do usuário | Criar/abrir/trocar (S-00, Config) | Config | contém tudo |
| Mesa (Workspace/Board) | Sandbox espacial | Criar, trocar (pílulas), arquivar | Mesa | contém Células, Arestas, Grupos, Peças |
| Item (documento) | `.md` + frontmatter em `ingest/{notes,web,media}` | Máquina de estados (Pending→…→Classified/NeedsManualReview/Failed) | Captura | gera Célula (via arrasto); recebe promoção |
| Célula (nó) | Ideia volátil em `topology.json` | Nasce no board; morre no board; **promoção é opcional** | Mesa | aponta p/ Item (`item_id`); agrupa-se; conecta-se |
| Aresta | Relação declarada A→B com âncora e rótulo | Criada por gesto; dirigida por padrão | Mesa | lê-se no Skeleton Map |
| Grupo | Moldura semântica (título+cor+limites) | Criar, renomear, dissolver (nós sobrevivem) | Mesa | contém Células |
| Peça | Documento editorial com citações | Fork de células; edição em blocos; copiloto | Mesa (expandida) | cita Células (`source_cell_id`) |
| Intenção | Meta/hipótese monitorada (`intentions.json`) | Criar, pausar, aposentar | Chat (config) | vigia tags; dispara alerta quieto |
| Categoria / Etiqueta | Vocabulário controlado (kebab, singular) | Normalização: Levenshtein ≤2 funde, cosseno ≥0.92 vira alias | Captura | N:N com Itens (`item_taxonomy`) |
| Fila de revisão | Itens `NeedsManualReview`/`Failed` | ESVAZIAR é job da Captura | Captura | — |
| Modelo / Provedor | SLM local ou API BYOK | Baixar (opt-in), verificar SHA-256, trocar, testar | Config | LocalLlama vs CloudApi |
| Evento de auditoria | Mutação semântica reversível | `audit_log.db` (alta frequência) + Git opcional | Config | sustenta undo (X-03) |
| Ativo (asset) | Binário por hash SHA-256 | Imutável, content-addressed | (invisível; referenciado) | nunca navegado diretamente |
| Snapshot web | `.md` em `ingest/web` + imagens em assets | Via widget (M-05) ou URL direta | Mesa→Captura | plota como nó antes/depois de ingerir |

---

## 7. Rastreabilidade (cada requisito termina num ecrã)

### 7.1. Requisitos funcionais → superfície

| FR | Enunciado resumido | Superfície | Observação crítica |
|----|-------------------|------------|-------------------|
| FR-001 | Ingestão passiva (drop + pasta vigiada) | S-01 | Drop global Tauri deve funcionar em **qualquer** ecrã e rotear p/ Captura |
| FR-002 | Tudo em `.md` + frontmatter | S-01 (visível) / invisível | P3: origem em disco é exibida, não escondida |
| FR-003 | Classificação 100% local | S-01 (pipeline) | **Norma**: nem com BYOK a Captura usa nuvem (P1) |
| FR-004 | Categoria + ≤5 tags + resumo 1 frase | S-01 (cartão) | Resumo é metadado, não "resposta" (P2) |
| FR-005 | Reaproveitar vocabulário | S-01 (sugestão de tags) | SC-003 (80%) vira métrica visível na gestão de taxonomia |
| FR-006 | Contingência + selo de revisão | S-01 (fila) + X-01 | Degradação silenciosa é proibida: selo + toast |
| FR-007 | Editar tags/categoria a qualquer momento | S-01 (editor de tags) | Inclui fundir/renomear termos (normalizador) |
| FR-008 | Canvas infinito pan/zoom | S-02 | — |
| FR-009 | Criar notas no canvas + instanciar itens | S-02 | Arrasto deve preservar `item_id` (§8 G-03) |
| FR-010 | Reposicionar + conectar | S-02 | Aresta por gesto borda-a-borda (US-3 cen. 2) |
| FR-011 | Persistir e restaurar idêntico | S-02 + X-01 | Crash-recovery narrado (P8, SC-006) |
| FR-012 | 3 níveis de LOD por zoom | S-02 (+generalização P7) | Thresholds 0.6/0.3 são lei |
| FR-013 | Unload do SLM após 5min | X-01 (status) + Config | Modelo descarregado deve ser visível (ícone de estado), nunca surpresa |

### 7.2. Critérios de sucesso → implicação de UI

SC-001 (<1s visível) → otimismo de UI: item aparece como `Pending` imediatamente, hidrata por evento.
SC-002 (<5s tagging) → progresso por estado, não spinner genérico.
SC-003 (80% reuso) → painel de taxonomia com frequência (tabela `taxonomy_terms` já tem `frequency`).
SC-004 (60 FPS/100 nós) → LOD + culling; contador de FPS só em modo diagnóstico (não poluir).
SC-005 (100% offline) → indicador offline-first permanente; zero UI que exija rede (fontes incluídas — §8 G-14).
SC-006 (recovery <1s) → aviso pós-crash com timestamp (P8).

### 7.3. Comandos IPC → superfície

| Comando / Evento | Superfície consumidora |
|---|---|
| `create_vault`, `open_vault` | S-00, Config |
| `ingest_file`, `ingest_url`, `list_ingested_items` | S-01 (+ drop global) |
| `trigger_classification`, `update_item_tags` | S-01 (como "Reprocessar" / "Editar tags") |
| `create_workspace`, `load_board_topology`, `save_board_topology_fast`, `create_cell`, `list_cells` | S-02 |
| `ingest://state-changed` | S-01 (badges ao vivo) + X-01 se falha |
| `model://download-progress` | X-01 (progresso global) + Config |
| `vault://file-watcher-event` | S-01 (lista reativa; ANIMAÇÃO de entrada, não teleporte) |

---

## 8. Auditoria crítica do estado atual (código v0.1)

Veredito: **scaffold fiel à spec v0.1 e ao diagrama defasado de 2 painéis** (architecture.md §1),
não ao modelo de 3 ecrãs. Estrutura, estados e tokens base prestam; navegação, identidade
e hierarquia — as 3 dores declaradas — estão em aberto, como esperado nesta fase.
Achados numerados (G = gap). Todos verificados no código em 2026-09-22.

**Pontos fortes a preservar:** máquina de estados com badges por cor (5 classes em
`IngestItemList.svelte` 195–227); sanitização de snippet (37–49); 3 caminhos de ingestão
+ captura de URL (`Dropzone.svelte`); overlay DOM projetado por matriz de câmera
(`CellOverlay.svelte` 37–52); `aria-label`/ papéis em overlays e dropzone; `lang="pt-BR"`
(`index.html` 2); autosave com debounce no store (450ms, `canvas.svelte.ts` 71–78).

| # | Achado | Evidência | Princípio violado / risco |
|---|--------|-----------|---------------------------|
| G-01 | Shell implementa o diagrama antigo (header + drawer + canvas), não os 3 ecrãs | `App.svelte` 28–59 | D-01: Captura é gaveta de 360px, não ecrã; Chat inexistente |
| G-02 | Nomes de cofre/workspace hardcoded na UI | `App.svelte` 9–10 | P3: identidade falsa; exige S-00 + troca real |
| G-03 | **Arrasto Captura→Mesa perde proveniência**: o payload do drag **contém** `item.id`, mas o handler de drop o ignora e `addNode` nem aceita `item_id` | `IngestItemList.svelte` 79–88 (id presente, desperdiçado); `CanvasViewport.svelte` 44–58 (só lê title/content); `canvas.svelte.ts` 136–149 (sem `item_id`) | P5: nó órfão não pode ser promovido/rastreado; quebra fork futuro |
| G-04 | Seleção sem destino: `selectedItem` só acende highlight | store 8; lista 74–75, 89 | X-06: clicar deve abrir detalhe/ação (Fase 3 desenha o destino) |
| G-05 | Busca dispara reload por tecla, sem debounce | lista 7–9, 53–60 | SC-001 sob risco com 1000 itens; debounce + cancelamento |
| G-06 | **Esc salva em vez de descartar; não há como cancelar edição** | `CellOverlay.svelte` 28–34 | P8 + convenção desktop: Esc deve descartar (ou perguntar); Ctrl+Enter salva |
| G-07 | `user-select: none` global sem exceção de leitura/edição | `tokens.css` 48 | Texto da Captura e do overlay não é selecionável — fere leitura e P8 (copiar p/ Peça) |
| G-08 | Ícones emoji (📥 🔍 🎯 📁 ⚠️) em 4 arquivos, sem sistema de ícones | `App.svelte` 48; controles 33–37; lista 101, 114 | Identidade (D-08/dor 2): `Dropzone` já usa SVG inline (159–163) — esse é o padrão a generalizar |
| G-09 | Sem menus de contexto (promover, duplicar, excluir, fixar) | ausência em `src/` (grep: sem `contextmenu`) | P5/P8: promoção e undo não têm porta de entrada |
| G-10 | IDs de nó via `Date.now()+random`, não UUIDv7 | `canvas.svelte.ts` 138 | Colisão em multi-sessão; contrato canônico é UUIDv7 (data-model §1.1) |
| G-11 | Nós-semente de boas-vindas viram topologia persistida indistinguível de conteúdo | `canvas.svelte.ts` 30–60 | P5: amostra deve ser marcada como efêmera/descartável, não como pensamento do usuário |
| G-12 | Minimapa + presets + smoothing (T044 `[X]`) **não existem no código** | grep `minimap`: zero em `src/` | Risco de "concluído" sem implementação; Fase 3 deve cobrir M-03 |
| G-13 | Dois clamps de zoom divergentes (0.1–3.0 vs 0.15–2.5) | store 95 vs 106 | LOD calibrado em 0.3/0.6 merece clamp único canônico |
| G-14 | **Fontes via CDN do Google** (network no boot) | `index.html` 7–12; CSP permite `fonts.googleapis.com` | Tensão direta com SC-005/offline-first: empacotar fontes localmente |
| G-15 | Feedback só local (banner com timeout), sem sistema de notificações | `Dropzone.svelte` 185–190; store com `console.error` | X-01 inexistente: progresso de modelo, alertas e erros não têm casa |
| G-16 | Arquivos incompatíveis no drop Tauri são **ignorados em silêncio** | `Dropzone.svelte` 23–37 (sem `else`) | Spec edge case 1 exige notificação amigável + quarentena em assets |

---

## 9. Contradições inter-fontes e questões abertas

### 9.1. Contradições (resolução proposta — valem até emenda)

| # | Conflito | Resolução proposta |
|---|----------|-------------------|
| C-01 | `architecture.md` mostra 2 painéis; D-01 manda 3 ecrãs | architecture.md §1 **defasado para navegação**; vale D-01. Motores e vault do doc continuam canônicos |
| C-02 | PRD §5/§7 mostra Peça como **painel lateral docked**; D-05 manda **widget que expande no board** | Vale D-05. Nota: o modelo-widget é *mais* consistente com fork-on-insert e ADR-0003 do que o docked-panel; Fase 3 resolve a geometria (Q-09) |
| C-03 | PRD §9 sugere **criação automática de canvas** por Intenções; P4 proíbe ação automática | **Sugestão, nunca execução**: notificação quieta com CTA "Criar canvas" (P4 > automação) |
| C-04 | Roadmap põe Chat/Intenções na v1.0, mas RRF (T046) e `intentions.json` já existem | Antecipação parcial é legítima (Chat-escopo-Acervo na v0.2); matriz da Fase 2 decide |
| C-05 | `topology.json` vs `board.canvas.json` coexistem em storage.md/constituição sem definição de papéis distintos | Questão de backend (Q-10); UX consome "topologia ativa" e "snapshot versionável" como conceitos, independente de nomes |
| C-06 | `index.html`+CSP assumem rede p/ fontes; SC-005 exige 100% offline | Empacotar Inter + JetBrains Mono localmente (G-14); CSP pode permanecer como fallback |

### 9.2. Questões abertas (dono: Fase 2, salvo indicação)

| # | Pergunta | Contexto | Proposta inicial (a validar) |
|---|----------|----------|------------------------------|
| Q-01 | Onde vivem as Intenções? | `intentions.json` sem surface; PRD §9 | Aba "Intenções" **dentro do Chat** (escopo + monitoramento) + alertas no X-01 |
| Q-02 | Escopos do Chat? | Dualidade de motores (§4.2) | Seletor: Acervo / Mesa ativa / Intenção; padrão = última usada |
| Q-03 | Casa da busca global? | RRF existe; lista filtra local | Captura = busca plena do Acervo; Mesa = palette (board+ações); sem 4ª busca |
| Q-04 | Surface do `scratchpad/`? | Pasta existe, UI não (M-08) | Captura rápida pela palette (nota volátil no board ativo); Fase 2 decide |
| Q-05 | UI de undo/histórico? | `audit_log.db` reversível; sem UI | Ctrl+Z + painel "Histórico" na Mesa; Config expõe trilha (D-07) |
| Q-06 | Central de notificações ou só toasts? | 4 fontes (X-01: modelo, intenções, falhas, recovery) | Toasts + sino com central (progresso de modelo exige persistência) |
| Q-07 | Fluxo de troca de vault? | Só `create/open` via IPC | Config + S-00; switcher compacto no shell (nome real, não hardcoded — G-02) |
| Q-08 | Gestão de taxonomia? | `taxonomy_terms.frequency` sem UI | Secção da Captura: termos, frequência, fundir/renomear (FR-005/007, SC-003) |
| Q-09 | Geometria da Peça expandida? (Fase 3) | D-05 vs PRD docked-panel (C-02) | Expansão sobre o board com contexto visível ao redor; copiloto em coluna interna |
| Q-10 | Papéis `topology.json` × `board.canvas.json`? | C-05 | Dono: backend; UX trava conceitos, não nomes |
| Q-11 | Revisão: filtro ou view dedicada? | Só contador existe (G-04 aparentado) | Filtro persistente + atalho; view dedicada se fila > N (Fase 2 calibra) |
| Q-12 | Nuvem configurada afeta a Captura? | BYOK + FR-003 | **Não**: classificação sempre local (P1, §7.1). Nuvem só Mesa/Chat com badge |

---

## 10. Glossário mínimo (vale na Fase 2 em diante)

Acervo = `ingest/` pesquisável · Captura = ecrã de entrada · Mesa = workspace ativo ·
Célula = nó volátil · Peça = editorial com citações · Intenção = meta monitorada ·
Promover = célula→nota (gesto) · Fork = célula→Peça (com citação) · Provedor = local ou BYOK ·
Pipeline = classificação invisível (nunca "a IA" na Captura) · Escopo = Acervo/Mesa/Intenção no Chat.

---

## 11. Handoff para a Fase 2 (Arquitetura da Informação)

A Fase 2 deve entregar, nesta ordem: (1) árvore de navegação dos 3 ecrãs + Config com
as zonas de cada um; (2) matriz telas×versão resolvendo Q-01–Q-04, Q-07–Q-08, Q-11–Q-12
e a antecipação do Chat (C-04); (3) fluxos-core §2.2 com os gestos §4.3;
(4) regras de palette (ações tipadas, escopo declarado) preparando M-01;
(5) modelo do X-01 (notificações) cobrindo as 4 fontes de eventos.
Insumos prontos: princípios (§3), superfícies (§5), objetos (§6), rastreabilidade (§7),
auditoria (§8). Nada na Fase 2 pode violar D-01–D-09 nem P1–P8 sem emenda aqui.
