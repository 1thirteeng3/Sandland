---
version: alpha
name: Sandland
description: Base de conhecimento local-first; governa a superfície Captura mais o shell global, em dark único, para mockups e implementação.
colors:
  bg-primary: '#0d0f12'
  bg-secondary: '#16191f'
  bg-tertiary: '#1e222b'
  bg-surface: '#252b37'
  border-subtle: '#2d3442'
  border-strong: '#3d4659'
  border-focus: '#5271ff'
  text-primary: '#f0f3f8'
  text-secondary: '#9da7b8'
  text-muted: '#758092'
  accent-blue: '#3b82f6'
  accent-cyan: '#06b6d4'
  accent-purple: '#8b5cf6'
  accent-amber: '#f59e0b'
  accent-emerald: '#10b981'
  accent-rose: '#f43f5e'
  state-pending-fg: '#f59e0b'
  state-pending-bg: 'rgba(245, 158, 11, 0.15)'
  state-active-fg: '#3b82f6'
  state-active-bg: 'rgba(59, 130, 246, 0.10)'
  state-ai-fg-strong: '#a78bfa'
  state-ai-bg: 'rgba(139, 92, 246, 0.15)'
  state-ok-fg: '#10b981'
  state-ok-bg: 'rgba(16, 185, 129, 0.15)'
  state-review-fg: '#f59e0b'
  state-review-bg: 'rgba(245, 158, 11, 0.22)'
  state-failed-fg: '#f43f5e'
  state-failed-bg: 'rgba(244, 63, 94, 0.15)'
  zone-capture: '#06b6d4'
  zone-mesa: '#3b82f6'
  zone-chat: '#8b5cf6'
  provider-local: '#10b981'
  provider-cloud: '#f59e0b'
typography:
  ui-body:
    fontFamily: Inter
    fontSize: 13px
    lineHeight: 19.5px
    fontWeight: 400
  card-title:
    fontFamily: Inter
    fontSize: 14px
    lineHeight: 20px
    fontWeight: 600
  section-title:
    fontFamily: Inter
    fontSize: 15px
    lineHeight: 22px
    fontWeight: 600
  panel-title:
    fontFamily: Inter
    fontSize: 20px
    lineHeight: 28px
    fontWeight: 600
    letterSpacing: -0.01em
  seal-label:
    fontFamily: Inter
    fontSize: 11px
    lineHeight: 14px
    fontWeight: 600
    letterSpacing: 0.05em
  caption:
    fontFamily: Inter
    fontSize: 12px
    lineHeight: 16px
    fontWeight: 400
  meta-mono:
    fontFamily: JetBrains Mono
    fontSize: 11px
    lineHeight: 14px
    fontWeight: 500
    letterSpacing: 0.02em
  data-mono:
    fontFamily: JetBrains Mono
    fontSize: 12px
    lineHeight: 16px
    fontWeight: 400
rounded:
  sm: 4px
  md: 8px
  lg: 12px
  full: 9999px
spacing:
  space-1: 4px
  space-2: 8px
  space-3: 12px
  space-4: 16px
  space-6: 24px
  space-8: 32px
  space-12: 48px
  space-16: 64px
---

## Overview

O Sandland é uma base de conhecimento local-first operada por analistas; este documento governa a superfície Captura mais o shell global, num dark único e matte, e vale para mockups e para a implementação.

## Colors

Superfícies sobem por planos tonais: fundo {colors.bg-primary}, painéis {colors.bg-secondary}, cartões {colors.bg-tertiary}; toda borda de conteúdo tem 1px em {colors.border-subtle}. Texto usa a tríade {colors.text-primary} para leitura, {colors.text-secondary} para metadados importantes e {colors.text-muted} só para timestamps e hints. Cada estado do pipeline combina dot mais rótulo em caixa-alta mais tint de fundo: ok em {colors.state-ok-fg} sobre {colors.state-ok-bg}, revisão em {colors.state-review-fg} sobre {colors.state-review-bg}, falha em {colors.state-failed-fg} sobre {colors.state-failed-bg}. Texto sobre tint violeta usa {colors.state-ai-fg-strong}, nunca o roxo base. A Captura assina-se em {colors.zone-capture}; o provedor declara-se em {colors.provider-local} para local e {colors.provider-cloud} para nuvem. Nenhuma matiz fora desta lista chega à superfície.

## Typography

Interface e leitura usam Inter; verdades de sistema como timestamps, hash, paths e contagens usam JetBrains Mono. Corpo de UI usa {typography.ui-body}, títulos de cartão {typography.card-title} e títulos de secção {typography.section-title}. Selos e rótulos de estado usam {typography.seal-label}, captions {typography.caption} e metadados mono {typography.meta-mono}. Caixa-alta só com tracking largo e peso a partir de 600; UI densa nunca abaixo de 11px.

## Layout

Largura mínima de 960px; a Captura divide-se em 7/12 mais 5/12 com gap de {spacing.space-4}. Todo cartão do acervo termina no rodapé canónico com categoria e tags à esquerda e estado à direita. O banner de utilidade ocupa uma linha com nome da zona, estado do índice e contador vivo de egresso. Padding de cartão é 12px, de painel é 16px e gap de lista é 8px, sempre da escala.

## Elevation & Depth

A profundidade usa a escala de elevação do sistema: repouso no nível mínimo, controlos e cartões elevados um nível acima, overlays e toasts no topo; sombras fora da escala são proibidas. Sobreposição fora da ordem canvas, painéis, shell e toasts é bug.

## Shapes

Componentes base usam {rounded.sm}, painéis e dropzones {rounded.md}; selos, chips, tags e badges de provedor são pill em {rounded.full}. Hairlines têm sempre 1px; sem bordas duplas.

## Components

Ação primária leva fundo {colors.accent-blue} com texto {colors.text-primary} em peso 600 e altura mínima de 28px; ghost usa superfície com hairline; danger usa contorno e texto em {colors.state-failed-fg} com confirmação em dois passos; desabilitado fica a 50% sem hover. Selo de estado combina dot de 6px com rótulo em {typography.seal-label} e nunca suprime o texto. Badge de provedor é pill mono com dot na cor do modo. Tag taxonómica carrega o prefixo # em tint cyan e a categoria usa superfície com ícone de pasta; clicar filtra o acervo. Controlo segmentado oferece 2 a 4 opções com a ativa em fundo elevado, sem glifos de texto como marcadores.

## Do's and Don'ts

Anuncie erros críticos com aria-live assertive e use polite nos toasts informativos. Devolva o foco ao fechar overlays com Esc. Não existe modo claro. Sem emoji na UI, só ícones SVG de traço 2px com cantos round. Sem jargão técnico visível: RRF, Skeleton, ingest, vec e IDs internos nunca na UI. Sem backdrop-blur em painéis e cartões de conteúdo; blur só em drawer, controlos e overlays. Sem spring ou bounce; transições entre 100ms e 400ms com desaceleração na entrada e aceleração na saída. Nunca só cor para comunicar estado. Nada repete sozinho: falha pede ação explícita com uma saída. Máximo de dois selos visíveis em simultâneo.
