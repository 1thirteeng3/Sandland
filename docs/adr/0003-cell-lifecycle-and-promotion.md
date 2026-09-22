# ADR 0003: Ciclo de Vida de Células e Política de Promoção Explícita a Notas

**Data:** 2026-09-21  
**Status:** Aceito  
**Contexto:** Sistema de Arquivos, Workspace Canvas e Prevenção de Poluição de Disco

---

## Contexto e Problema
Aplicações de anotação baseadas em sistema de arquivos local frequentemente enfrentam o problema da **poluição de micro-arquivos**:
- Ao criar post-its, caixas de texto temporárias ou ideias voláteis em um canvas, a criação automática de um arquivo `.md` físico para cada nó resulta em milhares de arquivos órfãos com nomes crípticos ou incompletos na raiz do cofre.
- Isso degrada a performance do sistema de arquivos, congestiona o File Watcher e polui a busca global do usuário com rascunhos sem relevância permanente.

## Decisão Arquitetural
Decidimos que **células do canvas não geram arquivos `.md` físicos no disco automaticamente**. O ciclo de vida é segregado em dois estágios:

1. **Estágio 1 — Nó Estruturado (Canvas-Only):**
   - As células criadas na mesa são persistidas exclusivamente como nós serializados no arquivo `topology.json` daquele workspace específico.
   - Podem ser criadas, movidas, editadas e excluídas em tempo real sem impacto no sistema de arquivos do cofre global.
2. **Estágio 2 — Promoção a Nota Canônica (File-as-Truth):**
   - Apenas quando o usuário decide que uma ideia tem valor permanente, ele aciona o comando explícito **"Promover Célula a Nota"**.
   - O backend cria então o arquivo físico `ingest/notes/<slug>-<uuid>.md` com YAML Frontmatter canônico e indexa o documento no SQLite `index.db` do Acervo.
   - O nó no `topology.json` passa a armazenar o `sourceItemId` da nota promovida, mantendo o vínculo bidirecional.

## Consequências
### Positivas
- **Cofre Limpo e Legível:** O diretório de notas físicas contém apenas conhecimento intencionalmente consolidado pelo usuário.
- **Performance:** Reduz drasticamente eventos de I/O em disco e carga de reindexação do file watcher durante sessões intensas de brainstorming no canvas.
- **Integridade:** Elimina arquivos zumbis deixados para trás por células apagadas rapidamente no whiteboard.

### Negativas / Desafios
- Células que não foram promovidas não aparecem na busca global de notas do Acervo (aparecendo apenas dentro do escopo do seu respectivo workspace via `topology.json`).
