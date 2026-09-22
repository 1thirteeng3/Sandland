# Design System — Sandland

**Fase:** 4 de 5 (Design System) · **Status:** Aprovado para derivação da Fase 5 (Consolidação)
**Data:** 2026-09-22 · **Arquivo:** `docs/ux/design-system.md`
**Insumos:** fundamentos (P1–P8) + ia (D-10–D-23, §6–§8) + wireframes (W-00–W-X01, D-24/D-25).
Tokens reais: `src/styles/tokens.css` (expandido nesta fase, 100% aditivo — D-09:
nenhum componente Svelte foi criado ou alterado; só spec + tokens).

---

## 1. Identidade: mesa de pesquisa, editorial-técnica, dark soberana

O Sandland parece **uma mesa de pesquisador à noite**: superfícies escuras foscas,
vidro pontual (painéis com blur — precedente v0.1: drawer, controles, overlay),
fios finos (hairlines `#2d3442`), e cor usada como **etiqueta**, nunca como decoração.
É o oposto do "azul/roxo genérico": os 6 acentos herdados viram vocabulário fechado
com significado fixo (§3). Brilho existe num só lugar: o anel do editor de célula
ativa (`--glow-focus`) — luz = "aqui se edita".

- **É:** contido, tipográfico, mono onde há verdade de sistema (IDs, %, paths,
  timestamps), editorial onde há leitura (resumos, Peça, citações).
- **Não é:** lúdico (sem spring/bounce — motion §7), colorido por estado de humor,
  claro (D-08: não existe light; "light será dark" = tema único).
- **Assinaturas:** chip "100% local" na Captura (P1); selos de pipeline com rótulo
  (P3); folha da Peça ancorada ao nó (D-24); densidade que segue o zoom (P7).

---

## 2. Cor (skill `color-system`, só Dark)

Camadas: **superfície** (4 bgs + 3 bordas + 3 textos, herdados) → **semântica**
(estados + zonas + provedor, aliases em `tokens.css` §4) → **identidade por restrição**
(nenhuma matiz nova sem emenda).

| Uso | Tokens | Contraste (calculado Fase 5, script WCAG) | Regra |
|-----|--------|-------------------------------------------|-------|
| Texto corpo | `--text-primary #f0f3f8` | 17.26:1 AAA | default |
| Texto secundário | `--text-secondary #9da7b8` | 7.91:1 AA | metadados importantes, labels |
| Texto apagado | `--text-muted #758092` | 4.81:1 AA (era `#647087` a 3.85 — corrigido Fase 5) | reservado a timestamps/hints por contenção |
| Pipeline ok | `--state-ok-*` emerald | 7.57:1 / 6.16 sobre tint | Classified, Local, índice ok |
| Pipeline ativo | `--state-active-*` blue | 5.22 sobre bg / 4.75 sobre tint 0.10 | Extracting + rótulo SEMPRE (tint 0.10, não 0.15) |
| Pipeline IA | `--state-ai-*` + `--state-ai-fg-strong #a78bfa` | purple 4.53 sobre bg (gráficos); **strong 6.15 sobre tint / 7.05 sobre bg** | texto de selo violet OBRIGATÓRIO strong (Fase 5) |
| Atenção | `--state-pending/review-*` amber | 8.94 / 7.08 e 6.05 sobre tints | Pending + fila de revisão (review bg 0.22 p/ separar de pending 0.15) |
| Falha | `--state-failed-*` rose | 5.23 / 4.56 sobre tint | Failed + rótulo + ação (nunca só cor) |
| Zonas | capture cyan / mesa blue / chat purple | — | assinatura contida por ecrã (§1); chat-purple = IA conversacional |
| Provedor | local emerald / cloud amber | — | nuvem = cautela (P1); streaming declara "tokens saindo" |

**Daltonismo e redundância:** todo estado = cor + rótulo textual + (onde couber) ícone;
selos v0.1 já fazem isso (`IngestItemList` 195–227) — a regra congela o acerto e corrige
a divergência: **review = amber, failed = rose** (no código v0.1 ambos são rose; migração
na implementação). Proporção mínima de alvo clicável desktop: 28px (44px onde houver
toque); todo ícone crítico anda com texto (§9).

---

## 3. Tipografia: dois registros (P7)

Inter (UI + editorial) e JetBrains Mono (verdade de sistema) — **empacotadas localmente**,
variáveis (fvar verificado via fontTools: Inter wght 100–900, Mono 400–800), subsets
latin + latin-ext (ã õ ç â ê ô cobertos), `font-display: swap`, zero rede (G-14/SC-005).
Pesos usados: 400–700 (dentro de ambos os intervalos). Licença permissiva OFL, padrão
Google Fonts — confirmar texto da licença no ato de release.

| Token | px | Registro | Uso (mapeia valores rem já usados no v0.1) |
|-------|----|----------|-------------------------------------------|
| `--text-2xs` | 11 | UI | selos (0.6875rem existente), micro-metadados |
| `--text-xs` | 12 | UI | captions, timestamps, hints (0.75rem) |
| `--text-sm` | 13 | UI | corpo UI padrão (0.8125rem) |
| `--text-base` | 14 | UI | títulos de cartão (0.875rem) |
| `--text-md` | 15 | UI | títulos de secção (0.9375rem) |
| `--text-lg` | 16 | Editorial-base | mínimo de leitura; lead pequeno |
| `--text-xl` | 20 | Editorial | H3 |
| `--text-2xl` | 24 | Editorial | H2 |
| `--text-3xl` | 32 | Editorial | H1 / título da Peça |

Alturas: `--leading-tight 1.2` (títulos), `--leading-normal 1.5` (UI),
`--leading-relaxed 1.75` (leitura longa). Tracking: tight −0.02em (títulos grandes),
wide 0.05em (caixa-alta, selos). Medida editorial: `--measure 65ch`.
Mono obrigatório em: zoom %, timestamps, hashes/IDs curtos, paths-resumo, cotas,
código. Regras: corpo editorial nunca <16px; UI densa nunca <11px; caixa-alta só com
tracking-wide + peso ≥600.

---

## 4. Espaçamento, elevação, z-index, foco, seleção

- **Espaço base 4px:** 4/8/12/16/24/32/48/64 (`--space-1…16`); padding de cartão = 12,
  de painel = 16, gap de lista = 8 (congela proporções v0.1 que funcionam).
- **Elevação:** `elev-1` repouso · `elev-2` controles/cartão elevado ·
  `elev-3` overlays/palette/folha/toasts; `glow-focus` exclusivo do editor ativo.
- **Ordem z canônica:** canvas 0 < drawer 20 < controles 30 < shell/overlay 50 <
  histórico 55 < palette/folha 60 < toasts 70 < anúncio 80. Sobreposição fora dessa
  ordem é bug.
- **Foco:** `:focus-visible` global (anel `--border-focus` + `--focus-ring`); mouse
  não vê. Todo fluxo é operável por teclado na Mesa (§8-ia regra 6).
- **Seleção (G-07):** `user-select:none` global permanece (app desktop), mas
  `.allow-select` é obrigatório em: corpo do inspetor (Z-C4), editor de célula,
  thread do Chat, editor da Peça, trechos de citação, tabela de taxonomia.
  `::selection` azul contido.

---

## 5. Motion: propositado · rápido · físico · acessível

Tokens: durações 50/100/200/300/400ms + easings standard/decelerate (entrando)/
accelerate (saindo)/linear (só loops). Coreografia: stagger 30–50ms liderado pelo
importante, teto de 500ms por sequência; LOD do canvas = troca **instantânea**
(perf > suavidade — 60 FPS é lei); inserção de item na Captura = fade+deslize
200ms decelerate (origem visível, F-01); expansão D-24 = 400ms decelerate a partir
do nó; saída de overlays = 200ms accelerate. **Nada quica** (sem spring: ferramenta
séria). `prefers-reduced-motion`: durações → 1ms no `:root` + kill de transitions/
animations — exceto spinners/progresso, que informam estado (skill `motion-system`).
**Não anima:** posição de nós (P6), ordem de lista (estabilidade espacial), selos.

---

## 6. Ícones: aposentadoria do emoji (G-08)

Grade 24, traço 2px, round caps/joins, `fill:none`, `stroke:currentColor` (precedente:
SVG do `Dropzone`); utilitário `.icon` + `.icon-sm/.icon-md`; tamanhos 12/16/20/24/32.
Todo ícone tem `aria-hidden` ou rótulo; ação crítica = ícone + texto.

| Emoji atual (arquivo) | Ícone nomeado | Uso |
|---|---|---|
| 📥 (`App`) | `download-tray` | Ingestão/captura |
| 🔍+ / 🔍− (controles) | `zoom-in` / `zoom-out` | câmera |
| 🎯 (controles) | `locate-fixed` | reset de câmera |
| 📁 (lista) | `folder` | categoria |
| ⚠️ (lista) | `alert-triangle` | revisão/falha (sempre + rótulo) |
| — (novos) | `plus, x, check, chevron-down/right, command, sparkles` (só IA/Mesa+Chat), `monitor` (Local), `cloud` (nuvem), `bell, gear, history, quote, arrow-right, refresh-cw, trash-2, pen, search` | cobertura W-00–W-X01 |

Contribuição: traço 2, cantos round, testar em 16px (menor uso real); versionar junto.

---

## 7. Densidade P7 (regra por superfície, sem mágica)

| Superfície | Nível 1 (varredura) | Nível 2 (padrão) | Nível 3 (foco) | Troca |
|---|---|---|---|---|
| Captura Z-C3 | linha: dot+título+selo+data | cartão: +snippet 2linhas+tags | inspetor Z-C4 | toggle manual (default cartão) |
| Mesa Z-M2 | proxy sólido (<0.3) | título+contorno (0.3–0.6) | completo (>0.6) | zoom (lei FR-012) |
| Chat Z-H2 | bolha + citações `[1]` | trecho expansível | fonte integral (→Captura/Mesa) | clique progressivo |

---

## 8. Spec dos componentes-base (anatomia · estados · tokens · a11y)

Formato compacto; implementação na Fase 5+. Todos usam tokens deste doc; nenhum hex
novo fora de `tokens.css`.
1. **Botão** (primary/ghost/danger; sm/md): label 13px/600; primary = blue sólido;
   ghost = surface+borda; danger = rose-outline; disabled 50% + `not-allowed`;
   foco visível §4; altura mín. 28px.
2. **Selo de estado**: dot 6px + rótulo caixa-alta 11px/tracking-wide + bg 0.15
   (review 0.22, active 0.10); 6 estados §2; texto nunca suprimido; selo violet usa
   `--state-ai-fg-strong` (não `--accent-purple`).
3. **Chip/tag + categoria**: tag = cyan-tint + `#`; categoria = surface + ícone folder;
   `+N` p/ overflow; clique filtra (Z-C1).
4. **Cartão-linha (3 densidades §7)**; selecionado = borda blue + tint 0.08 (precedente
   v0.1 mantido); arrastável anuncia destino.
5. **Toast**: barra sev 3px + título 13/600 + corpo 12 + [Ação][×]; progresso com barra
   linear; `polite` (erros `assertive`); empilha máx. 4, excedente vai à central.
6. **Sino + central**: badge = warning+error+progress+quiet; central = lista sessão
   (máx. 50), filtro Todas/Não lidas, "marcar lidas".
7. **Segmentado** (escopo Chat, densidade lista): 2–4 opções, ativo com fundo+barra.
8. **Pílula de workspace**: título + dot de atividade; overflow → N-01 (fixar + "todas").
9. **Overlay de célula**: título 14/700 + textarea markdown 13/1.5 + rodapé
   (hint + Concluir); borda blue 2px + `glow-focus`; Esc descarta (G-06).
10. **Palette**: input 14 + grupos fixos (§6-ia) + linha de escopo/provedor/reversível
    em item de IA + rodapé de confirmação 2-passos; `dialog` com foco preso.
11. **Folha da Peça (D-24)**: header breadcrumb + chips proveniência + 2 colunas
    (blocos | copiloto); não-modal; Esc colapsa sem perder rascunho.
12. **Painel de histórico (D-25)**: borda direita, grupos por sessão, entrada =
    ícone+descrição+hora+selo reversível + "Desfazer até aqui" (B-03 p/ trilha audit).
13. **Tabela de taxonomia**: termo + frequência + barra + [Fundir][Renomear]; % SC-003
    no cabeçalho.
14. **Vazio / skeleton / progresso**: vazio = título + gesto seguinte + atalho
    (X-05); skeleton = blocos shimmer linear (único linear permitido além de
    progresso); progresso de modelo = barra + % + badge SHA ✓/✗.
15. **Badge/chip de provedor**: `[Local✓]` emerald / `[Nuvem]` amber + streaming =
    "tokens saindo da máquina" (P1); Captura = chip estático "100% local".
16. **Chip de proveniência**: `⧉ célula-origem` + trecho hover; clique focaliza fonte
    (Mesa) ou abre inspetor (Captura); base do fork (CONST II).
17. **Faixa DatabaseError (P3)**: "Índice local falhou; arquivos intactos.
    [Reconstruir índice]" — promete porque a arquitetura garante (CONST I).

---

## 9. Regras finais: erros, N-03, fontes, adoção

- **Erros (§7.3-ia):** mensagem PT + causa curta + UMA ação + destino; nunca dump,
  nunca código nu; `SandboxEscape` sóbrio + link auditoria.
- **N-03 (fechado):** remoção externa de arquivo = toast com Desfazer; desfazer
  restaura da lixeira do SO se existir, senão oferece reimportar; semântica passa a
  constar do manual, não de tooltip.
- **Fontes:** 4 arquivos em `src/assets/fonts` (Inter latin/latin-ext wght 100–900;
  JetBrains Mono latin/latin-ext wght 400–800); unicode-ranges exatos do Google;
  `index.html` sem CDN; CSP sem `fonts.googleapis.com`/`gstatic.com` (G-14 fechado;
  SC-005 agora auditável de verdade).
- **Adoção (migração na implementação, fora desta rodada):** trocar em
  `IngestItemList` (classes badge-* → `--state-*`; review→amber), `App`+controles+lista
  (emoji → §6), `CellOverlay` (glow → `--glow-focus`), busca (debounce G-05),
  `user-select` onde §4 manda; **não tocar** em lógica de stores/serviços/IPC.

---

## 10. Handoff Fase 5 + conformidade

**Fase 5 deve:** auditar `tokens.css` × specs (paridade var), checar wireframes ×
princípios (tabela P→evidência), validar contraste das combinações usadas nos frames,
revisar com o responsável e congelar D-01–D-25 + P1–P8 como baseline visual.
**Conformidade desta fase:** cobre todo o handoff §10-wireframes (2 registros,
espaçamento, semântica por zona, elevação, motion+reduced-motion, foco, SVG, select,
specs dos 17 componentes, N-03); D-08 intacto (dark único, zero palette light);
D-09 intacto (zero componente codado — só `tokens.css`, `index.html`, CSP e fontes,
todos infra de token/asset); P1–P8 impõem ao menos uma regra cada (§1 assinatura
local … §8 reversibilidade visível). Handoff §10-wireframes: todos os itens têm
secção correspondente (tipografia §3 · espaçamento §4 · semântica §2 · elevação §4 ·
motion §5 · foco §4 · ícones §6 · select §4 · specs §8 · N-03 §9).
