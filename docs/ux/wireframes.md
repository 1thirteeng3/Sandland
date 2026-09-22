# Wireframes Estruturais — Sandland

**Fase:** 3 de 5 (Wireframes) · **Status:** Aprovado para derivação da Fase 4 (Design System)
**Data:** 2026-09-22 · **Arquivo:** `docs/ux/wireframes.md`
**Insumos:** `fundamentos.md` (D-01–D-09, P1–P8) + `ia.md` (D-10–D-23, zonas, fluxos F-01–F-10).
Fidelidade: **low-fi anotado** (skill `wireframe-spec`): só estrutura e comportamento,
sem cor, sem pixel, sem tipografia. Geometria vinculante só onde trava decisão (Q-05, Q-09).

---

## 1. Convenções de leitura

- Molduras `┌─┐` = regiões; `···` = texto placeholder; `×××` = mídia/imagem; `●○◐` = estados.
- Anotações `①②③` abaixo de cada frame: prioridade de conteúdo, interação, conteúdo dinâmico.
- Acessibilidade marcada como `[a11y]`; responsivo como `[resp]`; fonte de dados como `[src]`.
- Breakpoints estruturais (janela mín. 960px, `tauri.conf.json`): **≥1200px** = tudo ancorado;
  **960–1199px** = inspetor da Captura e painel de histórico viram overlay; Mesa não muda
  (canvas é fluido por natureza). Regra, não desenho.
- Rótulos em vocabulário do usuário (§2-ia): Captura, Mesa, Chat, Peça, Intenção, Acervo.

---

## 2. W-00 — Shell global (D-10)

```
┌──────────────────────────────────────────────────────────────────────────┐
│ [S] SANDLAND │ (●)Captura │ ( )Mesa │ ( )Chat │ ▾ Meu Cofre ▾ │ [Local✓] [🔔¹] [⚙] │
├──────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│                        ÁREA DO ECRÃ ATIVO (S-01/S-02/S-03/S-04)           │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

① Switcher: 3 itens + ativo por peso+barra (nunca só cor); badge numérico só em Captura
(= `N a rever`, D-17); atalhos `1/2/3`. ② Cofre: nome real (`VaultInfoDTO`; fixa G-02);
dropdown = recentes + Abrir/Criar (F-08); gestão plena em Config. ③ `[Local✓]`/­`[Nuvem]`:
provedor da Mesa+Chat; na Captura vira chip estático "100% local" (P1/FR-003); clique →
Config › Modelos. ④ Sino: badge = warning+error+progress+quiet não lidos (§7-ia);
abre central. ⑤ Engrenagem: Config (utility, separada do primário).
`[a11y]` switcher = `nav` com `aria-current`; sino = `aria-live polite` no badge.
`[resp]` <1200px: rótulos do switcher colapsam p/ ícones+title; ordem e posição intactas.

---

## 3. W-01 — Captura (S-01; ex-gaveta vira ecrã — G-01)

```
┌──────────────────────────────────────────────────────────────────────────┐
│ SHELL (W-00)                                                             │
├──────────────────────────────┬───────────────────────────────────────────┤
│ Z-C1  [Buscar no Acervo····] │ Z-C4 INSPETOR (destino da seleção — X-06) │
│  [Todas▾] [A rever (3)│R]   │  Título da nota selecionada               │
│  1.240 itens · índice ok ●   │  selo ESTADO · origem: notes/····.md      │
├──────────────────────────────┤  resumo de 1 frase (metadado, P2)         │
│ Z-C2  ╭ drop: solte .md ──╮  │  [categoria▾] [#tag ×][#tag +]            │
│       │ ou clique p/ escolher│  trecho ·······························    │
│       ╰─────────────────────╯  │  [Enviar à Mesa] [Reprocessar]            │
│       [colar URL·······][Capturar]├───────────────────────────────────────────┤
│ Z-C3  LISTA (densidade: ○Linhas ●Cartões) │ Z-C5 TAXONOMIA (aba/secção)   │
│ ┌ card: ●título · selo ─────┐ │  termo ─────── 48× ·······               │
│ │ ····snippet 2 linhas······ │ │  termo ─────── 12× [Fundir]             │
│ │ [cat] #t #t #t │ 12:04    │ │  reuso do vocabulário: 82% (SC-003)     │
│ └ ⚠ Requer revisão ─────────┘ │                                           │
└──────────────────────────────┴───────────────────────────────────────────┘
```

① Z-C1: busca plena RRF (placeholder ensina escopo: "no Acervo"); filtro "A rever (N)"
persistente + `R`; linha de origem/estado do índice (P3: "índice ok / hidratando…");
digitação com debounce 250ms (mesmo do watcher, T018) + cancelamento de consulta anterior (G-05).
② Z-C2: dropzone + URL; absorção confirma em <1s como `Pending` no topo (SC-001, F-01);
arquivo incompatível = **aviso + quarentena** (G-16), nunca silêncio.
③ Z-C3 densidades (P7; toggle manual, default Cartões — sem mágica, P4):
- **Linha:** dot-estado + título + selo + data (varredura massiva).
- **Cartão:** + snippet 2 linhas + categoria + até 3 tags + selo revisão (padrão).
- **Expandido** = abre Z-C4 (não é 3º modo de lista: detalhe mora no inspetor, X-06).
Entrada de item novo = animação de inserção no topo (evento watcher; remoção externa =
toast-desfazer, §7-ia). ④ Z-C4: destino obrigatório da seleção (fixa G-04); resumo
rotulado como metadado (P2, nunca "resposta da IA"); tags editáveis (FR-007);
"Reprocessar" = `trigger_classification` despido de IA (D-02); "Enviar à Mesa" = F-02
com `item_id` preservado (G-03). ⑤ Z-C5: termos + frequência + fundir/renomear (D-16);
% de reuso SC-003 como número.
`[a11y]` lista = `listbox` com setas; cards arrastáveis anunciam destino ("solte na Mesa");
selo de estado tem texto, não só cor. `[resp]` <1200px: Z-C4 vira overlay direita com
mesmo conteúdo; Esc fecha e devolve foco à lista.

**Estados Z-C3/Z-C4:** vazio virgem ("Solte o primeiro .md" + atalho Tour); vazio de busca
("Nada no Acervo p/ ‘x’" + limpar); fila zerada ("Tudo classificado ✓"); item
`Classifying` (badge pulsante + sem spinner global); `Failed` (cartão com erro + Repetir);
`DatabaseError` = faixa P3 ("arquivos intactos · Reconstruir índice") — §7.3-ia.

---

## 4. W-02 — Mesa (S-02)

### 4.1. Base + LOD (lei: 0.6 / 0.3 — FR-012)

```
┌──────────────────────────────────────────────────────────────────────────┐
│ SHELL │ Z-M1 [Mesa Principal▾][Tese▾][Ideias▾][+ Nova]  (pílulas — v0.2)  │
├──────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  Z-M2 VIEWPORT:      (zoom 100%: LOD completo)                            │
│   ┌──────────────┐      ┌──────────────┐                                  │
│   │ ● Título     │ ───► │ ● Título     │   aresta dirigida, rótulo opcional│
│   │ ····corpo··· │ edge │ ····corpo··· │                                  │
│   │ [cat] #t #t  │      │ [Peça▸ 3]    │   M-06 colapsada = nó especial   │
│   └──────────────┘      └──────────────┘                                  │
│   ╭──────── Grupo "Cap.2" ────────╮                                       │
│   │ ┌─────┐            ┌─────┐    │                                       │
│   └──────────────────────────────────────────────────────────────────────┘
│                                        Z-M3  [+] [─ % ─] [+] [reset] [hist]│
└──────────────────────────────────────────────────────────────────────────┘
```

LOD por zoom (P6+P7; posição/grupo/aresta intactos em todos):
- **>0.6 completo:** título + corpo + tags + categoria + handles de aresta.
- **0.3–0.6 intermediário:** título + contorno colorido por tipo + ícone; corpo some.
- **<0.3 panorâmico:** proxy sólido opaco; título some; grupos viram molduras nomeadas.
① Espaço é semântica (P6): sem auto-layout; sobreposição = smart offset (spec edge-4);
grupos dissolvem sem matar nós. ② M-06 colapsada distingue-se por selo `Peça▸ N`
(N = citações) — abre conforme D-24 (§9). ③ Z-M3: criar, zoom±, % (mono), reset,
`hist` (painel Q-05/D-25); minimapa acoplado aqui na v0.2 (G-12); clamp de zoom
único e canônico **[0.15, 2.5]** em toda a app (fixa G-13 — vence o `zoomAt`;
auditoria Fase 5).
`[a11y]` canvas = `region` rotulada; nós focáveis por Tab em ordem espacial aproximada;
toda ação do mouse tem equivalente em M-07/palette (P8 §8-ia regra 6).

### 4.2. Overlays da Mesa

**M-02 editor de célula** (duplo clique; existente, corrigido): overlay projetado sobre o
nó (título + textarea markdown + rodapé); **Esc descarta** (fixa G-06), `Ctrl+Enter`
salva; `user-select:text` dentro (fixa G-07); blur = salva rascunho no nó (volátil, P5),
não promove.

**M-07 contexto** (botão direito em nó / aresta / fundo — hoje inexistentes, G-09):
- nó: Promover a Nota · Enviar ao Chat · Duplicar-view · Excluir (2-passos) [v0.1; Chat-gated v0.2]
- aresta: Renomear rótulo · Inverter · Excluir · nó-fonte/­destino em destaque
- fundo: Nova célula aqui · Colar nota rápida · Centralizar · (v0.2) Nova Peça aqui

**M-05 widget web** (D-23): estados `consulta/URL → carregando (fast) → resultados
plotáveis → plotado ⇄ [Enviar à Captura]`; bloqueio = cartão "bloqueado" + botão
"Usar extração avançada" (consentimento stealth + progresso do Chromium em X-01);
nada retry sozinho (P4).

### 4.3. M-01 palette (global; IA só na Mesa — D-19)

```
┌─────────────────────────────────────────────────┐
│ › pro······                          [Mesa "Tese"]│
├─────────────────────────────────────────────────┤
│ NAVEGAR                                         │
│ → Ir p/ Captura / Chat / Config · Trocar de Mesa│
│ CRIAR                                           │
│ + Nota rápida (edita no board) · Nova célula    │
│ BOARD  [Escopo: Mesa "Tese" · 42 nós]           │
│ ◈ Promover seleção · Excluir (2-passos)…        │
│ IA     [Local Qwen✓] [desfazível]               │
│ ✦ Sintetizar board · Resumir seleção…           │
│ ── sem provedor: "Configure um modelo p/ IA  [Configurar]" (D-22)
│ SISTEMA                                         │
│ ↺ Desfazer · Histórico · Abrir Config › Modelos │
└─────────────────────────────────────────────────┘
```

① Agrupamento fixo na ordem acima; query filtra nós (título) + ações. ② Todo item de IA/
mutação exibe a linha de escopo+provedor+reversibilidade (§6-ia). ③ Excluir e ações
destrutivas = confirmação inline no rodapé (2-passos). ④ v0.1: secção IA ausente
(sem placeholder fantasma — honestidade > teaser). ⑤ Vazio: "Nada aqui" + 3 sugestões.
`[a11y]` `dialog` modal com foco preso, setas+Enter, Esc fecha e devolve foco (§8-ia).

---

## 5. W-03 — Chat (S-03; v0.2 Acervo — D-18)

```
┌──────────────────────────────────────────────────────────────────────────┐
│ SHELL                                                                    │
├──────────────────────────────────────────────────────────────────────────┤
│ Z-H1 Escopo: (●)Acervo │ ( )Mesa "Tese" │ ( )Intenção ▾   [? o que muda] │
├──────────────────────────────────────────────┬───────────────────────────┤
│ Z-H2 THREAD (1 por escopo — D-21)            │ Z-H5 INTENÇÕES (aba)      │
│  eu: ········                                │  Meta: ·········          │
│  ✦: resposta com [1][2] citações ────────────┤  Hipóteses: ·· [+]        │
│     [1] "trecho…" — nota X [abrir]           │  Tags vigiadas: #a #b     │
│     [Aplicar no board] (gesto explícito, P4) │  afinidade: ▓▓░░ (quiet)  │
├──────────────────────────────────────────────┤  [Pausar] [Aposentar]     │
│ Z-H3 [Perguntar sobre o Acervo·······] [➤]   │                           │
└──────────────────────────────────────────────┴───────────────────────────┘
```

① Escopo declara o motor (RRF vs Skeleton-v1.0) e o que é citável (D-12; default =
último). ② Citação = trecho + fonte + [abrir] (Acervo→Captura; Mesa→foca nó);
"Aplicar no board" plota e troca p/ Mesa (F-05). ③ Composer puro, Enter envia, sem
slash-commands (D-04). ④ Z-H5 declara/pausa/aposenta intenções; afinidade acumulada
visível; alerta ≥0.85 sai quieto no X-01 (C-03). Vazios: 1ª thread sugere 3 perguntas
neutras (N-04 calibrado p/ P4: sugestão, não execução).

---

## 6. W-04 — Configurações (S-04) + S-00

**Config** (4 secções, nada de conteúdo do usuário): `Cofre` (nome + pasta-resumo
— caminho absoluto só em "detalhes", P3/ADR-0004; abrir/trocar/criar; Reindexar) ·
`Modelos & chaves` (rádio Local/Nuvem; Local = lista GGUF + baixar + SHA ✓ + progresso;
Nuvem = chave por provedor + [Testar] + cota/erro; D-22 nasce aqui) ·
`Auditoria & governança` (trilha: tempo, evento, entidade, reversível, Desfazer;
Git opcional on/off; D-07) · `Sobre` (versões app/constituição; diagnósticos incl. FPS).

**S-00** (3 passos, D-15): `1 Cofre` (criar/abrir; erros §7.3-ia inline) →
`2 Modelos` (baixar SLM opt-in com progresso OU Pular — Captura funciona sem IA) →
`3 Tour` (3 frases: Captura/Mesa/Chat + amostra marcada "exemplo — descartável", G-11)
→ pouso na Mesa.

---

## 7. W-X01 — Notificações (§7-ia; fixa G-15)

Toast (baixo-direita): `┃sev │ Título │ corpo 1 linha │ [Ação] [×]`; variante progresso
com barra + % persistente. Sino→central: lista da sessão (máx. 50), filtro Todas/Não
lidas, "marcar lidas"; badge = warning+error+progress+quiet. Quiet de Intenção: toast
sem som, 10s, CTA [Criar canvas] (sugere, nunca executa — C-03). Boot pós-queda: faixa
topo-central "Recuperado até HH:MM:SS · perda <1s [ver auditoria]" (F-09), sem modal.
Watcher-remoção: toast com Desfazer (N-03 desenha semântica na Fase 4? Não — N-03 é
decisão de produto: **Desfazer restaura o arquivo da lixeira do SO se existir, senão
oferece reimportar** — registrado aqui como regra para a Fase 4 detalhar).

---

## 8. Cobertura dos fluxos (F-01–F-10 × wireframes)

| Fluxo | Estados cobertos em |
|-------|---------------------|
| F-01 ingest→revisão | W-01 (Z-C2 entrada, Z-C3 badges/animação, filtro R, Failed, DatabaseError, incompatível) |
| F-02 →Mesa | W-01 Z-C4 [Enviar à Mesa] + W-02 nó vinculado + toast |
| F-03 web→plot→ingerir | W-02 M-05 (URL/fast/bloqueio/stealth-consent/progresso) + F-01 |
| F-04 promover | W-02 M-07 + W-01 entrada animada + chip-fonte no nó |
| F-05 chat→board | W-03 (escopo, citações, Aplicar) + W-02 plotagem |
| F-06 taxonomia | W-01 Z-C5 (frequência, fundir, % SC-003) |
| F-07 onboarding | W-00/S-00 (3 passos, erros inline, pouso) |
| F-08 trocar vault | Shell dropdown + W-04 Cofre + estado "hidratando" (P3) |
| F-09 recovery | W-X01 faixa de boot + W-04 auditoria |
| F-10 undo | W-02 (Ctrl+Z) + D-25 painel; IA/promoção com entrada própria |

---

## 9. Geometrias vinculantes (Q-05, Q-09 resolvidos → D-24, D-25)

### D-24 — Peça expande ancorada ao nó (resolve Q-09; vale sobre C-02)
A Peça colapsada é um nó especial (`Peça▸ N`). Expandir = **folha ancorada na posição
do nó** (origem da animação = nó; P6: contexto espacial preservado), crescendo até
**60% da largura do viewport**, em 2 colunas internas (blocos | copiloto — PRD §7).
Cabeçalho: breadcrumb `Mesa › Grupo › célula-origem` + chips de proveniência clicáveis
(`source_cell_id`, trecho — CONST II) que focalizam a fonte. Folha **não-modal**:
board atrás permanece navegável; Esc colapsa preservando rascunho (P5/P8); fork por
arrasto de célula p/ a folha ou palette. Rejeitadas: modal centrada (quebra P6),
takeover fullscreen (perde o board), painel fixo docked (viola D-05 e ocupa região
permanente). Colapsar retorna ao nó de origem — ida e volta sem perda (D-20).

### D-25 — Histórico como painel de borda (resolve Q-05)
Painel overlay na borda direita da Mesa (toggle `hist` em Z-M3, palette "Histórico",
atalho proposto `Ctrl+Shift+H` — Fase 4 valida): entradas agrupadas por sessão/marco,
cada uma com ícone, descrição PT, hora e selo `reversível`; **"Desfazer até aqui" por
entrada** e Ctrl+Z linear funcionam desde v0.1 via pilha de comandos sobre a topologia
+ save integral (sem IPC novo). Leitura da trilha `audit_log` no painel (filtro,
busca, "ver no board") exige **`list_audit_events` (B-03, backend, v0.2)**.
`<1200px`: painel vira overlay total-direita; Esc fecha e devolve foco ao canvas.

---

## 10. Handoff Fase 4 + conformidade

**Fase 4 (Design System) deve:** tipografia em 2 registros (UI + leitura editorial,
P7) sobre Inter/JetBrains empacotadas (G-14); escala de espaçamento p/ densidades W-01;
cores semânticas por zona (captura/mesa/chat/revisão/alerta) + selos de estado F-01;
elevação p/ shell/overlays/folha D-24/toasts; motion (inserção F-01, expansão D-24,
LOD) com `prefers-reduced-motion`; foco visível global (§8-ia); sistema de ícones SVG
(padrão `Dropzone`, aposenta emoji — G-08); `user-select:text` em leitura/edição (G-07);
spec dos componentes-base usados aqui (botão, selo, chip, cartão-linha, toast, sino,
segmentado, pílula, overlay, folha, painel, tabela-taxonomia). N-03 registrado em §7.

**Conformidade:** zero cor/decisão visual além do estrutural (D-08 intacto: "só Dark"
é restrição, não palette); D-01–D-23 respeitados (3 ecrãs; IA só via palette na Mesa
e no Chat; Peça-widget D-24 não cria 4º ecrã); P1–P8: P1 badge/chip+consentimentos
(W-00/M-05/Config), P2 zonas sem mistura, P3 origem/índice/paths, P4 modos+quiet,
P5 volátil→gesto+proveniência, P6 LOD/D-24/sem auto-layout, P7 densidades+LOD-lei,
P8 undo/anúncio/auditoria. Cobertura: todos os fluxos F-01–F-10 têm ao menos um frame;
todas as zonas §3-ia têm frame; B-01/B-02/B-03 marcados onde a UI encosta no backend.
