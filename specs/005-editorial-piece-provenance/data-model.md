# Data Model: Peça Editorial e Proveniência Canônica

**Feature**: `005-editorial-piece-provenance`  
**Date**: 2026-09-24  
**Status**: Approved  

---

## Entities & Schemas

### 1. EditorialPiece (`workspaces/<workspace_id>/pieces/<piece_id>.md`)

Representação canônica do documento longo de síntese autoral, estruturado com Frontmatter YAML e corpo em Markdown.

| Campo | Tipo | Descrição | Regras de Validação |
| :--- | :--- | :--- | :--- |
| `id` | `String` | Identificador único da Peça (ex: `piece-artigo-1`) | Prefixo `piece-`, único por workspace |
| `workspaceId` | `String` | Workspace ao qual a Peça pertence | Alfanumérico e hífens |
| `title` | `String` | Título do documento | Não vazio, máximo 255 caracteres |
| `slug` | `String` | Nome do arquivo sanitizado no disco (sem extensão) | `^[a-z0-9_-]+$`, máx 64 caracteres |
| `citations` | `Vec<CitationRecord>` | Lista de citações de evidências oriundas da Mesa | Vetor ordenado por inserção |
| `wordCount` | `u32` | Contagem de palavras do corpo do texto | Valor não negativo |
| `createdAt` | `i64` | Timestamp Unix de criação | Timestamp válido |
| `updatedAt` | `i64` | Timestamp Unix da última gravação | Maior ou igual a `createdAt` |
| `body` | `String` | Conteúdo Markdown da Peça | Texto UTF-8 |

```markdown
---
id: "piece-artigo-1"
workspace_id: "default-workspace"
title: "Arquitetura e Soberania Local"
slug: "artigo-1"
word_count: 420
created_at: 1727180000
updated_at: 1727180400
citations:
  - id: "cit-101"
    source_cell_id: "cell-welcome"
    source_revision: 1
    source_title: "Bem-vindo ao Sandland"
    source_asset_hash: null
    quote: "Dê um duplo clique nesta célula para editar..."
    quote_hash: "a1b2c3d4e5f6..."
    inserted_at: 1727180200
---

# Arquitetura e Soberania Local

Este documento sintetiza os princípios basilares do ambiente analítico Sandland.

<cite id="cit-101">Dê um duplo clique nesta célula para editar...</cite>

A soberania do dado garante autonomia e independência duradoura.
```

---

### 2. CitationRecord

Estrutura canônica de metadado que confere rastreabilidade de proveniência a um trecho inserido via Fork-on-Insert.

| Campo | Tipo | Descrição | Validação |
| :--- | :--- | :--- | :--- |
| `id` | `String` | Identificador único da citação na Peça | Prefixo `cit-` |
| `sourceCellId` | `String` | ID da célula de origem na Mesa Espacial | Deve coincidir com arquivo `cells/<id>.md` |
| `sourceRevision` | `u64` | Número da revisão da célula no momento da citação | Valor inteiro positivo $\ge 1$ |
| `sourceTitle` | `String` | Título da célula de origem no momento da citação | Máximo 255 caracteres |
| `sourceAssetHash` | `Option<String>` | Hash SHA-256 do ativo se a citação contiver mídia CAS | 64 caracteres hexadecimais ou nulo |
| `quote` | `String` | Snapshot exato do texto ou trecho citado | Texto não vazio |
| `quoteHash` | `String` | Hash SHA-256 do trecho citado para integridade | 64 caracteres hexadecimais |
| `insertedAt` | `i64` | Timestamp Unix do momento do Fork-on-Insert | Timestamp positivo |

---

### 3. CitationDrift (Estado Dinâmico de Coerência)

Enumeração calculada em tempo de execução ao carregar a Peça, comparando a citação com o arquivo físico da célula em disco.

| Estado | Condição | Significado Visual |
| :--- | :--- | :--- |
| `Synchronized` | `current_cell.revision == citation.source_revision` | Verde: O cartão na Mesa não foi alterado desde a citação |
| `Diverged` | `current_cell.revision > citation.source_revision` | Âmbar: O cartão na Mesa foi editado posteriormente (*Drift*) |
| `Orphaned` | Arquivo `cells/<source_cell_id>.md` não encontrado | Cinza: O cartão na Mesa foi excluído, mas a citação é preservada |

---

### 4. PieceSummaryDTO

Representação resumida utilizada para popular menus de navegação rápida e a barra lateral de Peças.

```json
{
  "id": "piece-artigo-1",
  "workspaceId": "default-workspace",
  "title": "Arquitetura e Soberania Local",
  "slug": "artigo-1",
  "citationCount": 3,
  "wordCount": 420,
  "updatedAt": 1727180400
}
```

---

## State Transitions (Ciclo de Vida Editorial)

```
[Mesa Espacial (Nó na GPU/PixiJS)]
           │
           │ Arrastar cartão para o editor da Peça (Fork-on-Insert)
           ▼
[Clonagem Autônoma de Conteúdo]
           │
           ├─► Insere texto formatado no cursor da Peça
           ├─► Registra CitationRecord no YAML Frontmatter
           ▼
[Peça em Edição Ativa] ─── (Autosave em buffer com debounce de 400ms)
           │
           │ Gravação atômica em disco (FlushFileBuffers / fsync)
           ▼
[Arquivo Físico Atualizado] (workspaces/<id>/pieces/<piece_id>.md)
           │
           │ Modificação posterior do cartão na Mesa
           ▼
[Detecção de Divergência (Drift)]
           │
     ┌─────┴──────────────────┐
     ▼                        ▼
[Manter Citação Histórica]  [Atualizar Citação Conscientemente]
(preserva quote original)   (sincroniza com nova revisão da célula)
```

---

## Invariantes & Regras de Validação

1. **Precedência Canônica**: O arquivo físico `.md` é a única fonte da verdade da Peça.
2. **Imunidade a Mutações Retroativas**: A Peça NUNCA modifica o texto de suas citações sem ação deliberada do usuário.
3. **Persistência Atômica com Fsync**: Todo salvamento de Peça utiliza escrita em arquivo temporário com `FlushFileBuffers` / `fsync` seguido de substituição atômica no SO (Princípio VII).
4. **Isolamento de Diretório**: Peças residem estritamente em `workspaces/<workspace_id>/pieces/`, com verificação pelo `VaultGuard` contra tentativas de Directory Traversal.
