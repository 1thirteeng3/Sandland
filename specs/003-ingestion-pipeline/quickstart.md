# Quickstart: Validação do Pipeline de Ingestão e Processamento

**Feature**: `003-ingestion-pipeline`  
**Date**: 2026-09-22

## Cenário 1: Ingestão de Arquivo Markdown com Persistência em Disco

### Pré-requisitos
- Cofre aberto ou inicializado em `<vault_path>`.

### Passos de Teste
1. No painel lateral "Acervo & Ingestão", arrastar um arquivo `nota-teste.md` com o seguinte conteúdo:
   ```markdown
   ---
   title: Nota de Teste Local
   tags: [pesquisa, tauri]
   ---
   # Introdução
   Este é um documento de validação do pipeline físico de ingestão.
   ```
2. Verificar que o arquivo é gravado no disco em `<vault_path>/ingest/notes/nota-teste.md`.
3. Verificar que o item aparece no topo da lista do Acervo com título "Nota de Teste Local" e tags `[pesquisa, tauri]`.
4. Consultar o banco SQLite `<vault_path>/.system/index.db`:
   ```sql
   SELECT id, title, canonical_uri, word_count FROM ingested_items WHERE title = 'Nota de Teste Local';
   ```

---

## Cenário 2: Promoção para a Mesa Espacial (Fork-on-Insert)

### Passos de Teste
1. Na lista de itens do Acervo, clicar no botão de adicionar ou arrastar a nota para o Canvas.
2. Um novo nó retangular aparece no Canvas com o conteúdo da nota.
3. Editar o texto dentro do nó na Mesa.
4. Abrir o arquivo físico original em `<vault_path>/ingest/notes/nota-teste.md` e confirmar que o texto original permaneceu intacto e inalterado.
5. Inspecionar `<vault_path>/workspaces/default-workspace/topology.json` e verificar que o novo nó possui o campo `sourceUri: "ingest/notes/nota-teste.md"`.

---

## Cenário 3: Validação de Blindagem SSRF em Captura de URL

### Passos de Teste
1. Tentar submeter a URL `http://127.0.0.1:8080/admin` ou `http://localhost/secret`.
2. Verificar que o sistema rejeita imediatamente com erro amigável de segurança ("URLs de loopback ou rede privada não são permitidas").
3. Submeter uma URL pública (ex: `https://example.com`).
4. Verificar que o snapshot em Markdown é salvo em `<vault_path>/ingest/web/`.
