# Consolidação e Congelamento da Baseline — Sandland UX

**Fase:** 5 de 5 (Consolidação) · **Status:** BASELINE CONGELADA
**Data:** 2026-09-22 · **Arquivo:** `docs/ux/consolidacao.md`
**Escopo:** auditar os 4 documentos + tokens, aplicar correções de auditoria,
registrar decisões, congelar D-01–D-26 + P1–P8. Nenhuma feature nova.

---

## 1. Correções de auditoria aplicadas nesta fase

| # | Achado da auditoria | Correção | Arquivo |
|---|---------------------|----------|---------|
| A-01a | `--text-muted #647087` = **3.85:1** (calculado), não ~4.3 — abaixo do AA | → `#758092` (**4.81:1**) | `tokens.css`, design-system §2 |
| A-01b | Texto violet sobre seu tint = **3.95:1** (falha AA) | Novo `--state-ai-fg-strong #a78bfa` (6.15 sobre tint, 7.05 sobre bg); selo violet OBRIGATÓRIO strong | `tokens.css`, design-system §§2/8 |
| A-01c | Blue sobre tint 0.15 = 4.45:1 (marginal) | `--state-active-bg` 0.15 → **0.10** (4.75:1) | `tokens.css`, design-system §2 |
| A-01d | Valores "aprox." na tabela de cor divergiam do cálculo | Tabela reescrita com razões exatas (17.26 / 7.91 / 8.94 / 7.57 / 5.23…) | design-system §2 |
| A-02 | G-13 sem valor: dois clamps divergentes no código (0.1–3.0 vs 0.15–2.5) | Clamp único canónico **[0.15, 2.5]** (vence `zoomAt`) | wireframes §4.1 |
| A-03 | G-05 sem número (debounce indefinido) | **250ms** (mesmo do watcher, T018) + cancelamento | wireframes §3 |
| D-26 | N-02 sem dono (composer aceita anexo?) — única questão Fase 3 sem resolução | **Resolvido:** composer aceita anexos explícitos (célula/item) como chips de contexto; conta como gesto invocado (P4); sem arrasto implícito | este doc §4; spec na implementação |

Verificações re-executadas após as correções: paridade `var()` **OK** (103 defs × 29 usos,
0 faltantes), retrocompatibilidade **OK** (25/25 tokens v0.1), `tauri.conf.json` válido,
4 fontes variáveis no disco, `index.html` sem CDN.

---

## 2. Princípios P1–P8 → evidência (rastreabilidade de imposição)

| P | Onde é imposto (documento:secção + token onde houver) |
|---|--------------------------------------------------------|
| P1 Soberania | fund §0 D-02-leitura · ia §1 badge/chip + §8.8–8.9 + matriz FR-003 · wire W-00③ · ds §§2/8.15 (`--provider-*`, chip "100% local") |
| P2 Separação | fund §4 · ia §2 árvore + D-19 + matriz (IA só Mesa/Chat) · wire W-01–W-03 sem mistura · ds §1 |
| P3 Arquivo-verdade | fund P3 · ia §7.3 DatabaseError + §8.2 + rótulos §2 · wire W-01 Z-C1/Z-C4 + W-04 Cofre · ds §§2/8.17 |
| P4 Invocada | fund P4 · ia §6 + D-22 + C-03/quiet · wire M-01/M-05/W-03/D-26 · ds §§5/8 (sem spring, sem auto) |
| P5 Promover-gesto | fund P5 · ia F-02/F-04 + D-14 · wire M-02/M-07/D-24 + G-03/G-11 · ds §§8.9/8.11/8.16 |
| P6 Espaço-significa | fund P6 · ia §3 Z-M2 + LOD-lei · wire §4.1 + D-24 (rejeita modal/takeover) · ds §7 |
| P7 Densidade-zoom | fund P7 · ia §3 + FR-012 · wire 3 densidades + 3 LODs · ds §§3/7 (`--text-*`, `--measure`) |
| P8 Tudo-desfaz | fund P8 · ia F-09/F-10 + X-03 + §7 · wire W-X01 + D-25 · ds §§4/8 (`:focus-visible`, histórico) |

Precedência P1>…>P8 auditada: nenhum par de secções impõe ordens conflitantes;
o único conflito potencial (P4×P8, IA reversível) está resolvido em fund §3.

---

## 3. Registro de decisões congeladas (D-01–D-26)

**Fase 1** (fundamentos): D-01 3 ecrãs · D-02 Captura sem IA interativa (+leitura
pipeline-invisível) · D-03 palette-first na Mesa · D-04 Chat conversacional ·
D-05 Peça-widget · D-06 Descoberta-widget→plot→ingerir · D-07 Governança em Config ·
D-08 só Dark · D-09 só spec+tokens.
**Fase 2** (ia): D-10 shell · D-11 Intenções no Chat · D-12 escopos · D-13 duas buscas ·
D-14 captura rápida `N` · D-15 vault+onboarding · D-16 taxonomia na Captura ·
D-17 filtro "A rever" · D-18 Chat-Acervo v0.2 · D-19 palette global/IA-na-Mesa ·
D-20 preservação de estado · D-21 thread-por-escopo · D-22 IA-indisponível-com-CTA ·
D-23 web em 2 estágios.
**Fase 3** (wireframes): D-24 Peça ancorada ao nó (60%, não-modal, Esc colapsa) ·
D-25 histórico em painel de borda (undo linear v0.1 sem IPC; trilha = B-03).
**Fase 5** (este doc): D-26 anexos explícitos no composer.
Emenda a qualquer D exige justificativa + re-auditoria das secções que o citam
(§5 lista as citações por ID — todas verificadas existentes).

---

## 4. Ledger de questões (16/16 com estado final)

| Q | Estado | Q | Estado |
|---|--------|---|--------|
| Q-01 | → D-11 | Q-07 | → D-15 |
| Q-02 | → D-12 + D-21 | Q-08 | → D-16 |
| Q-03 | → D-13 | Q-09 | → D-24 |
| Q-04 | → D-14 | Q-10 | ABERTA, dona: backend (C-05 desacopla a UX: valem os conceitos, não os nomes) |
| Q-05 | → D-25 | Q-11 | → D-17 |
| Q-06 | → ia §7 | Q-12 | regra (classificação sempre local; ia matriz S-01) |
| N-01 | spec ds §8.8 (fixar + "todas") | N-02 | → D-26 (este doc) |
| N-03 | regra wire §7 (lixeira, senão reimportar) | N-04 | wire W-03 (3 perguntas neutras) |

---

## 5. Ledger de gaps do código (G-01–G-16)

- **Fechados em docs/tokens nesta rodada:** G-01 (W-01) · G-02 (D-10/D-15) · G-04 (Z-C4)
  · G-05 (A-03) · G-06 (M-02) · G-07 (`.allow-select` + §4-ds) · G-08 (ds §6) ·
  G-11 (S-00 + amostra marcada) · G-12 (documentado; minimapa v0.2) · G-13 (A-02) ·
  G-14 (fontes locais + CSP) · G-15 (W-X01) · G-16 (aviso + quarentena, W-01).
- **Backlog de implementação** (não desta rodada, D-09): G-03 (propagar `item_id` no
  drop + `addNode`) · G-09 (menus M-07) · badges→`--state-*` + emoji→§6-ds
  (lista de migração em ds §9) · I-01 (IDs de nó em UUIDv7, não `Date.now` — G-10).
- **Backlog de backend:** B-01 `list_workspaces` · B-02 suite `chat/*` · B-03
  `list_audit_events` (+ undo-até-aqui) · Q-10 nomes de topologia.

---

## 6. Riscos residuais e recomendação antes de implementar

1. **Rótulos e IA não validados com usuários** (skills IA recomendam tree-test +
   first-click): rodar tree-test do sitemap (§2-ia) e first-click em
   Captura/Mesa/Chat/Peça/Intenção antes da Fase de implementação.
2. **Densidade em 960px** é regra estrutural, não desenho testado: validar W-01 com
   inspetor-overlay no protótipo.
3. **Taxonomia com 1000+ termos** (escala v0.1): Z-C5 prevê tabela + frequência, mas
   virtualização de lista longa é requisito de implementação, não de spec.
4. **Performance de animação** (inserção F-01, expansão D-24) deve ser medida em
   hardware real de gama média, não só no dev machine (gates: 60 FPS, boot 2.2s).
5. **Chaves BYOK e screenshots**: nunca exibir chave completa (mascarar `sk-…••••`);
   regra implícita em P1, explícita aqui para a implementação.

---

## 7. Mapa final e freeze

`docs/ux/`: **fundamentos.md** (porquê) → **ia.md** (estrutura) →
**wireframes.md** (forma) → **design-system.md** (matéria) → **consolidacao.md**
(prova). Tokens vivos em `src/styles/tokens.css` (103 vars); assets em
`src/assets/fonts` (4 woff2 variáveis); `index.html` + CSP offline-puros.
**Congelado em 2026-09-22:** D-01–D-26, P1–P8 (ordem), LOD 0.6/0.3, clamp [0.15, 2.5],
debounce 250ms, contraste AA verificado por cálculo, Dark único, 3 ecrãs.
Próxima mudança estrutural = emenda aqui, não no código.
