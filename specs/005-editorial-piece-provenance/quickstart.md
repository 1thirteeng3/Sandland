# Quickstart: Validação da Peça Editorial e Proveniência Canônica

**Feature**: `005-editorial-piece-provenance`  
**Date**: 2026-09-24  

---

## Cenário 1: Criação e Redação de Peça Editorial

**Objetivo**: Validar a criação de um documento longo e persistência atômica em `workspaces/<id>/pieces/<piece_id>.md`.

1. Inicie a aplicação com `npm run dev` (ou abra `http://localhost:1420`).
2. No menu superior ou lateral do workspace, clique no ícone de **Peça Editorial** (ou atalho de alternância de modo).
3. Clique em **"Nova Peça"** e atribua o título `"Síntese da Arquitetura Local-First"`.
4. Digite alguns parágrafos introdutórios em Markdown.
5. Inspecione o sistema de arquivos local no diretório do cofre:
   - Caminho: `workspaces/default-workspace/pieces/sintese-da-arquitetura-local-first.md`.
   - Confirme que o arquivo foi criado com YAML frontmatter contendo `id`, `title`, `slug`, `word_count` e `citations: []`.

---

## Cenário 2: Fork-on-Insert da Mesa para a Peça

**Objetivo**: Validar a transferência autônoma de cartões com gravação de metadados de citação.

1. Ative o modo **Split-View** (ícone de divisão de tela lado a lado: Mesa à esquerda, Peça à direita).
2. Localize o cartão `"Arquitetura Local-First"` na Mesa Espacial.
3. Arraste o cartão para dentro do corpo de texto da Peça.
4. **Resultado Esperado**:
   - O conteúdo do cartão é inserido na posição solta com um badge visual de citação (`[Fonte: Arquitetura Local-First]`).
   - O arquivo físico da Peça no disco atualiza seu frontmatter contendo a entrada em `citations` com `source_cell_id`, `source_revision: 1`, `quote` e `quote_hash`.

---

## Cenário 3: Detecção de Divergência (*Drift Detection*)

**Objetivo**: Comprovar a imunidade da Peça a mutações retroativas e o alerta de divergência de evidências.

1. Na Mesa Espacial (lado esquerdo), dê duplo clique no cartão citado `"Arquitetura Local-First"`.
2. Altere o texto adicionando: `"Adição de nova descoberta analítica..."`.
3. Saia do editor (tecle `Esc` ou clique fora) para salvar a nova versão da célula no disco.
4. Observe o bloco citado na Peça Editorial:
   - **O texto na Peça NÃO se altera silenciosamente** (mantém o trecho original citado).
   - O badge de proveniência passa de verde (sincronizado) para **âmbar** com indicador de *"Evidência Atualizada no Canvas"*.
5. Clique no badge:
   - Uma gaveta comparativa exibe a versão citada vs. a versão atual do cartão na Mesa.
   - O usuário pode optar por *"Atualizar Citação"* ou *"Manter Versão Histórica"*.

---

## Cenário 4: Exportação Limpa com Referências

**Objetivo**: Validar a exportação compilada com apêndice de proveniência.

1. Na barra de ferramentas da Peça Editorial, clique em **"Exportar"**.
2. Selecione a opção **"Markdown com Apêndice de Referências"**.
3. **Resultado Esperado**:
   - É gerado um documento Markdown puro contendo o texto e, ao final, a seção:
     ```markdown
     ## Referências & Proveniência
     1. **Arquitetura Local-First** (Célula `cell-arch`, Revisão 1) - Inserido em 24/09/2026.
     ```
