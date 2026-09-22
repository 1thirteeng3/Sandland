# Sandland — Roadmap granular e plano de implementação até a v2.0

**Data de elaboração:** 22/09/2026  
**Modelo de execução escolhido:** uma pessoa responsável + agentes de IA  
**Cadência escolhida:** sprints encerrados por critérios de aceite, sem duração fixa  
**Horizonte escolhido:** produto completo na v1.0, maturação e evolução até a v2.0  
**Natureza:** projeto open source conduzido sem fins comerciais  
**Estado inicial:** planejamento; nenhuma tarefa de implementação está declarada concluída

> **Dimensão do plano:** 72 sprints, 720 tarefas detalhadas, 72 gates de aceite e 19 marcos de versão. São 792 itens rastreáveis no backlog. Esses números representam unidades de trabalho, não semanas, dias ou uma previsão de prazo.

## Como usar os arquivos

- **Este documento:** sequência completa, entregas por versão, dependências, passos, artefatos e critérios por tarefa.
- **[Sandland-Backlog.xlsx](Sandland-Backlog.xlsx):** acompanhamento com filtros, status, revisores, evidências, dependências, requisitos, ADRs e riscos.
- **[Sandland-Backlog.csv](Sandland-Backlog.csv):** exportação plana UTF-8 para adaptar ao rastreador de issues de preferência; não exige um serviço específico.
- **[Dados estruturados do plano](planejamento/Sandland-Roadmap-dados.json):** representação versionável usada para manter os artefatos coerentes.

**Base:** RFC/PRD enviado pelo autor e [plano de integração/desconstrução](Sandland-RFC-Integracao-e-Desconstrucao.md). O documento original permanece intacto. As fontes e licenças técnicas já examinadas estão no plano complementar; cada checkout será revalidado antes de incorporar código.

**Importante:** caminhos como `crates/vault/` ou `reports/v0.8/` nos itens são **artefatos a produzir no futuro repositório do app**, não arquivos já implementados nesta entrega. A validação executada para este roadmap verifica IDs, dependências, cobertura e consistência dos documentos — não compila os motores nem comprova segurança/performance do Sandland.

---

## A. Regras de execução e limites do horizonte

### A.1 O que significa “sprint” aqui

Você escolheu encerramento por aceite. Portanto, “sprint” é um pacote de objetivo/resultado, **não um timebox Scrum de uma ou duas semanas**. Sprints variam de esforço: um recorte de SDK, um mecanismo de sandbox e uma publicação não têm a mesma duração.

A ordem S01→S72 é a rota recomendada para um executor principal. Dependências técnicas são explicitadas separadamente. Dentro de cada sprint, as tarefas seguem uma sequência conservadora de implementação; subdividir uma tarefa complexa em PRs/subtarefas é permitido sem alterar seus critérios.

Não multiplicar quantidade de sprints por duas semanas para obter prazo. Depois de executar os primeiros ciclos, usar throughput real, bloqueios e acesso a hardware para estimativas revisáveis.

### A.2 Fronteira da v1.0 e da v2.0

- **v1.0:** cobre o escopo aprovado de Captura, Mesa, Peça, IA, busca, web, intenções, histórico, portabilidade e cinco alvos de desktop. Recursos opcionais podem exigir pacote/modelo e rede autorizada, mas não podem ser substituídos por promessas.
- **v1.1:** confiabilidade, recuperação e manutenção decorrentes do uso real.
- **v1.2 e v2.0:** escala/interoperabilidade ampliadas e extensibilidade controlada são **propostas de evolução**, ratificadas em ADR-017/018. Não se apresentam como decisões já tomadas pelo autor.
- **v2.0 é o fim deste horizonte**, não a última versão que o projeto poderá ter. Correções, manutenção e evolução continuam depois.

Não escopo automático: SaaS obrigatório, colaboração multiusuário em tempo real, sincronização CRDT entre dispositivos, marketplace de código irrestrito, treinamento de um modelo próprio ou port de todos os motores para Rust. A presença de Yjs, LocalAI ou MCP não autoriza introduzir esses produtos paralelos.

### A.3 Decisões que bloqueiam implementação

1. **Core Rust + workers poliglotas:** precisa substituir formalmente a exigência original de Rust estrito. Se rejeitada, o plano deve ser refeito para ports/reimplementação.
2. **Fonte da verdade:** conteúdo/topologia/histórico em formatos abertos; bancos, MPK e stores de motores são derivados.
3. **Jev:** integração nativa quando autorizada; oferta pública é API, não pesos que se instalam no LocalAI. Offline exige fallback distinto ou classificação pendente.
4. **Busca:** prototipar Qdrant e HelixDB sob o mesmo contrato e **promover somente um** inicialmente.
5. **Distribuição:** obter capacidade de validar Linux x86_64/aarch64, Windows x86_64, macOS Apple Silicon/Intel. São cinco alvos; suporte de GPU/modelos varia por perfil.
6. **Licença:** ratificar licença open source e obrigações upstream. Atividade não comercial do projeto não significa proibição de uso comercial por terceiros nem isenção de termos de APIs/modelos.

Os 18 ADRs do plano são decisões registradas, não fatos já aprovados. Não avançar por suposição quando uma escolha altera soberania, segurança, persistência ou licença.

### A.4 Produto útil antes de todos os motores

- v0.1: Vault confiável e recuperável;
- v0.2: notas locais com BlockSuite;
- v0.3: Mesa espacial;
- v0.4: Peça, proveniência e ciclo autoral completo sem IA;
- versões seguintes acrescentam motores especializados sem mudar a autoridade dos arquivos.

Isso dá ao mantenedor um produto utilizável para testar o próprio trabalho, sem esperar até a v1.0 para descobrir problemas de edição e persistência.

---

## B. Método de desconstrução dos projetos-base

**Desconstruir não significa necessariamente copiar diretórios ou criar um fork de cada engine.** É identificar quais responsabilidades pertencem ao upstream, quais precisam de adaptação e quais permanecem próprias.

### B.1 Receita aplicada a cada motor

1. **Fixar origem:** tag/commit, licença, build, submódulos, modelos e dependências.
2. **Reproduzir baseline upstream:** executar fixture controlada antes de modificar código.
3. **Mapear dependências e acoplamentos:** localizar UI, cloud, stores, redes e frameworks não desejados.
4. **Definir contrato Sandland:** entradas, saídas, erros, capacidades, escopo e versão.
5. **Criar adapter/facade:** tentar interface pública antes de depender de internals.
6. **Extrair apenas o inevitável:** manter patch set pequeno e rastreável; não reimplementar índices ou kernels sem evidência de necessidade.
7. **Substituir autoridade externa:** direcionar dados, modelos e permissões ao Core; nunca criar duas fontes da verdade.
8. **Testar diferencialmente:** comparar com baseline e validar casos específicos do Sandland, inclusive falhas.
9. **Empacotar e confinar:** lifecycle, quotas, logs, licença e testes por sistema operacional.
10. **Integrar e manter:** feature gate só abre após evidência; atualização upstream exige os mesmos contratos e rollback.

**Jev é a exceção importante:** desconstruímos o fluxo de decisão e integramos o serviço; não existe neste plano extração do modelo hospedado. **Qdrant/HelixDB e LocalAI:** integramos motores através de contratos, não transplantamos seus algoritmos internos de storage/inferência.

### B.2 Trilha por componente

| Motor | Sprints | Integração | Reaproveitar | Excluir | Responsabilidade própria |
| --- | --- | --- | --- | --- | --- |
| BlockSuite | S12–S15; S18; S20–S21; S63; S70 | Bibliotecas + facade; patch set mínimo | store/std/rich-text, modelos/blocos necessários; confirmar grafo workspace na revisão fixada | UI AFFiNE, cloud e segundo canvas | Codec Markdown, IDs, salvamento, overlay PixiJS e proveniência |
| PageIndex | S42–S46; S63; S70 | Recorte/fork delimitado + portas de modelo/dados | page_index_md, navegação/resumos/tools úteis; confirmar módulos reais no checkout | DocStore como autoridade, páginas de PDF artificiais, acesso global e cloud implícita | Snapshot estrutural, IDs estáveis, relações cruzadas, budgets, invalidação e ToolBroker |
| Qdrant | S37–S38; S40–S41 se escolhido | Spike; depois adapter sem extrair o storage interno | Edge ou Server conforme capacidades testadas | Segundo banco permanente se não for escolhido; UI/cluster sem necessidade | SearchProvider, fusão/escopo, revisões e reconstrução |
| HelixDB | S37; S39; S40–S41 se escolhido | Spike; depois adapter embutido/local conforme evidência | Engine/SDK e consultas requeridas de texto, vetores e relações derivadas | Fonte da verdade e object storage remoto obrigatório | SearchProvider, projeções da topologia e ciclo de índices |
| Jev | S34–S35; S52; S63 | Adapter de serviço + contrato de decisões | Choice, Score e Noul da API, com versão registrada | Tentativa de extrair pesos fechados, executar Jev no LocalAI ou gerar texto livre | Taxonomia, candidatos, calibração, consentimento, fallback e revisão |
| Docling | S27–S28; S30; S63; S70 | Worker Python empacotado; adapter mínimo | Parsing textual, tabelas, OCR aprovado e mapas de fonte | Servidor sempre residente, todos os modelos e acesso amplo ao Vault | NormalizedDocument, jobs, persistência, limites e reprocessamento |
| Readability / Turndown | S29–S30; S48; S63; S70 | Bibliotecas JS em ambiente DOM isolado | Extração de artigo e conversão HTML para Markdown | Scripts/trackers remotos e janela principal como parser hostil | Sanitização, assets, origem, workers e normalização canônica |
| SearxNG | S47; S50; S63; S70 | Serviço local/pessoal configurado + adapter | Metabusca e API JSON dos engines escolhidos | Frontend próprio, instância pública implícita, alegação de busca offline | Consulta/egress, UI, histórico, seleção, lifecycle e distribuição AGPL |
| Scrapling | S48–S50; S63; S70 | Worker HTTP e browser separado sob demanda | Fetchers, parser/sessões e aquisição dinâmica quando aprovada | Crawl irrestrito, um browser por widget e bypass universal prometido | Política SSRF/rede, snapshots, pool, limits e promoção à Mesa |
| LocalAI | S31–S32; S36; S63; S65; S70 | Runtime gerenciado, não reescrita de kernels | APIs e backends homologados, carga/descarga e mecanismos de recursos | Agentes, shell, memória/RAG paralelos e downloads automáticos arbitrários | Catálogo, gateway, consentimento, pacotes, supervisor e diagnóstico |
| PixiJS / Tauri / SQLite | S04–S11; S16–S19; S54–S57 | Infraestrutura reutilizada com domínio próprio | Janela/IPC, renderização e projeções SQL | Autoridade de dados nas caches e permissões implícitas | Vault, board, interação, conflitos, política de segurança e lifecycle |

### B.3 O que continua sendo desenvolvimento próprio

- VaultStore, journal/revisões, CAS, reconciliação e conflitos;
- domínio de células/ocorrências, grupos, relações, Peças e intenções;
- ponte Markdown↔BlockSuite e overlay DOM↔PixiJS;
- semântica de fork-on-insert e cadeia de proveniência;
- projeção da topologia para PageIndex, expansão relacional, budgets e invalidação;
- taxonomia, candidatos, políticas de aceitação/abstenção e revisão humana;
- orquestração de ingest/jobs, validação e promoção dos resultados;
- autorização de filesystem, ferramentas, rede e segredos;
- interface de pesquisa/copiloto, lifecycle global, migrações, pacotes e suporte.

Reutilização reduz a construção de motores fundamentais; não terceiriza as invariantes do produto.

---

## C. Trabalho solo com agentes de IA

### C.1 Papéis, não pessoas extras

| Papel | Responsabilidade | Limite |
|---|---|---|
| Autor/mantenedor | Decisões, produto, revisão, licença, segurança e aceite final | Único responsável humano; não presumir equipe de QA inexistente |
| Agente de implementação | Tarefa estreita em branch/escopo delimitado | Não altera contrato ou dependência fora do combinado |
| Agente de testes | Fixtures, casos adversariais e execução permitida | Não inventa resultado nem usa dados/chaves sem autorização |
| Agente revisor | Ler diff, procurar regressões e confrontar critérios | Outra saída de IA não substitui evidência nem aprovação humana |
| CI/harness | Verificar contratos e comportamentos repetíveis | Resultado só vale para commit, ambiente e casos efetivamente executados |

**WIP recomendado:** uma alteração de produto principal em andamento; no máximo uma frente auxiliar de testes/documentação sobre interfaces estáveis. Não paralelizar vários agentes escrevendo os mesmos codecs, schemas ou políticas de autorização.

### C.2 Ciclo de uma tarefa

1. Ler ID, objetivo, contrato, dependências e não escopo.
2. Abrir branch/worktree com apenas os arquivos necessários.
3. Criar fixture/teste negativo antes ou junto à implementação.
4. Implementar menor mudança que satisfaz o contrato.
5. Executar testes; registrar comando, commit, ambiente e resultado bruto.
6. Solicitar revisão adversarial, corrigir e repetir testes relevantes.
7. Autor verifica evidência, origem/licença e efeito no produto.
8. Registrar merge, revisão humana e evidência; somente então marcar concluída.

### C.3 Pacote para delegar a um agente

```text
Tarefa: <SL-Sxx-yy>
Objetivo e não escopo: <copiar do plano + limite da mudança>
Contexto autorizado: <schemas, arquivos e baseline relevantes>
Dependências satisfeitas: <IDs + evidências>
Arquivos que pode alterar: <lista explícita>
Artefato esperado: <caminho do plano>
Critério de aceite: <texto literal, sem reinterpretar para facilitar>
Testes/fixtures: <caso positivo + negativo + regressão>
Permissões de rede/segredos: <normalmente nenhuma>
Condições de parada: contrato contraditório, licença incerta, bypass de sandbox,
  migração destrutiva, API inexistente, falta de runner/hardware.
Resposta final: diff, testes realmente executados, pendências e riscos.
Proibido: declarar teste verde sem log; atualizar latest; usar chave real sem consentimento;
  substituir motor ou reduzir requisito por conta própria.
```

---

## D. Gates, evidências e acompanhamento

### D.1 Definition of Ready

Uma tarefa está pronta quando suas dependências foram aceitas, os contratos necessários estão definidos, existe fixture/teste verificável, o escopo de arquivos está delimitado e há ambiente suficiente. Falta de acesso a um Mac/Windows ou de artefato licenciado é bloqueio real, não detalhe a ignorar.

### D.2 Definition of Done universal

- artefato produzido e revisado;
- critérios particulares e regressões pertinentes executados;
- entradas inválidas/falhas relevantes tratadas;
- origem de código/dependências e licenças preservadas;
- sem bypass de VaultStore, ToolBroker ou política de rede;
- sem conteúdo autoral perdido ao apagar caches;
- evidência vinculada a commit/ambiente;
- revisão humana e data de aceite registradas;
- documentação/rollback atualizados quando o contrato mudou.

O gate de sprint agrega as dez tarefas, a demonstração específica e os bloqueios abertos. Não é só “build passou”. Em spike, uma conclusão negativa **com protocolo completo** pode fechar a investigação; ela não aprova o motor rejeitado para produção.

### D.3 Estrutura das evidências

```text
reports/<sprint>/<task-id>.md
  commit / versões / ambiente / hardware
  objetivo / fixture / critério
  comandos e logs ou links para runs
  resultado: passou | falhou | bloqueado | não executado
  dados brutos e comparação, quando houver
  limitações / risco residual
  revisão humana / data
reports/<sprint>/acceptance.md
  dez tarefas verificadas + demonstração + go/no-go
```

Os caminhos são **esperados**, não comprovantes já existentes. A coluna “Evidência obtida” permanece vazia até execução. CI offline usa fakes; testes reais Jev/BYOK/web são separados, consentidos e potencialmente pagos. Mocks não validam qualidade do modelo nem disponibilidade do serviço.

### D.4 Status e regras de dispensa

`Não iniciado → Pronto → Em andamento → Em revisão → Concluído`, com `Bloqueado` a qualquer momento. `Dispensado (ADR)` só se aplica a um item realmente condicional, com mudança aprovada e efeito no escopo documentado. Não serve para dispensar perda de dados, privacidade, segurança ou requisito essencial da v1.0.

A planilha calcula fechamento somente com status apropriado, revisor, data e evidência, e respeita dependências registradas. Isso auxilia acompanhamento, **não prova que uma evidência preenchida é verdadeira**. O mantenedor continua responsável pela verificação.

Ao mudar escopo, inserir subtarefas ou reordenar, atualizar dados e dependências — não só a célula de porcentagem. As versões são acumulativas. Um requisito essencial não pode ser adiado a v2 apenas para chamar v1 de completo.

### D.5 Regras de recursos e suporte

- Repouso a frio e retorno a idle após unload são cenários distintos; ambos precisam ser definidos.
- Contabilizar processos filhos, WebView e recursos pertinentes; não medir só o Core.
- Os budgets originais continuam metas/gates a homologar, não resultados já observados.
- OCR/transcrição pesada não recebe exceção automática aos tetos; decidir perfil ou indisponibilidade de forma explícita.
- Build cruzado não valida execução nativa; GPU não tem paridade presumida entre os cinco alvos.
- Hotfix de integridade/segurança interrompe qualquer sprint. Não esperar um marco futuro para corrigir vulnerabilidade conhecida.

---

## E. Mapa de versões e entregas

| Versão | Sprints | Entrega | Maturidade | Resultado verificável |
| --- | --- | --- | --- | --- |
| v0.0.1 | S01–S03 | Arquitetura executável | Interna | Decisões, contratos de dados e inventário de origem aprovados; nenhuma funcionalidade de produto prometida. |
| v0.0.2 | S04–S06 | Bancada reproduzível | Interna | Repositório, CI, fixtures e shell desktop compiláveis sem importar aplicações completas. |
| v0.1.0 | S07–S11 | Vault recuperável | Alpha técnica | Criar, alterar, recuperar e reconstruir conhecimento local com ferramenta de diagnóstico. |
| v0.2.0 | S12–S15 | Editor local BlockSuite | Alpha de autoria | Editar notas no perfil Markdown suportado e reabrir sem depender do estado interno do editor. |
| v0.3.0 | S16–S19 | Mesa espacial | Alpha de autoria | Organizar células, grupos e relações com PixiJS e edição BlockSuite integrada. |
| v0.4.0 | S20–S22 | Peça e proveniência | Alpha funcional sem IA | Produzir texto editorial a partir da Mesa com fork-on-insert, referências e exportação. |
| v0.5.0 | S23–S30 | Ingestão segura | Alpha de acervo | Capturar arquivos/HTML, extrair texto e OCR em workers confinados e organizar o acervo. |
| v0.6.0 | S31–S36 | Enriquecimento e modelos | Alpha assistida | LocalAI gerenciado, resumos, Jev autorizado, fallback local e transcrição homologada. |
| v0.7.0 | S37–S41 | Busca híbrida | Alpha integrada | Escolher Qdrant OU HelixDB e recuperar materiais/Peças com índices descartáveis e revisões válidas. |
| v0.8.0 | S42–S46 | Contexto estrutural | Alpha de raciocínio | PageIndex adaptado às células, Skeleton Map e recuperação relacional limitada por escopo e orçamento. |
| v0.9.0 | S47–S53 | Ciclo analítico completo | Alpha de ciclo completo | Pesquisa web, widget, copiloto, chat e intenções conectam Captura → Mesa → Peça. |
| v1.0.0-beta.1 | S54–S57 | Homologação multiplataforma | Beta | Segurança, performance e pacotes validados nos cinco alvos, sem presumir paridade de aceleração. |
| v1.0.0-rc.1 | S58–S59 | Candidata à estabilidade | RC | Migrações, backup/Git, documentação, acessibilidade e instalação limpa avaliados. |
| v1.0.0 | S60 | Escopo integral estável | Estável | Todos os requisitos do escopo aprovado, evidências e condições de distribuição satisfeitos. |
| v1.1.0 | S61–S63 | Confiabilidade operacional | Estável — maturação | Correções do uso real, recuperação assistida e manutenção upstream sem perda de dados. |
| v1.2.0 | S64–S66 | Escala e portabilidade | Estável — evolução proposta | Acervos maiores, pacotes offline e interoperabilidade ampliada, mediante ratificação de escopo. |
| v2.0.0-beta.1 | S67–S70 | Extensibilidade controlada | Beta — evolução proposta | Contratos de provedores, kit de conformidade e instalação de extensões com permissões explícitas. |
| v2.0.0-rc.1 | S71 | Compatibilidade consolidada | RC | Migração de v1.x, rollback e matriz completa de regressão de motores/formatos. |
| v2.0.0 | S72 | Plataforma estável e sustentável | Estável — fim do horizonte | Base madura, interoperável e extensível; continuidade de manutenção, não fim definitivo do projeto. |

**Sem datas de conclusão:** o plano é ordenado por dependências e aceite. Beta/RC têm gates próprios, em vez de nomes promocionais para o mesmo artefato.

## F. Navegação dos sprints

- **v0.0.1:** [S01](#s01) · [S02](#s02) · [S03](#s03)
- **v0.0.2:** [S04](#s04) · [S05](#s05) · [S06](#s06)
- **v0.1.0:** [S07](#s07) · [S08](#s08) · [S09](#s09) · [S10](#s10) · [S11](#s11)
- **v0.2.0:** [S12](#s12) · [S13](#s13) · [S14](#s14) · [S15](#s15)
- **v0.3.0:** [S16](#s16) · [S17](#s17) · [S18](#s18) · [S19](#s19)
- **v0.4.0:** [S20](#s20) · [S21](#s21) · [S22](#s22)
- **v0.5.0:** [S23](#s23) · [S24](#s24) · [S25](#s25) · [S26](#s26) · [S27](#s27) · [S28](#s28) · [S29](#s29) · [S30](#s30)
- **v0.6.0:** [S31](#s31) · [S32](#s32) · [S33](#s33) · [S34](#s34) · [S35](#s35) · [S36](#s36)
- **v0.7.0:** [S37](#s37) · [S38](#s38) · [S39](#s39) · [S40](#s40) · [S41](#s41)
- **v0.8.0:** [S42](#s42) · [S43](#s43) · [S44](#s44) · [S45](#s45) · [S46](#s46)
- **v0.9.0:** [S47](#s47) · [S48](#s48) · [S49](#s49) · [S50](#s50) · [S51](#s51) · [S52](#s52) · [S53](#s53)
- **v1.0.0-beta.1:** [S54](#s54) · [S55](#s55) · [S56](#s56) · [S57](#s57)
- **v1.0.0-rc.1:** [S58](#s58) · [S59](#s59)
- **v1.0.0:** [S60](#s60)
- **v1.1.0:** [S61](#s61) · [S62](#s62) · [S63](#s63)
- **v1.2.0:** [S64](#s64) · [S65](#s65) · [S66](#s66)
- **v2.0.0-beta.1:** [S67](#s67) · [S68](#s68) · [S69](#s69) · [S70](#s70)
- **v2.0.0-rc.1:** [S71](#s71)
- **v2.0.0:** [S72](#s72)

---

## G. Plano operacional sprint a sprint

Cada item abaixo tem ID estável, passo de execução, artefato e aceite. “Dependências” são referências de trabalho, não bibliotecas de runtime. `SL-Sxx-GATE` exige conclusão/evidência das tarefas do sprint; o backlog inclui esses gates como linhas separadas.


# v0.0.1 — Arquitetura executável

**Maturidade:** Interna  
**Entrega acumulada:** Decisões, contratos de dados e inventário de origem aprovados; nenhuma funcionalidade de produto prometida.

<a id="s01"></a>

## S01 — Ratificar escopo, soberania e composição

- **Objetivo:** Eliminar decisões contraditórias antes de criar dependências de implementação.
- **Componente / forma:** Arquitetura / produto / Próprio.
- **Pré-requisitos técnicos:** Nenhum sprint anterior; requer acesso aos documentos de base.
- **Entregável do sprint:** Escopo v1.0 e registro de decisões iniciais.
- **Demonstração exigida:** Autor consegue explicar offline, Jev, workers e cinco alvos sem contradições.
- **Requisitos:** R01, R03, R07, R08, R33, R34. **Risco de integração:** Crítico.

### SL-S01-01 — Congelar documentos de entrada

- **Executar:** Copiar RFC e plano de integração para referências; registrar hashes e divergências.
- **Produzir:** `docs/baseline/manifest.json`.
- **Aceitar quando:** Os dois documentos permanecem íntegros e cada divergência aponta à seção de origem.
- **Controle:** Decisão; P0 na versão; depende de Ready de S01; evidência em `reports/S01/SL-S01-01.md`.

### SL-S01-02 — Delimitar v1.0

- **Executar:** Enumerar jornadas Captura→Mesa→Peça e separar requisitos de propostas pós-v1.0.
- **Produzir:** `docs/product/scope-v1.md`.
- **Aceitar quando:** Nenhuma funcionalidade original é silenciosamente adiada para v2; exclusões estão explícitas.
- **Controle:** Decisão; P0 na versão; depende de SL-S01-01; evidência em `reports/S01/SL-S01-02.md`.

### SL-S01-03 — Ratificar núcleo e workers

- **Executar:** Resolver ADR-001; distinguir linguagem do Core, runtime e modelo de distribuição.
- **Produzir:** `docs/adr/001-composicao.md`.
- **Aceitar quando:** Decisão aprovada pelo autor; rejeição de workers bloqueia o restante e exige novo plano.
- **Controle:** Decisão; P0 na versão; depende de SL-S01-02; evidência em `reports/S01/SL-S01-03.md`.

### SL-S01-04 — Ratificar Jev/offline

- **Executar:** Escolher fallback local ou pendência; definir quando a API pode receber conteúdo.
- **Produzir:** `docs/adr/004-egress-offline.md`.
- **Aceitar quando:** Falta de chave/rede não bloqueia captura, leitura ou autoria; nenhuma chamada implícita permitida.
- **Controle:** Decisão; P0 na versão; depende de SL-S01-03; evidência em `reports/S01/SL-S01-04.md`.

### SL-S01-05 — Escolher host de referência

- **Executar:** Registrar máquina principal e acesso de teste aos cinco alvos previstos.
- **Produzir:** `docs/adr/006-targets.md`.
- **Aceitar quando:** Plataforma de desenvolvimento escolhida e lacunas de hardware identificadas, sem alegar homologação.
- **Controle:** Decisão; P0 na versão; depende de SL-S01-04; evidência em `reports/S01/SL-S01-05.md`.

### SL-S01-06 — Fixar semântica de versões

- **Executar:** Separar versão do app, formato do Vault, contratos de workers e modelos.
- **Produzir:** `docs/product/version-policy.md`.
- **Aceitar quando:** Atualizar o app não implica migrar o Vault; regras pre-1.0 e compatibilidade estão claras.
- **Controle:** Decisão; P0 na versão; depende de SL-S01-05; evidência em `reports/S01/SL-S01-06.md`.

### SL-S01-07 — Definir dados sensíveis

- **Executar:** Classificar conteúdo, consultas, metadados, chaves e logs; negar envio por padrão.
- **Produzir:** `docs/security/data-classification.md`.
- **Aceitar quando:** Casos de resumo e título sensível também exigem política de saída, não apenas documentos completos.
- **Controle:** Decisão; P0 na versão; depende de SL-S01-06; evidência em `reports/S01/SL-S01-07.md`.

### SL-S01-08 — Mapear atores e ameaças

- **Executar:** Incluir arquivo malformado, página hostil, extensão, modelo e processo local malicioso.
- **Produzir:** `docs/security/threat-model.md`.
- **Aceitar quando:** Cada ator tem ativos, fronteiras e ataques de teste; modelo não aparece como autoridade confiável.
- **Controle:** Decisão; P0 na versão; depende de SL-S01-07; evidência em `reports/S01/SL-S01-08.md`.

### SL-S01-09 — Registrar não escopo

- **Executar:** Excluir SaaS obrigatório, colaboração em tempo real e marketplace irrestrito deste horizonte.
- **Produzir:** `docs/product/non-goals.md`.
- **Aceitar quando:** Funcionalidades não aprovadas não entram por implicação da presença de Yjs, LocalAI ou MCP.
- **Controle:** Decisão; P0 na versão; depende de SL-S01-08; evidência em `reports/S01/SL-S01-09.md`.

### SL-S01-10 — Aprovar inventário de requisitos

- **Executar:** Numerar invariantes, métricas e critérios; designar o autor como aprovador dos gates.
- **Produzir:** `docs/product/requirements.yaml`.
- **Aceitar quando:** Cada requisito tem origem, critério e responsável; ambiguidades não são tratadas como concluídas.
- **Controle:** Decisão; P0 na versão; depende de SL-S01-09; evidência em `reports/S01/SL-S01-10.md`.

### SL-S01-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Autor consegue explicar offline, Jev, workers e cinco alvos sem contradições.**

Registrar commit, ambiente, testes e pendências em `reports/S01/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s02"></a>

## S02 — Definir formatos, identidade e autoridade

- **Objetivo:** Especificar representação persistente antes do editor e dos índices.
- **Componente / forma:** Vault / contratos / Próprio.
- **Pré-requisitos técnicos:** S01 (Ratificar escopo, soberania e composição)
- **Entregável do sprint:** Schemas v0 do Vault, board, eventos, células e evidências.
- **Demonstração exigida:** Fixtures independentes dos motores representam uma mesa e uma Peça completas.
- **Requisitos:** R02, R04, R05, R11, R13, R32, R37. **Risco de integração:** Crítico.

### SL-S02-01 — Definir entidade e ocorrência

- **Executar:** Separar fonte, conteúdo da célula/nota, ocorrência visual, grupo, aresta, Peça e bloco.
- **Produzir:** `schemas/domain-v0.json`.
- **Aceitar quando:** A mesma nota em dois boards tem conteúdo único e duas geometrias independentes.
- **Controle:** Contrato; P0 na versão; depende de SL-S01-GATE; evidência em `reports/S02/SL-S02-01.md`.

### SL-S02-02 — Definir IDs estáveis

- **Executar:** Fixar geração, namespace e regras de colisão sem depender do nome do arquivo.
- **Produzir:** `schemas/identity-v0.json`.
- **Aceitar quando:** Renomear/mover não altera identidade; duplicar como nova autoria recebe novo ID.
- **Controle:** Contrato; P0 na versão; depende de SL-S02-01; evidência em `reports/S02/SL-S02-02.md`.

### SL-S02-03 — Definir autoridade de arquivos

- **Executar:** Formalizar JSON+histórico e MPK derivado; resolver ADR-002.
- **Produzir:** `docs/adr/002-file-truth.md`.
- **Aceitar quando:** Recuperação tem precedência determinística e não escolhe fonte por timestamp arbitrário.
- **Controle:** Decisão; P0 na versão; depende de SL-S02-02; evidência em `reports/S02/SL-S02-03.md`.

### SL-S02-04 — Especificar perfil Markdown

- **Executar:** Listar blocos suportados e representação de frontmatter, IDs, tabelas e citações.
- **Produzir:** `docs/adr/005-markdown-profile.md`.
- **Aceitar quando:** Cada bloco possui round-trip previsto; recurso não representável é preservado ou recusado, não achatado.
- **Controle:** Contrato; P0 na versão; depende de SL-S02-03; evidência em `reports/S02/SL-S02-04.md`.

### SL-S02-05 — Especificar topologia

- **Executar:** Registrar posições, dimensões, grupos, conexões, rótulos e referências de conteúdo.
- **Produzir:** `schemas/board-v0.json`.
- **Aceitar quando:** Ciclos e arestas entre grupos são representáveis sem duplicar conteúdo.
- **Controle:** Contrato; P0 na versão; depende de SL-S02-04; evidência em `reports/S02/SL-S02-05.md`.

### SL-S02-06 — Especificar revisões e eventos

- **Executar:** Definir base_revision, operação, autor, origem IA e marcador de confirmação.
- **Produzir:** `schemas/event-v0.json`.
- **Aceitar quando:** Evento não confirmado é distinguível de estado salvo; replay e conflitos têm regras explícitas.
- **Controle:** Contrato; P0 na versão; depende de SL-S02-05; evidência em `reports/S02/SL-S02-06.md`.

### SL-S02-07 — Especificar evidência

- **Executar:** Combinar source_id, revisão, trecho, localizador, hash e bloco de destino.
- **Produzir:** `schemas/evidence-v0.json`.
- **Aceitar quando:** Referência distingue texto citado de comentário e não se torna inválida por renomeação.
- **Controle:** Contrato; P0 na versão; depende de SL-S02-06; evidência em `reports/S02/SL-S02-07.md`.

### SL-S02-08 — Especificar metadados de IA

- **Executar:** Separar resumo gerado/editado, tags sugeridas/aceitas e identificação do pipeline.
- **Produzir:** `schemas/enrichment-v0.json`.
- **Aceitar quando:** Reprocessamento não sobrescreve correção humana; cobertura parcial pode ser declarada.
- **Controle:** Contrato; P0 na versão; depende de SL-S02-07; evidência em `reports/S02/SL-S02-08.md`.

### SL-S02-09 — Construir fixtures válidas e inválidas

- **Executar:** Criar um Vault mínimo, um completo e casos com campos desconhecidos.
- **Produzir:** `tests/fixtures/schema/`.
- **Aceitar quando:** Validadores aceitam extensões permitidas e rejeitam IDs duplicados, tipos errados e referências impossíveis.
- **Controle:** Teste; P0 na versão; depende de SL-S02-08; evidência em `reports/S02/SL-S02-09.md`.

### SL-S02-10 — Definir evolução de schema

- **Executar:** Documentar migração, versão desconhecida, abertura somente leitura e exportação.
- **Produzir:** `docs/contracts/schema-evolution.md`.
- **Aceitar quando:** Um app antigo nunca regrava silenciosamente um formato novo que não compreende.
- **Controle:** Decisão; P0 na versão; depende de SL-S02-09; evidência em `reports/S02/SL-S02-10.md`.

### SL-S02-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Fixtures independentes dos motores representam uma mesa e uma Peça completas.**

Registrar commit, ambiente, testes e pendências em `reports/S02/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s03"></a>

## S03 — Inventariar upstreams e definir recortes

- **Objetivo:** Fixar origem, licenças e modo de integração sem criar forks indiscriminados.
- **Componente / forma:** Todos os motores / licenças / Desconstrução.
- **Pré-requisitos técnicos:** S01 (Ratificar escopo, soberania e composição); S02 (Definir formatos, identidade e autoridade)
- **Entregável do sprint:** Catálogo de componentes e política de manutenção upstream.
- **Demonstração exigida:** Cada motor tem origem rastreável, fronteira proposta e decisão de manter/extrair/adaptar.
- **Requisitos:** R03, R30, R33, R34. **Risco de integração:** Crítico.

### SL-S03-01 — Inventariar repositórios

- **Executar:** Registrar URL, licença observada, linguagens, releases e componentes transitivos críticos.
- **Produzir:** `upstream/catalog.yaml`.
- **Aceitar quando:** BlockSuite, PageIndex, Docling, Jev, Qdrant, HelixDB, Scrapling, SearxNG e LocalAI estão cobertos.
- **Controle:** Upstream; P0 na versão; depende de SL-S01-GATE, SL-S02-GATE; evidência em `reports/S03/SL-S03-01.md`.

### SL-S03-02 — Criar manifesto de origem

- **Executar:** Definir campos commit/tag, hashes, patches, avisos e comando de build.
- **Produzir:** `upstream/manifest.schema.json`.
- **Aceitar quando:** Nenhum componente empacotado pode usar latest ou branch móvel sem resolução para uma revisão.
- **Controle:** Upstream; P0 na versão; depende de SL-S03-01; evidência em `reports/S03/SL-S03-02.md`.

### SL-S03-03 — Examinar BlockSuite por árvore

- **Executar:** Distinguir standalone de AFFiNE e fechar o inventário dos arquivos candidatos.
- **Produzir:** `docs/legal/blocksuite-scope.md`.
- **Aceitar quando:** Licença não é inferida apenas pelo nome do projeto ou por um único package.json.
- **Controle:** Licença; P0 na versão; depende de SL-S03-02; evidência em `reports/S03/SL-S03-03.md`.

### SL-S03-04 — Examinar SearxNG e modelos

- **Executar:** Separar código AGPL, integração por API e termos dos pesos/backends.
- **Produzir:** `docs/legal/component-obligations.md`.
- **Aceitar quando:** Fonte correspondente, avisos e redistribuição estão mapeados por artefato.
- **Controle:** Licença; P0 na versão; depende de SL-S03-03; evidência em `reports/S03/SL-S03-04.md`.

### SL-S03-05 — Ratificar licença própria

- **Executar:** Resolver ADR-003; explicar operação não comercial versus direitos open source de terceiros.
- **Produzir:** `docs/adr/003-license.md`.
- **Aceitar quando:** Licença aprovada sem cláusula non-commercial incompatível disfarçada de open source.
- **Controle:** Decisão; P0 na versão; depende de SL-S03-04; evidência em `reports/S03/SL-S03-05.md`.

### SL-S03-06 — Delimitar reutilização

- **Executar:** Marcar bibliotecas, adapters, forks mínimos e serviços; excluir UIs completas e stores concorrentes.
- **Produzir:** `docs/architecture/reuse-map.md`.
- **Aceitar quando:** Cada motor tem lista de incluir/excluir e interfaces Sandland responsáveis pela autoridade.
- **Controle:** Upstream; P0 na versão; depende de SL-S03-05; evidência em `reports/S03/SL-S03-06.md`.

### SL-S03-07 — Definir política de patches

- **Executar:** Criar série pequena e reprodutível; separar correções genéricas de customizações.
- **Produzir:** `upstream/PATCH_POLICY.md`.
- **Aceitar quando:** Um fork pode ser rebaseado e sua diferença para upstream pode ser enumerada.
- **Controle:** Upstream; P0 na versão; depende de SL-S03-06; evidência em `reports/S03/SL-S03-07.md`.

### SL-S03-08 — Preparar avisos e SBOM

- **Executar:** Criar estrutura de LICENSES, NOTICE e inventário de dependências.
- **Produzir:** `compliance/`.
- **Aceitar quando:** Não há licença de terceiros substituída pela licença própria sem base legal.
- **Controle:** Governança; P0 na versão; depende de SL-S03-07; evidência em `reports/S03/SL-S03-08.md`.

### SL-S03-09 — Definir ciclo de atualização

- **Executar:** Fixar auditoria, testes diferenciais, rollback e prioridade para vulnerabilidades.
- **Produzir:** `docs/maintenance/upstream.md`.
- **Aceitar quando:** Atualização sem evidência não chega automaticamente ao usuário ou altera modelos instalados.
- **Controle:** Governança; P0 na versão; depende de SL-S03-08; evidência em `reports/S03/SL-S03-09.md`.

### SL-S03-10 — Definir limites de spikes

- **Executar:** Para cada prova, listar pergunta, corpus, evidência mínima e decisão de continuar/replanejar.
- **Produzir:** `docs/spikes/register.md`.
- **Aceitar quando:** Relatório inconclusivo não vira integração aprovada; Qdrant e Helix têm protocolo comparável.
- **Controle:** Decisão; P0 na versão; depende de SL-S03-09; evidência em `reports/S03/SL-S03-10.md`.

### SL-S03-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Cada motor tem origem rastreável, fronteira proposta e decisão de manter/extrair/adaptar.**

Registrar commit, ambiente, testes e pendências em `reports/S03/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

# v0.0.2 — Bancada reproduzível

**Maturidade:** Interna  
**Entrega acumulada:** Repositório, CI, fixtures e shell desktop compiláveis sem importar aplicações completas.

<a id="s04"></a>

## S04 — Montar repositório e fluxo solo com IA

- **Objetivo:** Tornar alterações pequenas, revisáveis e reproduzíveis por um único mantenedor.
- **Componente / forma:** Ferramentas / governança / Próprio.
- **Pré-requisitos técnicos:** S03 (Inventariar upstreams e definir recortes)
- **Entregável do sprint:** Monorepo de trabalho e instruções de contribuição/agentes.
- **Demonstração exigida:** Um agente implementa uma tarefa isolada e o autor consegue revisar o patch e seus testes.
- **Requisitos:** R03, R30, R33, R34. **Risco de integração:** Crítico.

### SL-S04-01 — Criar estrutura do monorepo

- **Executar:** Separar app desktop, crates, adapters, workers, schemas, fixtures e docs.
- **Produzir:** `repository-layout.md`.
- **Aceitar quando:** Dependências apontam para contratos e não para UI/estado privado de outro motor.
- **Controle:** Implementação; P0 na versão; depende de SL-S03-GATE; evidência em `reports/S04/SL-S04-01.md`.

### SL-S04-02 — Fixar toolchains

- **Executar:** Pin Rust, JS e runtimes necessários para desenvolvimento, sem instaladores globais implícitos.
- **Produzir:** `toolchains.lock`.
- **Aceitar quando:** Ambiente limpo reproduz versões; mudanças de versão são diff explícito.
- **Controle:** Implementação; P0 na versão; depende de SL-S04-01; evidência em `reports/S04/SL-S04-02.md`.

### SL-S04-03 — Configurar formatadores e lint

- **Executar:** Definir regras por linguagem e exclusões de fontes vendorizadas.
- **Produzir:** `tooling/lint/`.
- **Aceitar quando:** Comando único reporta erros sem reformatar ou apagar avisos de arquivos upstream.
- **Controle:** Implementação; P0 na versão; depende de SL-S04-02; evidência em `reports/S04/SL-S04-03.md`.

### SL-S04-04 — Criar comandos de projeto

- **Executar:** Padronizar bootstrap, check, test, fixtures e pacote de evidências.
- **Produzir:** `tooling/tasks/`.
- **Aceitar quando:** Comandos têm help, retorno de erro confiável e não dependem do histórico do shell.
- **Controle:** Implementação; P0 na versão; depende de SL-S04-03; evidência em `reports/S04/SL-S04-04.md`.

### SL-S04-05 — Criar política de branches

- **Executar:** Uma frente mutável principal; PRs pequenos com IDs de tarefa e rollback.
- **Produzir:** `CONTRIBUTING.md`.
- **Aceitar quando:** Dois agentes não alteram contratos compartilhados em paralelo sem acordo explícito.
- **Controle:** Governança; P0 na versão; depende de SL-S04-04; evidência em `reports/S04/SL-S04-05.md`.

### SL-S04-06 — Escrever instruções de agentes

- **Executar:** Delimitar arquivos, segredos, rede, testes obrigatórios e critérios de parada.
- **Produzir:** `AGENTS.md`.
- **Aceitar quando:** Agente não pode declarar testes executados sem logs nem aprovar a própria entrega.
- **Controle:** Governança; P0 na versão; depende de SL-S04-05; evidência em `reports/S04/SL-S04-06.md`.

### SL-S04-07 — Criar template de tarefa

- **Executar:** Incluir contrato, fixture, não escopo, aceite e evidências.
- **Produzir:** `.github/ISSUE_TEMPLATE/task.yml`.
- **Aceitar quando:** Tarefa pode ser entregue a novo agente sem depender de memória informal da conversa.
- **Controle:** Governança; P0 na versão; depende de SL-S04-06; evidência em `reports/S04/SL-S04-07.md`.

### SL-S04-08 — Criar template de PR

- **Executar:** Exigir origem de código, mudanças de schema, segurança, testes e artefatos.
- **Produzir:** `.github/pull_request_template.md`.
- **Aceitar quando:** PR sem evidências ou com teste pulado tem bloqueio explícito.
- **Controle:** Governança; P0 na versão; depende de SL-S04-07; evidência em `reports/S04/SL-S04-08.md`.

### SL-S04-09 — Separar configuração e segredos

- **Executar:** Fornecer arquivos de exemplo sem credenciais e política de logs.
- **Produzir:** `config/examples/`.
- **Aceitar quando:** Scanner detecta segredos plantados em fixture e não coleta dados reais do desenvolvedor.
- **Controle:** Implementação; P0 na versão; depende de SL-S04-08; evidência em `reports/S04/SL-S04-09.md`.

### SL-S04-10 — Executar ensaio do fluxo

- **Executar:** Fazer alteração descartável em fixture, revisão adversarial e reversão.
- **Produzir:** `reports/workflow-dry-run.md`.
- **Aceitar quando:** Histórico evidencia proposta, teste, revisão humana e rollback, sem merge automático por agente.
- **Controle:** Teste; P0 na versão; depende de SL-S04-09; evidência em `reports/S04/SL-S04-10.md`.

### SL-S04-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Um agente implementa uma tarefa isolada e o autor consegue revisar o patch e seus testes.**

Registrar commit, ambiente, testes e pendências em `reports/S04/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s05"></a>

## S05 — Construir CI, corpus e protocolo de medição

- **Objetivo:** Criar a bancada que impedirá promessas de qualidade ou recursos sem medição.
- **Componente / forma:** Qualidade / desempenho / Próprio.
- **Pré-requisitos técnicos:** S04 (Montar repositório e fluxo solo com IA)
- **Entregável do sprint:** CI inicial, corpus licenciado e perfis mensuráveis.
- **Demonstração exigida:** Falha introduzida em contrato bloqueia pipeline e um benchmark gera dados reproduzíveis.
- **Requisitos:** R01, R09, R12, R15, R20, R34. **Risco de integração:** Alto.

### SL-S05-01 — Montar CI de contratos

- **Executar:** Rodar schemas, lint e testes determinísticos sem chaves de serviços.
- **Produzir:** `.github/workflows/check.yml`.
- **Aceitar quando:** PR com schema inválido falha; CI básica passa sem acesso a Jev/BYOK.
- **Controle:** Implementação; P1 na versão; depende de SL-S04-GATE; evidência em `reports/S05/SL-S05-01.md`.

### SL-S05-02 — Criar matriz de builds

- **Executar:** Incluir os cinco alvos quando suportado pelo ambiente; distinguir build e teste nativo.
- **Produzir:** `ci/target-matrix.yaml`.
- **Aceitar quando:** Painel não mostra target compilado como target funcionalmente homologado.
- **Controle:** Implementação; P1 na versão; depende de SL-S05-01; evidência em `reports/S05/SL-S05-02.md`.

### SL-S05-03 — Montar corpus documental

- **Executar:** Reunir PDFs textuais/escaneados, DOCX, HTML, Markdown e transcrições com direitos de uso.
- **Produzir:** `tests/corpus/manifest.json`.
- **Aceitar quando:** Cada item possui hash, idioma, formato e licença/origem; nenhum dado privado entra no repositório.
- **Controle:** Teste; P1 na versão; depende de SL-S05-02; evidência em `reports/S05/SL-S05-03.md`.

### SL-S05-04 — Montar corpus de busca

- **Executar:** Rotular perguntas, respostas esperadas, ausência de resposta e fontes em português.
- **Produzir:** `tests/eval/retrieval-v1.jsonl`.
- **Aceitar quando:** Conjunto de avaliação é separado dos exemplos usados para ajustar prompts e thresholds.
- **Controle:** Teste; P1 na versão; depende de SL-S05-03; evidência em `reports/S05/SL-S05-04.md`.

### SL-S05-05 — Criar corpus hostil

- **Executar:** Incluir traversal, HTML ativo, injeção de prompt, arquivos truncados e URLs internas.
- **Produzir:** `tests/security/fixtures/`.
- **Aceitar quando:** Cada fixture é inerte fora de harness controlado e tem comportamento negado esperado.
- **Controle:** Teste; P1 na versão; depende de SL-S05-04; evidência em `reports/S05/SL-S05-05.md`.

### SL-S05-06 — Fixar métricas de memória

- **Executar:** Definir RAM/RSS/PSS/VRAM, processos incluídos e diferenças por SO.
- **Produzir:** `docs/adr/007-resource-profiles.md`.
- **Aceitar quando:** Meta de 350 MB não é validada medindo apenas o processo Rust.
- **Controle:** Decisão; P0 na versão; depende de SL-S05-05; evidência em `reports/S05/SL-S05-06.md`.

### SL-S05-07 — Criar harness de desempenho

- **Executar:** Medir startup interativo, ack IPC, latência de busca e frame times.
- **Produzir:** `tooling/bench/`.
- **Aceitar quando:** Dados brutos incluem hardware, versão, dataset, warm/cold e repetição; não apenas médias soltas.
- **Controle:** Implementação; P1 na versão; depende de SL-S05-06; evidência em `reports/S05/SL-S05-07.md`.

### SL-S05-08 — Gerar cenas sintéticas

- **Executar:** Criar boards de 100 e 1.000 nós com textos, imagens, grupos e conexões.
- **Produzir:** `tests/fixtures/boards/`.
- **Aceitar quando:** Cenas são determinísticas e não representam só caixas vazias favoráveis ao benchmark.
- **Controle:** Teste; P1 na versão; depende de SL-S05-07; evidência em `reports/S05/SL-S05-08.md`.

### SL-S05-09 — Padronizar evidências

- **Executar:** Definir JSON de resultados e caminho por tarefa, hash de commit e ambiente.
- **Produzir:** `schemas/evidence-report.json`.
- **Aceitar quando:** Relatório distingue executado, falhou, bloqueado e não executado; campo de aprovação é humano.
- **Controle:** Implementação; P1 na versão; depende de SL-S05-08; evidência em `reports/S05/SL-S05-09.md`.

### SL-S05-10 — Validar a própria bancada

- **Executar:** Injetar regressão de tempo, erro de schema e ausência de runner.
- **Produzir:** `reports/ci-self-test.md`.
- **Aceitar quando:** Cada situação gera falha ou estado bloqueado, nunca resultado verde fictício.
- **Controle:** Teste; P1 na versão; depende de SL-S05-09; evidência em `reports/S05/SL-S05-10.md`.

### SL-S05-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Falha introduzida em contrato bloqueia pipeline e um benchmark gera dados reproduzíveis.**

Registrar commit, ambiente, testes e pendências em `reports/S05/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s06"></a>

## S06 — Criar shell desktop e contratos IPC

- **Objetivo:** Abrir um app local mínimo sem antecipar a integração de todos os motores.
- **Componente / forma:** Tauri / apresentação / Integração.
- **Pré-requisitos técnicos:** S04 (Montar repositório e fluxo solo com IA); S05 (Construir CI, corpus e protocolo de medição)
- **Entregável do sprint:** Shell Tauri com navegação Ingest/Mesa/Peça e IPC validado.
- **Demonstração exigida:** Instalar build de desenvolvimento, navegar e selecionar um diretório sem rede.
- **Requisitos:** R01, R03, R07, R08, R31, R34. **Risco de integração:** Crítico.

### SL-S06-01 — Inicializar Tauri v2

- **Executar:** Montar frontend mínimo e Core; manter motores pesados fora do startup.
- **Produzir:** `apps/desktop/`.
- **Aceitar quando:** Shell abre sem modelos, browsers, servidores de pesquisa ou bibliotecas Python carregados.
- **Controle:** Implementação; P0 na versão; depende de SL-S04-GATE, SL-S05-GATE; evidência em `reports/S06/SL-S06-01.md`.

### SL-S06-02 — Definir contratos IPC

- **Executar:** Versionar request/response, erros, IDs e cancelamento; gerar tipos onde possível.
- **Produzir:** `schemas/ipc-v0.json`.
- **Aceitar quando:** Frontend não chama comandos arbitrários ou APIs nativas por nomes não autorizados.
- **Controle:** Implementação; P0 na versão; depende de SL-S06-01; evidência em `reports/S06/SL-S06-02.md`.

### SL-S06-03 — Configurar capabilities

- **Executar:** Expor apenas comandos do app e aplicar CSP restritiva à apresentação.
- **Produzir:** `apps/desktop/capabilities/`.
- **Aceitar quando:** Script de conteúdo não obtém acesso automático a filesystem, shell ou segredos.
- **Controle:** Implementação; P0 na versão; depende de SL-S06-02; evidência em `reports/S06/SL-S06-03.md`.

### SL-S06-04 — Montar navegação de fases

- **Executar:** Criar estados Ingest, Mesa, Peça e ajustes sem duplicar dados por tela.
- **Produzir:** `apps/desktop/src/shell/`.
- **Aceitar quando:** Alternar fase não perde estado nem exige conta ou serviço remoto.
- **Controle:** Implementação; P0 na versão; depende de SL-S06-03; evidência em `reports/S06/SL-S06-04.md`.

### SL-S06-05 — Implementar seleção de Vault

- **Executar:** Usar diálogo nativo e raiz dinâmica; adiar leitura ao serviço autorizado.
- **Produzir:** `crates/app-core/src/vault_selection.rs`.
- **Aceitar quando:** Paths com espaços/Unicode funcionam e nenhuma raiz de usuário está hardcoded.
- **Controle:** Implementação; P0 na versão; depende de SL-S06-04; evidência em `reports/S06/SL-S06-05.md`.

### SL-S06-06 — Criar estados de tarefa

- **Executar:** Padronizar vazio, carregando, falhou, bloqueado, offline e concluído.
- **Produzir:** `apps/desktop/src/components/status/`.
- **Aceitar quando:** Falha recuperável mostra ação útil e não fica em spinner infinito.
- **Controle:** Implementação; P0 na versão; depende de SL-S06-05; evidência em `reports/S06/SL-S06-06.md`.

### SL-S06-07 — Preparar acessibilidade básica

- **Executar:** Dar nomes, foco e atalhos às regiões principais.
- **Produzir:** `apps/desktop/src/accessibility/`.
- **Aceitar quando:** Percurso essencial por teclado funciona sem mouse; foco não desaparece ao trocar de fase.
- **Controle:** Implementação; P0 na versão; depende de SL-S06-06; evidência em `reports/S06/SL-S06-07.md`.

### SL-S06-08 — Integrar logs locais saneados

- **Executar:** Emitir IDs e tempos sem incluir texto do Vault por padrão.
- **Produzir:** `crates/diagnostics/`.
- **Aceitar quando:** Fixture sensível não aparece em log, erro serializado ou console da UI.
- **Controle:** Implementação; P0 na versão; depende de SL-S06-07; evidência em `reports/S06/SL-S06-08.md`.

### SL-S06-09 — Fazer smoke nos alvos disponíveis

- **Executar:** Abrir/fechar, selecionar pasta e testar IPC em ambientes reais.
- **Produzir:** `reports/shell-targets/`.
- **Aceitar quando:** Ambiente não disponível é marcado bloqueado, com tarefa de acesso antes da homologação.
- **Controle:** Teste; P0 na versão; depende de SL-S06-08; evidência em `reports/S06/SL-S06-09.md`.

### SL-S06-10 — Medir baseline vazio

- **Executar:** Registrar árvore de processos e startup sem funcionalidades opcionais.
- **Produzir:** `reports/baseline-shell.json`.
- **Aceitar quando:** Baseline reproduzível permite atribuir consumo incremental a motores futuros.
- **Controle:** Teste; P0 na versão; depende de SL-S06-09; evidência em `reports/S06/SL-S06-10.md`.

### SL-S06-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Instalar build de desenvolvimento, navegar e selecionar um diretório sem rede.**

Registrar commit, ambiente, testes e pendências em `reports/S06/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

# v0.1.0 — Vault recuperável

**Maturidade:** Alpha técnica  
**Entrega acumulada:** Criar, alterar, recuperar e reconstruir conhecimento local com ferramenta de diagnóstico.

<a id="s07"></a>

## S07 — Implementar Vault, identidade e assets

- **Objetivo:** Criar a primeira autoridade persistente do produto.
- **Componente / forma:** VaultStore / CAS / Próprio.
- **Pré-requisitos técnicos:** S02 (Definir formatos, identidade e autoridade); S06 (Criar shell desktop e contratos IPC)
- **Entregável do sprint:** VaultStore básico, manifest e armazenamento de assets por hash.
- **Demonstração exigida:** Criar Vault, salvar nota/asset, fechar e listar os mesmos IDs ao reabrir.
- **Requisitos:** R02, R04, R05, R07. **Risco de integração:** Crítico.

### SL-S07-01 — Criar lifecycle do Vault

- **Executar:** Abrir, criar, validar versão e recusar raiz inválida sem modificar seus arquivos.
- **Produzir:** `crates/vault/src/lifecycle.rs`.
- **Aceitar quando:** Pasta existente incompatível não é sobrescrita; erro aponta a causa.
- **Controle:** Implementação; P0 na versão; depende de SL-S02-GATE, SL-S06-GATE; evidência em `reports/S07/SL-S07-01.md`.

### SL-S07-02 — Implementar identidade

- **Executar:** Gerar IDs, detectar duplicatas e separar ID de localização atual.
- **Produzir:** `crates/domain/src/identity.rs`.
- **Aceitar quando:** Renomear não muda ID; cópia com ID repetido gera conflito tratável.
- **Controle:** Implementação; P0 na versão; depende de SL-S07-01; evidência em `reports/S07/SL-S07-02.md`.

### SL-S07-03 — Ler/escrever frontmatter

- **Executar:** Preservar campos desconhecidos e tipos permitidos no perfil aprovado.
- **Produzir:** `crates/vault/src/markdown.rs`.
- **Aceitar quando:** Round-trip de fixtures não apaga extensões nem converte datas/strings inadvertidamente.
- **Controle:** Implementação; P0 na versão; depende de SL-S07-02; evidência em `reports/S07/SL-S07-03.md`.

### SL-S07-04 — Implementar CAS

- **Executar:** Calcular hash por streaming, detectar duplicatas e publicar arquivo completo.
- **Produzir:** `crates/vault/src/assets.rs`.
- **Aceitar quando:** Dois imports iguais compartilham bytes; interrupção não cria asset final parcial.
- **Controle:** Implementação; P0 na versão; depende de SL-S07-03; evidência em `reports/S07/SL-S07-04.md`.

### SL-S07-05 — Mapear referências de assets

- **Executar:** Referenciar hash/tipo sem caminhos absolutos dependentes da máquina.
- **Produzir:** `crates/domain/src/asset_ref.rs`.
- **Aceitar quando:** Vault copiado para outra raiz continua resolvendo anexos.
- **Controle:** Implementação; P0 na versão; depende de SL-S07-04; evidência em `reports/S07/SL-S07-05.md`.

### SL-S07-06 — Criar workspace/célula

- **Executar:** Persistir manifest, board vazio e célula Markdown por contratos.
- **Produzir:** `crates/vault/src/workspaces.rs`.
- **Aceitar quando:** Criação incompleta é detectada e reparável sem gerar workspace invisível perdido.
- **Controle:** Implementação; P0 na versão; depende de SL-S07-05; evidência em `reports/S07/SL-S07-06.md`.

### SL-S07-07 — Implementar inventário por varredura

- **Executar:** Ler arquivos canônicos sem exigir banco existente.
- **Produzir:** `crates/vault/src/scan.rs`.
- **Aceitar quando:** Inventário inicial retorna os mesmos IDs após remover índices derivados.
- **Controle:** Implementação; P0 na versão; depende de SL-S07-06; evidência em `reports/S07/SL-S07-07.md`.

### SL-S07-08 — Definir descarte e recuperação

- **Executar:** Criar lixeira/tombstones sem destruir imediatamente a fonte de rollback.
- **Produzir:** `crates/vault/src/deletion.rs`.
- **Aceitar quando:** Excluir uma referência não apaga asset ainda usado por outra célula ou revisão.
- **Controle:** Implementação; P0 na versão; depende de SL-S07-07; evidência em `reports/S07/SL-S07-08.md`.

### SL-S07-09 — Testar portabilidade de paths

- **Executar:** Cobrir Unicode, case sensitivity, nomes reservados e diretórios longos.
- **Produzir:** `tests/vault/path-compat/`.
- **Aceitar quando:** Casos não suportados são recusados com explicação em vez de truncados silenciosamente.
- **Controle:** Teste; P0 na versão; depende de SL-S07-08; evidência em `reports/S07/SL-S07-09.md`.

### SL-S07-10 — Criar ferramenta de inspeção

- **Executar:** Listar invariantes e problemas em modo somente leitura.
- **Produzir:** `tooling/vault-inspect/`.
- **Aceitar quando:** Ferramenta lê o Vault sem iniciar UI, modelos ou serviços externos.
- **Controle:** Teste; P0 na versão; depende de SL-S07-09; evidência em `reports/S07/SL-S07-10.md`.

### SL-S07-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Criar Vault, salvar nota/asset, fechar e listar os mesmos IDs ao reabrir.**

Registrar commit, ambiente, testes e pendências em `reports/S07/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s08"></a>

## S08 — Implementar transações de arquivos e histórico

- **Objetivo:** Tornar confirmação, replay e desfazer independentes de bancos descartáveis.
- **Componente / forma:** Journal / RevisionStore / Próprio.
- **Pré-requisitos técnicos:** S07 (Implementar Vault, identidade e assets)
- **Entregável do sprint:** Journal durável e protocolo de publicação de revisões.
- **Demonstração exigida:** Interromper cada etapa de uma edição e recuperar o último estado confirmado.
- **Requisitos:** R02, R05, R13, R37. **Risco de integração:** Crítico.

### SL-S08-01 — Especificar máquina de estados de commit

- **Executar:** Definir preparação, confirmação, publicação e replay idempotente.
- **Produzir:** `docs/contracts/file-commit.md`.
- **Aceitar quando:** Ordem de flush/rename e estado reconhecido após cada falha estão definidos por SO.
- **Controle:** Contrato; P0 na versão; depende de SL-S07-GATE; evidência em `reports/S08/SL-S08-01.md`.

### SL-S08-02 — Criar escritor único

- **Executar:** Serializar mutações por Vault e validar expected_revision.
- **Produzir:** `crates/vault/src/writer.rs`.
- **Aceitar quando:** Duas edições concorrentes não substituem uma à outra sem conflito explícito.
- **Controle:** Implementação; P0 na versão; depende de SL-S08-01; evidência em `reports/S08/SL-S08-02.md`.

### SL-S08-03 — Persistir objetos de revisão

- **Executar:** Armazenar snapshots/deltas necessários antes de anunciar commit.
- **Produzir:** `crates/history/src/objects.rs`.
- **Aceitar quando:** Toda revisão confirmada possui objetos íntegros para reconstrução.
- **Controle:** Implementação; P0 na versão; depende de SL-S08-02; evidência em `reports/S08/SL-S08-03.md`.

### SL-S08-04 — Implementar journal

- **Executar:** Registrar eventos, integridade de registros e tratamento de cauda truncada.
- **Produzir:** `crates/history/src/journal.rs`.
- **Aceitar quando:** Registro parcial não invalida commits anteriores e não é executado como completo.
- **Controle:** Implementação; P0 na versão; depende de SL-S08-03; evidência em `reports/S08/SL-S08-04.md`.

### SL-S08-05 — Publicar arquivos atômicos

- **Executar:** Usar temporários, flush e troca apropriada; documentar garantias reais de cada SO.
- **Produzir:** `crates/vault/src/atomic_write.rs`.
- **Aceitar quando:** Leitor observa versão anterior ou nova completa; falhas de disco são propagadas.
- **Controle:** Implementação; P0 na versão; depende de SL-S08-04; evidência em `reports/S08/SL-S08-05.md`.

### SL-S08-06 — Implementar replay

- **Executar:** Repetir operações confirmadas sem criar cópias ou efeitos extras.
- **Produzir:** `crates/history/src/replay.rs`.
- **Aceitar quando:** Executar replay duas vezes produz o mesmo estado e contagem de entidades.
- **Controle:** Implementação; P0 na versão; depende de SL-S08-05; evidência em `reports/S08/SL-S08-06.md`.

### SL-S08-07 — Criar undo semântico

- **Executar:** Reverter edição, tag ou operação de board por nova operação registrada.
- **Produzir:** `crates/history/src/undo.rs`.
- **Aceitar quando:** Undo preserva proveniência e não apaga a existência do evento original.
- **Controle:** Implementação; P0 na versão; depende de SL-S08-06; evidência em `reports/S08/SL-S08-07.md`.

### SL-S08-08 — Encadear eventos

- **Executar:** Definir serialização canônica, hashes e verificação do histórico.
- **Produzir:** `crates/history/src/integrity.rs`.
- **Aceitar quando:** Alteração detectada relativamente ao checkpoint confiável; limite de reescrita total documentado.
- **Controle:** Implementação; P0 na versão; depende de SL-S08-07; evidência em `reports/S08/SL-S08-08.md`.

### SL-S08-09 — Injetar falhas de I/O

- **Executar:** Simular disco cheio, permissão, flush/rename interrompidos e erro de objeto ausente.
- **Produzir:** `tests/history/fault-injection/`.
- **Aceitar quando:** App não informa salvo para commit não durável nem remove a única cópia válida.
- **Controle:** Teste; P0 na versão; depende de SL-S08-08; evidência em `reports/S08/SL-S08-09.md`.

### SL-S08-10 — Medir janela de perda

- **Executar:** Instrumentar timestamps de edição/commit e executar encerramentos controlados.
- **Produzir:** `reports/durability-window.json`.
- **Aceitar quando:** Janela de 500 ms é medida; teste de processo não é apresentado como teste de queda de energia.
- **Controle:** Teste; P0 na versão; depende de SL-S08-09; evidência em `reports/S08/SL-S08-10.md`.

### SL-S08-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Interromper cada etapa de uma edição e recuperar o último estado confirmado.**

Registrar commit, ambiente, testes e pendências em `reports/S08/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s09"></a>

## S09 — Implementar broker de arquivos e defesa TOCTOU

- **Objetivo:** Eliminar validação de path separada da abertura antes de integrar workers.
- **Componente / forma:** VaultGuard / ToolBroker / Próprio.
- **Pré-requisitos técnicos:** S07 (Implementar Vault, identidade e assets); S08 (Implementar transações de arquivos e histórico)
- **Entregável do sprint:** I/O autorizado por referências/handles e suíte de ataques de paths.
- **Demonstração exigida:** Solicitar um arquivo permitido e negar escapes durante resolução e concorrência.
- **Requisitos:** R06, R08, R34. **Risco de integração:** Crítico.

### SL-S09-01 — Definir capabilities de I/O

- **Executar:** Vincular ator, workspace, objetos permitidos, operação, expiração e revisão.
- **Produzir:** `schemas/io-capability.json`.
- **Aceitar quando:** Capacidade de leitura não autoriza escrita nem acesso a outro workspace.
- **Controle:** Contrato; P0 na versão; depende de SL-S07-GATE, SL-S08-GATE; evidência em `reports/S09/SL-S09-01.md`.

### SL-S09-02 — Implementar resolução por IDs

- **Executar:** Converter referências em objetos autorizados dentro do Core.
- **Produzir:** `crates/security/src/object_broker.rs`.
- **Aceitar quando:** Texto vindo do modelo não pode se tornar path arbitrário por concatenação.
- **Controle:** Implementação; P0 na versão; depende de SL-S09-01; evidência em `reports/S09/SL-S09-02.md`.

### SL-S09-03 — Implementar abertura Linux

- **Executar:** Usar diretório-base/handle e resolução restrita conforme kernel disponível.
- **Produzir:** `crates/security/src/fs/linux.rs`.
- **Aceitar quando:** Troca de symlink durante acesso não sai do escopo; recurso ausente tem política fail-closed.
- **Controle:** Implementação; P0 na versão; depende de SL-S09-02; evidência em `reports/S09/SL-S09-03.md`.

### SL-S09-04 — Implementar abertura Windows

- **Executar:** Tratar handles, reparse points, junctions e canonicalização por mecanismo efetivo.
- **Produzir:** `crates/security/src/fs/windows.rs`.
- **Aceitar quando:** Junction trocada e path especial não contornam a autorização.
- **Controle:** Implementação; P0 na versão; depende de SL-S09-03; evidência em `reports/S09/SL-S09-04.md`.

### SL-S09-05 — Implementar abertura macOS

- **Executar:** Usar primitivas relativas/handles e política de links consistente com o contrato.
- **Produzir:** `crates/security/src/fs/macos.rs`.
- **Aceitar quando:** Testes nativos demonstram contenção; não se reutiliza apenas canonicalize seguido de open.
- **Controle:** Implementação; P0 na versão; depende de SL-S09-04; evidência em `reports/S09/SL-S09-05.md`.

### SL-S09-06 — Limitar leitura e escrita

- **Executar:** Aplicar tamanho, tipo e destino permitido; evitar arquivos especiais inesperados.
- **Produzir:** `crates/security/src/io_limits.rs`.
- **Aceitar quando:** Arquivo/dispositivo excessivo ou tipo indevido é rejeitado antes de consumo sem limite.
- **Controle:** Implementação; P0 na versão; depende de SL-S09-05; evidência em `reports/S09/SL-S09-06.md`.

### SL-S09-07 — Autorizar fontes compartilhadas

- **Executar:** Separar assets/origens read-only de conteúdo privado de workspaces.
- **Produzir:** `crates/security/src/source_scope.rs`.
- **Aceitar quando:** Fonte aprovada pode ser lida sem conceder o diretório global inteiro.
- **Controle:** Implementação; P0 na versão; depende de SL-S09-06; evidência em `reports/S09/SL-S09-07.md`.

### SL-S09-08 — Criar corridas controladas

- **Executar:** Trocar symlinks/diretórios em loop de teste durante leituras.
- **Produzir:** `tests/security/toctou/`.
- **Aceitar quando:** Harness registra nenhuma leitura de sentinela externa; falhas não são ignoradas.
- **Controle:** Teste; P0 na versão; depende de SL-S09-07; evidência em `reports/S09/SL-S09-08.md`.

### SL-S09-09 — Cobrir nomes hostis e hard links

- **Executar:** Definir política explícita e testar absolutos, UNC, traversal e links.
- **Produzir:** `tests/security/path-policy/`.
- **Aceitar quando:** Política é aplicada e limitações de hard links/mounts constam do relatório.
- **Controle:** Teste; P0 na versão; depende de SL-S09-08; evidência em `reports/S09/SL-S09-09.md`.

### SL-S09-10 — Auditar APIs de filesystem

- **Executar:** Procurar acessos diretos fora dos módulos autorizados e criar regra de revisão.
- **Produzir:** `reports/security/fs-authority.md`.
- **Aceitar quando:** Fluxos de aplicação não têm bypass conhecido do broker para operações com dados não confiáveis.
- **Controle:** Teste; P0 na versão; depende de SL-S09-09; evidência em `reports/S09/SL-S09-10.md`.

### SL-S09-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Solicitar um arquivo permitido e negar escapes durante resolução e concorrência.**

Registrar commit, ambiente, testes e pendências em `reports/S09/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s10"></a>

## S10 — Reconciliar alterações externas e reconstruir índices

- **Objetivo:** Manter o disco como autoridade mesmo com edição externa e perda de notificações.
- **Componente / forma:** Watcher / projeções / Próprio.
- **Pré-requisitos técnicos:** S08 (Implementar transações de arquivos e histórico); S09 (Implementar broker de arquivos e defesa TOCTOU)
- **Entregável do sprint:** Reconciliação incremental e cache de inventário descartável.
- **Demonstração exigida:** Editar/renomear fora do app, perder um evento de watcher e reconciliar sem perda.
- **Requisitos:** R02, R04, R05, R21. **Risco de integração:** Crítico.

### SL-S10-01 — Integrar watcher

- **Executar:** Debounce eventos, distinguir escrita própria e tratar sequência rename/delete/create.
- **Produzir:** `crates/vault/src/watch.rs`.
- **Aceitar quando:** Uma escrita do Core não gera loop infinito de reimportação.
- **Controle:** Implementação; P0 na versão; depende de SL-S08-GATE, SL-S09-GATE; evidência em `reports/S10/SL-S10-01.md`.

### SL-S10-02 — Criar reconciliação por scan

- **Executar:** Comparar hashes/revisões e descobrir mudanças perdidas pelo watcher.
- **Produzir:** `crates/vault/src/reconcile.rs`.
- **Aceitar quando:** Eventos descartados intencionalmente são recuperados pela varredura posterior.
- **Controle:** Implementação; P0 na versão; depende de SL-S10-01; evidência em `reports/S10/SL-S10-02.md`.

### SL-S10-03 — Resolver renomeação por ID

- **Executar:** Atualizar localização sem recriar nota e sem quebrar referências.
- **Produzir:** `crates/vault/src/relocation.rs`.
- **Aceitar quando:** Citações e ocorrências continuam apontando para a mesma identidade.
- **Controle:** Implementação; P0 na versão; depende de SL-S10-02; evidência em `reports/S10/SL-S10-03.md`.

### SL-S10-04 — Detectar edição concorrente

- **Executar:** Comparar base, estado em edição e mudança externa.
- **Produzir:** `crates/domain/src/conflict.rs`.
- **Aceitar quando:** Nenhuma estratégia last-write-wins silenciosa destrói alteração autoral.
- **Controle:** Implementação; P0 na versão; depende de SL-S10-03; evidência em `reports/S10/SL-S10-04.md`.

### SL-S10-05 — Criar UI de conflito

- **Executar:** Mostrar versões, salvar cópia e permitir resolução explícita.
- **Produzir:** `apps/desktop/src/conflicts/`.
- **Aceitar quando:** Cancelar resolução conserva as duas versões e não bloqueia todo o Vault.
- **Controle:** Implementação; P0 na versão; depende de SL-S10-04; evidência em `reports/S10/SL-S10-05.md`.

### SL-S10-06 — Criar projeção SQLite

- **Executar:** Indexar inventário/revisões para navegação sem torná-lo canônico.
- **Produzir:** `crates/index/src/catalog.rs`.
- **Aceitar quando:** Apagar DB e reconstruir produz inventário equivalente ao scan direto.
- **Controle:** Implementação; P0 na versão; depende de SL-S10-05; evidência em `reports/S10/SL-S10-06.md`.

### SL-S10-07 — Registrar eventos de invalidação

- **Executar:** Emitir mudança de texto, metadado e topologia separadamente.
- **Produzir:** `crates/domain/src/change_events.rs`.
- **Aceitar quando:** Mudança de posição não é tratada como alteração do corpo textual.
- **Controle:** Implementação; P0 na versão; depende de SL-S10-06; evidência em `reports/S10/SL-S10-07.md`.

### SL-S10-08 — Invalidar tombstones

- **Executar:** Remover projeções de itens excluídos e impedir ressurreição por job atrasado.
- **Produzir:** `crates/index/src/tombstones.rs`.
- **Aceitar quando:** Resultado de revisão anterior não recria material excluído.
- **Controle:** Implementação; P0 na versão; depende de SL-S10-07; evidência em `reports/S10/SL-S10-08.md`.

### SL-S10-09 — Testar importação externa parcial

- **Executar:** Cobrir arquivo ainda sendo escrito, frontmatter inválido e lock externo.
- **Produzir:** `tests/vault/external-edits/`.
- **Aceitar quando:** App aguarda/reporta erro sem normalizar arquivo incompleto de modo destrutivo.
- **Controle:** Teste; P0 na versão; depende de SL-S10-08; evidência em `reports/S10/SL-S10-09.md`.

### SL-S10-10 — Validar reconstrução repetida

- **Executar:** Executar rebuild em Vault com referências e conflitos conhecidos.
- **Produzir:** `reports/rebuild-catalog.json`.
- **Aceitar quando:** Duas reconstruções têm as mesmas entidades e não alteram arquivos canônicos.
- **Controle:** Teste; P0 na versão; depende de SL-S10-09; evidência em `reports/S10/SL-S10-10.md`.

### SL-S10-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Editar/renomear fora do app, perder um evento de watcher e reconciliar sem perda.**

Registrar commit, ambiente, testes e pendências em `reports/S10/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s11"></a>

## S11 — Concluir v0.1: recuperação e diagnóstico do Vault

- **Objetivo:** Entregar um núcleo de dados utilizável antes de adicionar complexidade visual.
- **Componente / forma:** Vault / integração / Próprio.
- **Pré-requisitos técnicos:** S07 (Implementar Vault, identidade e assets); S08 (Implementar transações de arquivos e histórico); S09 (Implementar broker de arquivos e defesa TOCTOU); S10 (Reconciliar alterações externas e reconstruir índices)
- **Entregável do sprint:** v0.1.0 alpha técnica e utilitário de recuperação.
- **Demonstração exigida:** Criar, modificar, corromper um cache, reconstruir e exportar um Vault local.
- **Requisitos:** R02, R05, R06, R07, R29, R34. **Risco de integração:** Crítico.

### SL-S11-01 — Criar comando validate

- **Executar:** Inspecionar schemas, referências, objetos e journal sem efetuar reparo automático.
- **Produzir:** `tooling/vault-cli/validate`.
- **Aceitar quando:** Relatório distingue erro canônico, cache inválido e aviso recuperável.
- **Controle:** Implementação; P0 na versão; depende de SL-S07-GATE, SL-S08-GATE, SL-S09-GATE, SL-S10-GATE; evidência em `reports/S11/SL-S11-01.md`.

### SL-S11-02 — Criar comando rebuild

- **Executar:** Recriar projeções a partir de snapshot coerente e trocar índice ao concluir.
- **Produzir:** `tooling/vault-cli/rebuild`.
- **Aceitar quando:** Interrupção conserva o índice anterior ou retorna modo degradado; não altera autoria.
- **Controle:** Implementação; P0 na versão; depende de SL-S11-01; evidência em `reports/S11/SL-S11-02.md`.

### SL-S11-03 — Criar recuperação assistida

- **Executar:** Listar revisões recuperáveis e pedir confirmação antes de publicar reparo.
- **Produzir:** `tooling/vault-cli/recover`.
- **Aceitar quando:** Preview corresponde ao resultado e existe ponto de retorno.
- **Controle:** Implementação; P0 na versão; depende de SL-S11-02; evidência em `reports/S11/SL-S11-03.md`.

### SL-S11-04 — Criar exportação canônica

- **Executar:** Exportar manifest, textos, topologia, assets e histórico conforme perfil.
- **Produzir:** `tooling/vault-cli/export`.
- **Aceitar quando:** Vault exportado valida numa raiz diferente sem dados da máquina original.
- **Controle:** Implementação; P0 na versão; depende de SL-S11-03; evidência em `reports/S11/SL-S11-04.md`.

### SL-S11-05 — Executar matriz de crashes

- **Executar:** Interromper commits em todos os pontos instrumentados e comparar resultados.
- **Produzir:** `reports/v0.1/crash-matrix.json`.
- **Aceitar quando:** Nenhum estado confirmado fica irrecuperável nas falhas modeladas.
- **Controle:** Teste; P0 na versão; depende de SL-S11-04; evidência em `reports/S11/SL-S11-05.md`.

### SL-S11-06 — Executar ensaio de energia/disco

- **Executar:** Documentar setup seguro e limitações; usar ambiente descartável apropriado.
- **Produzir:** `reports/v0.1/power-failure.md`.
- **Aceitar quando:** Não há afirmação de durabilidade física baseada apenas em matar processo.
- **Controle:** Teste; P0 na versão; depende de SL-S11-05; evidência em `reports/S11/SL-S11-06.md`.

### SL-S11-07 — Apagar todos os caches

- **Executar:** Remover SQL/MPK/projeções previstas e executar a recuperação completa.
- **Produzir:** `reports/v0.1/cache-destruction.json`.
- **Aceitar quando:** Conteúdo, referências e histórico autoral são preservados byte a byte ou semanticamente conforme contrato.
- **Controle:** Teste; P0 na versão; depende de SL-S11-06; evidência em `reports/S11/SL-S11-07.md`.

### SL-S11-08 — Escrever manual do formato

- **Executar:** Explicar diretórios, autoridade, recovery e edição externa.
- **Produzir:** `docs/vault/format-v0.md`.
- **Aceitar quando:** Outro implementador consegue localizar dados sem abrir o app.
- **Controle:** Documentação; P0 na versão; depende de SL-S11-07; evidência em `reports/S11/SL-S11-08.md`.

### SL-S11-09 — Empacotar alpha técnica

- **Executar:** Anexar manifests, avisos, fontes e limitações dos alvos efetivamente testados.
- **Produzir:** `release/v0.1.0/`.
- **Aceitar quando:** Pacote não afirma conter editor/canvas/IA prontos nem homologação ausente.
- **Controle:** Licença; P0 na versão; depende de SL-S11-08; evidência em `reports/S11/SL-S11-09.md`.

### SL-S11-10 — Gravar demonstração de aceite

- **Executar:** Repetir jornada do sprint em instalação de desenvolvimento limpa.
- **Produzir:** `reports/v0.1/acceptance.md`.
- **Aceitar quando:** Autor aprova com logs, hashes e pendências; problemas de integridade bloqueiam a tag.
- **Controle:** Teste; P0 na versão; depende de SL-S11-09; evidência em `reports/S11/SL-S11-10.md`.

### SL-S11-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Criar, modificar, corromper um cache, reconstruir e exportar um Vault local.**

Registrar commit, ambiente, testes e pendências em `reports/S11/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

# v0.2.0 — Editor local BlockSuite

**Maturidade:** Alpha de autoria  
**Entrega acumulada:** Editar notas no perfil Markdown suportado e reabrir sem depender do estado interno do editor.

<a id="s12"></a>

## S12 — Extrair o recorte editorial do BlockSuite

- **Objetivo:** Reproduzir a edição mínima sem importar o produto AFFiNE.
- **Componente / forma:** BlockSuite / upstream / Desconstrução.
- **Pré-requisitos técnicos:** S03 (Inventariar upstreams e definir recortes); S04 (Montar repositório e fluxo solo com IA); S06 (Criar shell desktop e contratos IPC); S11 (Concluir v0.1: recuperação e diagnóstico do Vault)
- **Entregável do sprint:** Pacote editorial isolado e ADR do recorte.
- **Demonstração exigida:** Editor mínimo funciona fora do monorepo de origem e sua origem/licença é rastreável.
- **Requisitos:** R03, R10, R30, R33, R34. **Risco de integração:** Crítico.

### SL-S12-01 — Revalidar origem e revisão

- **Executar:** Escolher release/commit e árvore standalone/AFFiNE após auditar o recorte.
- **Produzir:** `upstream/blocksuite/manifest.json`.
- **Aceitar quando:** Revisão imutável e licenças por arquivo registradas; nenhuma decisão depende de canary flutuante.
- **Controle:** Upstream; P0 na versão; depende de SL-S03-GATE, SL-S04-GATE, SL-S06-GATE, SL-S11-GATE; evidência em `reports/S12/SL-S12-01.md`.

### SL-S12-02 — Reproduzir exemplo upstream

- **Executar:** Executar editor mínimo com fixtures antes de modificar código.
- **Produzir:** `reports/blocksuite/upstream-baseline.md`.
- **Aceitar quando:** Ambiente, comandos e comportamento original registrados; falha de origem não é atribuída ao adapter.
- **Controle:** Upstream; P0 na versão; depende de SL-S12-01; evidência em `reports/S12/SL-S12-02.md`.

### SL-S12-03 — Mapear grafo mínimo

- **Executar:** Rastrear store, std, rich-text, modelos, blocos e dependências workspace.
- **Produzir:** `upstream/blocksuite/dependency-graph.json`.
- **Aceitar quando:** Todas as dependências de runtime têm origem; copiar três diretórios não é presumido suficiente.
- **Controle:** Upstream; P0 na versão; depende de SL-S12-02; evidência em `reports/S12/SL-S12-03.md`.

### SL-S12-04 — Classificar componentes

- **Executar:** Separar edição, sync, gfx, copilot, cloud e componentes dependentes da aplicação.
- **Produzir:** `docs/upstream/blocksuite-boundaries.md`.
- **Aceitar quando:** Funcionalidades excluídas não reentram por import agregador não auditado.
- **Controle:** Upstream; P0 na versão; depende de SL-S12-03; evidência em `reports/S12/SL-S12-04.md`.

### SL-S12-05 — Criar pacote facade

- **Executar:** Expor criação/destruição, snapshot, comandos e eventos necessários ao Sandland.
- **Produzir:** `adapters/blocksuite/`.
- **Aceitar quando:** App não importa internals espalhados por toda a UI.
- **Controle:** Implementação; P0 na versão; depende de SL-S12-04; evidência em `reports/S12/SL-S12-05.md`.

### SL-S12-06 — Fechar resolução de pacotes

- **Executar:** Resolver workspace deps, assets e registro de custom elements sem duplicar Yjs.
- **Produzir:** `adapters/blocksuite/build/`.
- **Aceitar quando:** Build fora da árvore AFFiNE é reproduzível e não contém imports não resolvidos.
- **Controle:** Implementação; P0 na versão; depende de SL-S12-05; evidência em `reports/S12/SL-S12-06.md`.

### SL-S12-07 — Definir patch set mínimo

- **Executar:** Isolar mudanças indispensáveis e manter comparação contra upstream.
- **Produzir:** `upstream/blocksuite/patches/`.
- **Aceitar quando:** Cada patch tem motivação/teste; removê-lo revela apenas a diferença documentada.
- **Controle:** Implementação; P0 na versão; depende de SL-S12-06; evidência em `reports/S12/SL-S12-07.md`.

### SL-S12-08 — Testar ciclo mount/unmount

- **Executar:** Montar/desmontar editores repetidamente e verificar observers/listeners.
- **Produzir:** `tests/editor/lifecycle/`.
- **Aceitar quando:** Editor destruído não recebe eventos nem mantém documentos órfãos conhecidos.
- **Controle:** Teste; P0 na versão; depende de SL-S12-07; evidência em `reports/S12/SL-S12-08.md`.

### SL-S12-09 — Aprovar ADR editorial

- **Executar:** Fixar versão, recorte, risco, estratégia de atualização e fronteira com PixiJS.
- **Produzir:** `docs/adr/008-editor-engine.md`.
- **Aceitar quando:** Autor ratifica recorte; segundo canvas completo não é incorporado.
- **Controle:** Decisão; P0 na versão; depende de SL-S12-08; evidência em `reports/S12/SL-S12-09.md`.

### SL-S12-10 — Medir pacote isolado

- **Executar:** Registrar bundle, memória e dependências carregadas sem app completo.
- **Produzir:** `reports/blocksuite/isolation.json`.
- **Aceitar quando:** Relatório permite atribuir overhead ao editor e não declara meta de produto já satisfeita.
- **Controle:** Teste; P0 na versão; depende de SL-S12-09; evidência em `reports/S12/SL-S12-10.md`.

### SL-S12-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Editor mínimo funciona fora do monorepo de origem e sua origem/licença é rastreável.**

Registrar commit, ambiente, testes e pendências em `reports/S12/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s13"></a>

## S13 — Implementar ponte Markdown ↔ BlockSuite

- **Objetivo:** Conectar o modelo editorial ao formato autoral sem perda silenciosa.
- **Componente / forma:** EditorBridge / formato / Adaptação.
- **Pré-requisitos técnicos:** S02 (Definir formatos, identidade e autoridade); S12 (Extrair o recorte editorial do BlockSuite)
- **Entregável do sprint:** Serializer/deserializer e fixtures de round-trip.
- **Demonstração exigida:** Abrir Markdown externo, editar no BlockSuite e preservar a semântica e metadados.
- **Requisitos:** R02, R04, R10, R13, R32. **Risco de integração:** Alto.

### SL-S13-01 — Mapear blocos suportados

- **Executar:** Associar cada tipo aprovado a modelos/views e representação Markdown.
- **Produzir:** `adapters/blocksuite/block-map.json`.
- **Aceitar quando:** Todo bloco habilitado possui entrada/saída; tipo experimental não aparece como persistível sem codec.
- **Controle:** Contrato; P0 na versão; depende de SL-S02-GATE, SL-S12-GATE; evidência em `reports/S13/SL-S13-01.md`.

### SL-S13-02 — Importar texto estrutural

- **Executar:** Converter títulos, parágrafos, listas e blocos de código preservando conteúdo.
- **Produzir:** `adapters/blocksuite/import/basic.ts`.
- **Aceitar quando:** Fixtures com listas aninhadas, código e caracteres especiais mantêm significado.
- **Controle:** Implementação; P1 na versão; depende de SL-S13-01; evidência em `reports/S13/SL-S13-02.md`.

### SL-S13-03 — Importar estruturas ricas

- **Executar:** Tratar tabelas, citações, links e mídias no perfil suportado.
- **Produzir:** `adapters/blocksuite/import/rich.ts`.
- **Aceitar quando:** Elementos não suportados são preservados/opacos ou recusados explicitamente.
- **Controle:** Implementação; P1 na versão; depende de SL-S13-02; evidência em `reports/S13/SL-S13-03.md`.

### SL-S13-04 — Exportar Markdown

- **Executar:** Serializar a estrutura em ordem determinística sem depender da seleção visual.
- **Produzir:** `adapters/blocksuite/export/markdown.ts`.
- **Aceitar quando:** Exportação de documento completo não omite conteúdo fora da viewport.
- **Controle:** Implementação; P1 na versão; depende de SL-S13-03; evidência em `reports/S13/SL-S13-04.md`.

### SL-S13-05 — Preservar frontmatter

- **Executar:** Manter campos conhecidos/desconhecidos separados do estado de edição.
- **Produzir:** `adapters/blocksuite/frontmatter.ts`.
- **Aceitar quando:** Abrir/salvar não remove propriedades de ferramentas externas nem muda tipos indevidamente.
- **Controle:** Implementação; P1 na versão; depende de SL-S13-04; evidência em `reports/S13/SL-S13-05.md`.

### SL-S13-06 — Preservar identidade de blocos

- **Executar:** Associar IDs ao formato aprovado e reconciliar alterações externas.
- **Produzir:** `adapters/blocksuite/identity.ts`.
- **Aceitar quando:** Referência permanece estável em edição simples; mudança ambígua gera estado de reancoragem.
- **Controle:** Implementação; P1 na versão; depende de SL-S13-05; evidência em `reports/S13/SL-S13-06.md`.

### SL-S13-07 — Resolver assets por referência

- **Executar:** Trocar paths absolutos por IDs/hash e broker de leitura.
- **Produzir:** `adapters/blocksuite/assets.ts`.
- **Aceitar quando:** Conteúdo colado não injeta leitura de arquivo fora do Vault.
- **Controle:** Implementação; P1 na versão; depende de SL-S13-06; evidência em `reports/S13/SL-S13-07.md`.

### SL-S13-08 — Preservar conteúdo desconhecido

- **Executar:** Implementar nó opaco/estado read-only para extensão não compreendida.
- **Produzir:** `adapters/blocksuite/unknown-block.ts`.
- **Aceitar quando:** Documento novo aberto em versão antiga não perde extensão ao salvar conteúdo conhecido.
- **Controle:** Implementação; P1 na versão; depende de SL-S13-07; evidência em `reports/S13/SL-S13-08.md`.

### SL-S13-09 — Executar golden round-trip

- **Executar:** Comparar entrada, snapshot intermediário e saída com critérios sintáticos/semânticos definidos.
- **Produzir:** `tests/editor/roundtrip/`.
- **Aceitar quando:** Formatação permitida pode normalizar, mas textos, IDs e referências não desaparecem.
- **Controle:** Teste; P1 na versão; depende de SL-S13-08; evidência em `reports/S13/SL-S13-09.md`.

### SL-S13-10 — Fuzzar import/export

- **Executar:** Gerar Markdown/frontmatter truncados e combinações de blocos.
- **Produzir:** `tests/editor/codec-fuzz/`.
- **Aceitar quando:** Erro é recuperável e não produz publicação parcial ou loop sem limite.
- **Controle:** Teste; P1 na versão; depende de SL-S13-09; evidência em `reports/S13/SL-S13-10.md`.

### SL-S13-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Abrir Markdown externo, editar no BlockSuite e preservar a semântica e metadados.**

Registrar commit, ambiente, testes e pendências em `reports/S13/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s14"></a>

## S14 — Integrar comandos, salvamento e undo editorial

- **Objetivo:** Fazer o editor participar do protocolo de revisão do Sandland.
- **Componente / forma:** BlockSuite / domínio / Integração.
- **Pré-requisitos técnicos:** S08 (Implementar transações de arquivos e histórico); S10 (Reconciliar alterações externas e reconstruir índices); S13 (Implementar ponte Markdown ↔ BlockSuite)
- **Entregável do sprint:** Edição de notas com salvamento confiável e undo de produto.
- **Demonstração exigida:** Editar, desfazer, conflitar com editor externo e recuperar sem cópia oculta no Yjs.
- **Requisitos:** R04, R05, R10, R31. **Risco de integração:** Crítico.

### SL-S14-01 — Adaptar eventos de edição

- **Executar:** Transformar mudanças do editor em operações com base_revision.
- **Produzir:** `adapters/blocksuite/change-adapter.ts`.
- **Aceitar quando:** Duas sessões desatualizadas não publicam sobre a mesma revisão sem conflito.
- **Controle:** Implementação; P0 na versão; depende de SL-S08-GATE, SL-S10-GATE, SL-S13-GATE; evidência em `reports/S14/SL-S14-01.md`.

### SL-S14-02 — Implementar fila de autosave

- **Executar:** Agregar mudanças sem bloquear renderização e enviar ao escritor único.
- **Produzir:** `apps/desktop/src/editor/save-queue.ts`.
- **Aceitar quando:** Digitação permanece responsiva e erro de disco não é exibido como salvo.
- **Controle:** Implementação; P0 na versão; depende de SL-S14-01; evidência em `reports/S14/SL-S14-02.md`.

### SL-S14-03 — Exibir estados de persistência

- **Executar:** Diferenciar editando, pendente, confirmado, conflito e falha.
- **Produzir:** `apps/desktop/src/editor/save-status.ts`.
- **Aceitar quando:** Fechar com dados não confirmados alerta e conserva material recuperável.
- **Controle:** Implementação; P0 na versão; depende de SL-S14-02; evidência em `reports/S14/SL-S14-03.md`.

### SL-S14-04 — Integrar undo/redo

- **Executar:** Delimitar histórico de edição em sessão e eventos semânticos duráveis.
- **Produzir:** `crates/editor-domain/src/history.rs`.
- **Aceitar quando:** Undo não atravessa silenciosamente uma alteração externa ou apaga proveniência.
- **Controle:** Implementação; P0 na versão; depende de SL-S14-03; evidência em `reports/S14/SL-S14-04.md`.

### SL-S14-05 — Integrar comandos e atalhos

- **Executar:** Mapear seleção, inserção, formatação e navegação ao shell.
- **Produzir:** `apps/desktop/src/editor/commands.ts`.
- **Aceitar quando:** Atalho de editor não dispara simultaneamente comando de canvas ou aplicação.
- **Controle:** Implementação; P0 na versão; depende de SL-S14-04; evidência em `reports/S14/SL-S14-05.md`.

### SL-S14-06 — Controlar clipboard

- **Executar:** Importar texto/HTML/assets por pipeline seguro e apresentar resultado previsível.
- **Produzir:** `apps/desktop/src/editor/clipboard.ts`.
- **Aceitar quando:** Colagem de HTML não executa script e referências locais não autorizadas são negadas.
- **Controle:** Implementação; P0 na versão; depende de SL-S14-05; evidência em `reports/S14/SL-S14-06.md`.

### SL-S14-07 — Retomar rascunho após falha

- **Executar:** Reconstruir documento a partir de revisão/checkpoint autorizado.
- **Produzir:** `apps/desktop/src/editor/recovery.ts`.
- **Aceitar quando:** Reabertura não depende de IndexedDB/Yjs como única fonte do conteúdo salvo.
- **Controle:** Implementação; P0 na versão; depende de SL-S14-06; evidência em `reports/S14/SL-S14-07.md`.

### SL-S14-08 — Tratar documento read-only

- **Executar:** Exibir fontes imutáveis e sugerir criar nota/recorte para editar.
- **Produzir:** `apps/desktop/src/editor/readonly.ts`.
- **Aceitar quando:** Formatação/clipboard não conseguem modificar fonte marcada somente leitura pelo domínio.
- **Controle:** Implementação; P0 na versão; depende de SL-S14-07; evidência em `reports/S14/SL-S14-08.md`.

### SL-S14-09 — Exercitar IME e seleção

- **Executar:** Cobrir composição de acentos, teclados diferentes e seleção extensa.
- **Produzir:** `tests/editor/input/`.
- **Aceitar quando:** Autosave e rerender não interrompem composição nem deslocam caret indevidamente.
- **Controle:** Teste; P0 na versão; depende de SL-S14-08; evidência em `reports/S14/SL-S14-09.md`.

### SL-S14-10 — Validar conflitos ponta a ponta

- **Executar:** Editar o mesmo arquivo fora do app durante digitação.
- **Produzir:** `reports/editor/concurrency.md`.
- **Aceitar quando:** Usuário resolve o conflito e ambas as versões podem ser recuperadas.
- **Controle:** Teste; P0 na versão; depende de SL-S14-09; evidência em `reports/S14/SL-S14-10.md`.

### SL-S14-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Editar, desfazer, conflitar com editor externo e recuperar sem cópia oculta no Yjs.**

Registrar commit, ambiente, testes e pendências em `reports/S14/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s15"></a>

## S15 — Entregar notas locais e homologar editor

- **Objetivo:** Concluir um produto de autoria textual útil antes de adicionar a mesa.
- **Componente / forma:** Editor / Tauri / Integração.
- **Pré-requisitos técnicos:** S14 (Integrar comandos, salvamento e undo editorial)
- **Entregável do sprint:** v0.2.0 alpha de notas locais.
- **Demonstração exigida:** Instalação limpa permite criar, editar, pesquisar por título e reabrir notas sem rede.
- **Requisitos:** R07, R09, R10, R31, R34. **Risco de integração:** Alto.

### SL-S15-01 — Criar lista e navegação de notas

- **Executar:** Usar inventário local e identidade, sem indexação semântica obrigatória.
- **Produzir:** `apps/desktop/src/notes/`.
- **Aceitar quando:** Renomeação preserva seleção e documento aberto.
- **Controle:** Implementação; P1 na versão; depende de SL-S14-GATE; evidência em `reports/S15/SL-S15-01.md`.

### SL-S15-02 — Criar ações de documento

- **Executar:** Novo, duplicar, mover, lixeira e restaurar usando o Core.
- **Produzir:** `crates/editor-domain/src/doc_actions.rs`.
- **Aceitar quando:** Duplicata possui nova identidade e restauração não sobrepõe outro arquivo existente.
- **Controle:** Implementação; P1 na versão; depende de SL-S15-01; evidência em `reports/S15/SL-S15-02.md`.

### SL-S15-03 — Implementar busca simples por título

- **Executar:** Consultar inventário sem antecipar o backend vetorial.
- **Produzir:** `apps/desktop/src/notes/title-search.ts`.
- **Aceitar quando:** Funciona com modelos ausentes e não envia texto a serviço externo.
- **Controle:** Implementação; P1 na versão; depende de SL-S15-02; evidência em `reports/S15/SL-S15-03.md`.

### SL-S15-04 — Construir leitura segura

- **Executar:** Renderizar links/assets e bloquear conteúdo ativo não pertencente ao app.
- **Produzir:** `apps/desktop/src/reader/`.
- **Aceitar quando:** Abrir nota hostil não acessa IPC privilegiado nem recursos remotos por padrão.
- **Controle:** Implementação; P1 na versão; depende de SL-S15-03; evidência em `reports/S15/SL-S15-04.md`.

### SL-S15-05 — Polir foco e teclado

- **Executar:** Definir sequência entre lista, editor, toolbar e estados de erro.
- **Produzir:** `apps/desktop/src/notes/focus.ts`.
- **Aceitar quando:** Jornada criar→editar→salvar→reabrir pode ser feita com teclado.
- **Controle:** Implementação; P1 na versão; depende de SL-S15-04; evidência em `reports/S15/SL-S15-05.md`.

### SL-S15-06 — Comparar renderização entre WebViews

- **Executar:** Executar corpus visual e de input nos alvos disponíveis.
- **Produzir:** `tests/editor/webview-matrix/`.
- **Aceitar quando:** Divergência tem issue e capacidade limitada; ausência de teste não é tratada como aprovação.
- **Controle:** Teste; P1 na versão; depende de SL-S15-05; evidência em `reports/S15/SL-S15-06.md`.

### SL-S15-07 — Medir sessões repetidas

- **Executar:** Alternar documentos e destruir instâncias medindo memória residual.
- **Produzir:** `reports/v0.2/editor-resources.json`.
- **Aceitar quando:** Crescimento acumulativo fora do perfil aprovado bloqueia a release.
- **Controle:** Teste; P1 na versão; depende de SL-S15-06; evidência em `reports/S15/SL-S15-07.md`.

### SL-S15-08 — Validar autoria offline

- **Executar:** Bloquear rede, reiniciar e repetir edição/importação de Markdown.
- **Produzir:** `reports/v0.2/offline.json`.
- **Aceitar quando:** Zero egress inesperado e nenhuma dependência de autenticação remota.
- **Controle:** Teste; P1 na versão; depende de SL-S15-07; evidência em `reports/S15/SL-S15-08.md`.

### SL-S15-09 — Publicar perfil suportado

- **Executar:** Listar blocos, atalhos, limites e comportamento de alterações externas.
- **Produzir:** `docs/user/notes.md`.
- **Aceitar quando:** Usuário sabe quando um bloco é opaco/read-only e como recuperar conteúdo.
- **Controle:** Documentação; P1 na versão; depende de SL-S15-08; evidência em `reports/S15/SL-S15-09.md`.

### SL-S15-10 — Preparar alpha de autoria

- **Executar:** Anexar corpus, origem BlockSuite, avisos e demonstração de reconstrução.
- **Produzir:** `release/v0.2.0/`.
- **Aceitar quando:** Caches do editor podem ser apagados sem perder notas confirmadas.
- **Controle:** Release; P1 na versão; depende de SL-S15-09; evidência em `reports/S15/SL-S15-10.md`.

### SL-S15-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Instalação limpa permite criar, editar, pesquisar por título e reabrir notas sem rede.**

Registrar commit, ambiente, testes e pendências em `reports/S15/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

# v0.3.0 — Mesa espacial

**Maturidade:** Alpha de autoria  
**Entrega acumulada:** Organizar células, grupos e relações com PixiJS e edição BlockSuite integrada.

<a id="s16"></a>

## S16 — Construir cena PixiJS e interação básica

- **Objetivo:** Criar espaço de trabalho visual sem delegar a semântica do board a um motor externo.
- **Componente / forma:** PixiJS / Whiteboard / Integração.
- **Pré-requisitos técnicos:** S06 (Criar shell desktop e contratos IPC); S07 (Implementar Vault, identidade e assets); S15 (Entregar notas locais e homologar editor)
- **Entregável do sprint:** Viewport com cartões, câmera, seleção e previews.
- **Demonstração exigida:** Criar e navegar por cartões mantendo coordenadas consistentes em diferentes zooms.
- **Requisitos:** R11, R12, R31. **Risco de integração:** Alto.

### SL-S16-01 — Fixar PixiJS e viewport

- **Executar:** Escolher versões e APIs mínimas; registrar origem e controles de teardown.
- **Produzir:** `upstream/pixi/manifest.json`.
- **Aceitar quando:** Cena mínima reproduzível sem importar outro produto de whiteboard.
- **Controle:** Upstream; P1 na versão; depende de SL-S06-GATE, SL-S07-GATE, SL-S15-GATE; evidência em `reports/S16/SL-S16-01.md`.

### SL-S16-02 — Criar sistema de coordenadas

- **Executar:** Separar screen/world e documentar transformação, escala e precisão.
- **Produzir:** `apps/desktop/src/board/coordinates.ts`.
- **Aceitar quando:** Conversão ida/volta é testada em zooms extremos e escalas de tela.
- **Controle:** Implementação; P1 na versão; depende de SL-S16-01; evidência em `reports/S16/SL-S16-02.md`.

### SL-S16-03 — Criar câmera

- **Executar:** Pan, zoom ancorado ao cursor, fit e restauração de viewport.
- **Produzir:** `apps/desktop/src/board/camera.ts`.
- **Aceitar quando:** Zoom não altera posições canônicas dos nós.
- **Controle:** Implementação; P1 na versão; depende de SL-S16-02; evidência em `reports/S16/SL-S16-03.md`.

### SL-S16-04 — Renderizar cartões passivos

- **Executar:** Desenhar contorno, título, preview e placeholder de asset.
- **Produzir:** `apps/desktop/src/board/card-renderer.ts`.
- **Aceitar quando:** Cartão sem editor montado é legível e mantém identidade de domínio.
- **Controle:** Implementação; P1 na versão; depende de SL-S16-03; evidência em `reports/S16/SL-S16-04.md`.

### SL-S16-05 — Implementar seleção

- **Executar:** Clique, multiseleção, retângulo e limpar seleção com foco previsível.
- **Produzir:** `apps/desktop/src/board/selection.ts`.
- **Aceitar quando:** Seleção visual não modifica conteúdo e não inclui nós ocultos indevidos.
- **Controle:** Implementação; P1 na versão; depende de SL-S16-04; evidência em `reports/S16/SL-S16-05.md`.

### SL-S16-06 — Implementar drag/resize

- **Executar:** Aplicar transformações temporárias e confirmar operação semântica ao terminar.
- **Produzir:** `apps/desktop/src/board/manipulation.ts`.
- **Aceitar quando:** Cancelar gesto restaura posição; não há commit por pixel movido.
- **Controle:** Implementação; P1 na versão; depende de SL-S16-05; evidência em `reports/S16/SL-S16-06.md`.

### SL-S16-07 — Implementar hit testing

- **Executar:** Separar alvos de cartão, conector, grupo e canvas.
- **Produzir:** `apps/desktop/src/board/hit-test.ts`.
- **Aceitar quando:** Zoom/escala não fazem clique em célula acionar widget vizinho.
- **Controle:** Implementação; P1 na versão; depende de SL-S16-06; evidência em `reports/S16/SL-S16-07.md`.

### SL-S16-08 — Criar previews limitados

- **Executar:** Truncar apresentação sem truncar o conteúdo canônico.
- **Produzir:** `apps/desktop/src/board/previews.ts`.
- **Aceitar quando:** Texto oculto no preview continua presente e editável no documento.
- **Controle:** Implementação; P1 na versão; depende de SL-S16-07; evidência em `reports/S16/SL-S16-08.md`.

### SL-S16-09 — Testar cena determinística

- **Executar:** Reproduzir gestos e capturas em corpus de posições e tamanhos.
- **Produzir:** `tests/board/scene-basic/`.
- **Aceitar quando:** Mesma sequência de comandos produz mesma topologia, sem depender de frame rate.
- **Controle:** Teste; P1 na versão; depende de SL-S16-08; evidência em `reports/S16/SL-S16-09.md`.

### SL-S16-10 — Medir frame time inicial

- **Executar:** Registrar baseline com cenas sintéticas e identificar gargalos antes do LOD.
- **Produzir:** `reports/board/scene-baseline.json`.
- **Aceitar quando:** Resultado inclui hardware e payload; 60 FPS não é alegado por observação subjetiva.
- **Controle:** Teste; P1 na versão; depende de SL-S16-09; evidência em `reports/S16/SL-S16-10.md`.

### SL-S16-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Criar e navegar por cartões mantendo coordenadas consistentes em diferentes zooms.**

Registrar commit, ambiente, testes e pendências em `reports/S16/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s17"></a>

## S17 — Persistir topologia, grupos e relações

- **Objetivo:** Transformar a cena em conhecimento espacial persistente.
- **Componente / forma:** BoardService / topologia / Próprio.
- **Pré-requisitos técnicos:** S08 (Implementar transações de arquivos e histórico); S10 (Reconciliar alterações externas e reconstruir índices); S16 (Construir cena PixiJS e interação básica)
- **Entregável do sprint:** BoardService canônico e cache MPK validado por revisão.
- **Demonstração exigida:** Criar ciclo de relações, agrupar, fechar, apagar MPK e reconstruir a mesma mesa.
- **Requisitos:** R02, R05, R11, R22. **Risco de integração:** Crítico.

### SL-S17-01 — Criar comandos de board

- **Executar:** Adicionar/remover ocorrência e alterar posição/tamanho por revisão.
- **Produzir:** `crates/board/src/commands.rs`.
- **Aceitar quando:** Operação visual só fica confirmada após persistência no domínio.
- **Controle:** Implementação; P0 na versão; depende de SL-S08-GATE, SL-S10-GATE, SL-S16-GATE; evidência em `reports/S17/SL-S17-01.md`.

### SL-S17-02 — Separar conteúdo/ocorrência

- **Executar:** Referenciar nota por ID e manter propriedades locais no cartão.
- **Produzir:** `crates/board/src/occurrences.rs`.
- **Aceitar quando:** Duas ocorrências editam o mesmo conteúdo, mas não compartilham coordenadas.
- **Controle:** Implementação; P0 na versão; depende de SL-S17-01; evidência em `reports/S17/SL-S17-02.md`.

### SL-S17-03 — Implementar grupos

- **Executar:** Criar membership explícito, rótulo e limites geométricos.
- **Produzir:** `crates/board/src/groups.rs`.
- **Aceitar quando:** Proximidade não cria relação semântica por conta própria.
- **Controle:** Implementação; P0 na versão; depende de SL-S17-02; evidência em `reports/S17/SL-S17-03.md`.

### SL-S17-04 — Implementar arestas

- **Executar:** Persistir direção, rótulo, origem manual/IA e endpoints válidos.
- **Produzir:** `crates/board/src/edges.rs`.
- **Aceitar quando:** Relações apoia/contradiz são distintas; loops/ciclos permitidos têm representação consistente.
- **Controle:** Implementação; P0 na versão; depende de SL-S17-03; evidência em `reports/S17/SL-S17-04.md`.

### SL-S17-05 — Validar referência órfã

- **Executar:** Definir comportamento para nó removido, conteúdo ausente e fonte excluída.
- **Produzir:** `crates/board/src/validation.rs`.
- **Aceitar quando:** Estado inconsistente aparece para reparo, sem remoção silenciosa de autoria.
- **Controle:** Implementação; P0 na versão; depende de SL-S17-04; evidência em `reports/S17/SL-S17-05.md`.

### SL-S17-06 — Serializar JSON estável

- **Executar:** Ordenar coleções por regra explícita e evitar ruído desnecessário de diff.
- **Produzir:** `crates/board/src/codec_json.rs`.
- **Aceitar quando:** Regravação sem mudança não produz diff semântico nem altera IDs.
- **Controle:** Implementação; P0 na versão; depende de SL-S17-05; evidência em `reports/S17/SL-S17-06.md`.

### SL-S17-07 — Gerar MPK derivado

- **Executar:** Vincular cache ao hash/schema da topologia e reconstruir quando inválido.
- **Produzir:** `crates/board/src/cache_mpk.rs`.
- **Aceitar quando:** MPK mais recente por timestamp não vence JSON/journal autoritativo.
- **Controle:** Implementação; P0 na versão; depende de SL-S17-06; evidência em `reports/S17/SL-S17-07.md`.

### SL-S17-08 — Criar undo de operações espaciais

- **Executar:** Agrupar drag/resize e manter alterações de relações reversíveis.
- **Produzir:** `crates/board/src/undo.rs`.
- **Aceitar quando:** Desfazer movimento não reverte indevidamente edição textual posterior.
- **Controle:** Implementação; P0 na versão; depende de SL-S17-07; evidência em `reports/S17/SL-S17-08.md`.

### SL-S17-09 — Testar topologias complexas

- **Executar:** Cobrir ciclos, referências cruzadas, grupos e múltiplas ocorrências.
- **Produzir:** `tests/board/topology/`.
- **Aceitar quando:** Nenhuma transformação para árvore apaga arestas canônicas.
- **Controle:** Teste; P0 na versão; depende de SL-S17-08; evidência em `reports/S17/SL-S17-09.md`.

### SL-S17-10 — Executar reconstrução do board

- **Executar:** Remover caches e reabrir após edições externas controladas.
- **Produzir:** `reports/board/rebuild.json`.
- **Aceitar quando:** Geometria, grupos e relações são equivalentes ao estado confirmado.
- **Controle:** Teste; P0 na versão; depende de SL-S17-09; evidência em `reports/S17/SL-S17-10.md`.

### SL-S17-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Criar ciclo de relações, agrupar, fechar, apagar MPK e reconstruir a mesma mesa.**

Registrar commit, ambiente, testes e pendências em `reports/S17/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s18"></a>

## S18 — Integrar overlay BlockSuite à Mesa

- **Objetivo:** Editar conteúdo no espaço visual sem instanciar um editor completo por nó.
- **Componente / forma:** PixiJS + BlockSuite / Adaptação.
- **Pré-requisitos técnicos:** S14 (Integrar comandos, salvamento e undo editorial); S17 (Persistir topologia, grupos e relações)
- **Entregável do sprint:** Overlay DOM sincronizado à câmera e política de editores ativos.
- **Demonstração exigida:** Editar uma célula em diferentes zooms e voltar à cena sem perder caret ou conteúdo.
- **Requisitos:** R04, R10, R11, R12, R31. **Risco de integração:** Alto.

### SL-S18-01 — Definir política de edição ativa

- **Executar:** Fixar limite de editores, modo overlay/painel e interação com seleção de cartão.
- **Produzir:** `docs/contracts/board-editor.md`.
- **Aceitar quando:** Número de nós não implica o mesmo número de documentos DOM montados.
- **Controle:** Contrato; P0 na versão; depende de SL-S14-GATE, SL-S17-GATE; evidência em `reports/S18/SL-S18-01.md`.

### SL-S18-02 — Posicionar overlay

- **Executar:** Traduzir coordenadas world→screen e aplicar clipping/dimensões apropriadas.
- **Produzir:** `apps/desktop/src/board/editor-overlay.ts`.
- **Aceitar quando:** Pan, zoom e resize de janela mantêm editor alinhado ao cartão selecionado.
- **Controle:** Implementação; P1 na versão; depende de SL-S18-01; evidência em `reports/S18/SL-S18-02.md`.

### SL-S18-03 — Gerenciar foco

- **Executar:** Transferir foco entre canvas e editor sem conflitos de comandos.
- **Produzir:** `apps/desktop/src/board/focus-controller.ts`.
- **Aceitar quando:** Escape encerra edição conforme regra e não descarta alterações não salvas.
- **Controle:** Implementação; P1 na versão; depende de SL-S18-02; evidência em `reports/S18/SL-S18-03.md`.

### SL-S18-04 — Integrar caret/seleção

- **Executar:** Manter seleção textual durante atualização de preview e mudanças de layout.
- **Produzir:** `apps/desktop/src/board/text-selection.ts`.
- **Aceitar quando:** Rerender do board não reposiciona caret a cada tecla.
- **Controle:** Implementação; P1 na versão; depende de SL-S18-03; evidência em `reports/S18/SL-S18-04.md`.

### SL-S18-05 — Tratar IME no overlay

- **Executar:** Coordenar composição com autosave e término da edição.
- **Produzir:** `apps/desktop/src/board/ime.ts`.
- **Aceitar quando:** Acentos e composição não confirmada não são cortados por drag ou blur.
- **Controle:** Implementação; P1 na versão; depende de SL-S18-04; evidência em `reports/S18/SL-S18-05.md`.

### SL-S18-06 — Atualizar preview por revisão

- **Executar:** Renderizar snapshot do conteúdo após mudança sem reabrir todos os editores.
- **Produzir:** `apps/desktop/src/board/preview-cache.ts`.
- **Aceitar quando:** Mover cartão não invalida cache textual; editar texto invalida o preview correto.
- **Controle:** Implementação; P1 na versão; depende de SL-S18-05; evidência em `reports/S18/SL-S18-06.md`.

### SL-S18-07 — Limitar montagem por visibilidade

- **Executar:** Destruir instâncias passivas e preservar somente estado necessário.
- **Produzir:** `apps/desktop/src/board/editor-pool.ts`.
- **Aceitar quando:** Sessões repetidas não acumulam editores/observers ocultos.
- **Controle:** Implementação; P1 na versão; depende de SL-S18-06; evidência em `reports/S18/SL-S18-07.md`.

### SL-S18-08 — Criar modo de acessibilidade

- **Executar:** Oferecer navegação em lista/estrutura correspondente à mesa.
- **Produzir:** `apps/desktop/src/board/accessible-view.ts`.
- **Aceitar quando:** Conteúdo e relações essenciais são acessíveis sem depender exclusivamente do WebGL.
- **Controle:** Implementação; P1 na versão; depende de SL-S18-07; evidência em `reports/S18/SL-S18-08.md`.

### SL-S18-09 — Executar matriz de input visual

- **Executar:** Testar zoom, DPI, janelas e seleção atravessando mudanças de viewport.
- **Produzir:** `tests/board/editor-overlay/`.
- **Aceitar quando:** Nenhuma perda de texto/seleção crítica nos cenários homologados.
- **Controle:** Teste; P1 na versão; depende de SL-S18-08; evidência em `reports/S18/SL-S18-09.md`.

### SL-S18-10 — Medir limite ativo

- **Executar:** Abrir grande board, alternar edição e contar instâncias/memória.
- **Produzir:** `reports/board/editor-pool.json`.
- **Aceitar quando:** Limite aprovado é respeitado e consumo não cresce linearmente por editores inativos.
- **Controle:** Teste; P1 na versão; depende de SL-S18-09; evidência em `reports/S18/SL-S18-10.md`.

### SL-S18-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Editar uma célula em diferentes zooms e voltar à cena sem perder caret ou conteúdo.**

Registrar commit, ambiente, testes e pendências em `reports/S18/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s19"></a>

## S19 — Concluir navegação, LOD e workspaces

- **Objetivo:** Entregar a Mesa como etapa de exploração independente da IA.
- **Componente / forma:** Whiteboard / performance / Próprio.
- **Pré-requisitos técnicos:** S16 (Construir cena PixiJS e interação básica); S17 (Persistir topologia, grupos e relações); S18 (Integrar overlay BlockSuite à Mesa)
- **Entregável do sprint:** v0.3.0 com pílulas, grupos, conexões e LOD.
- **Demonstração exigida:** Navegar entre boards de 100/1.000 nós e reconstruir a cena após reinício.
- **Requisitos:** R01, R09, R11, R12, R31. **Risco de integração:** Alto.

### SL-S19-01 — Criar galeria de workspaces

- **Executar:** Abrir, criar, renomear e arquivar pílulas por identidade.
- **Produzir:** `apps/desktop/src/workspaces/`.
- **Aceitar quando:** Trocar workspace salva/expõe pendências e não mistura seleção/conteúdo entre boards.
- **Controle:** Implementação; P1 na versão; depende de SL-S16-GATE, SL-S17-GATE, SL-S18-GATE; evidência em `reports/S19/SL-S19-01.md`.

### SL-S19-02 — Implementar culling

- **Executar:** Excluir da renderização elementos fora da câmera preservando dados do domínio.
- **Produzir:** `apps/desktop/src/board/culling.ts`.
- **Aceitar quando:** Nó invisível continua persistido e retorna corretamente ao viewport.
- **Controle:** Implementação; P1 na versão; depende de SL-S19-01; evidência em `reports/S19/SL-S19-02.md`.

### SL-S19-03 — Implementar níveis de detalhe

- **Executar:** Usar previews, títulos e proxies conforme zoom e orçamento.
- **Produzir:** `apps/desktop/src/board/lod.ts`.
- **Aceitar quando:** Mudança de LOD não altera conteúdo, tamanho persistido ou alvo de interação.
- **Controle:** Implementação; P1 na versão; depende de SL-S19-02; evidência em `reports/S19/SL-S19-03.md`.

### SL-S19-04 — Controlar assets/texturas

- **Executar:** Carregar imagens por demanda e liberar recursos gráficos não usados.
- **Produzir:** `apps/desktop/src/board/texture-cache.ts`.
- **Aceitar quando:** Alternar boards pesados não mantém todas as texturas residentes.
- **Controle:** Implementação; P1 na versão; depende de SL-S19-03; evidência em `reports/S19/SL-S19-04.md`.

### SL-S19-05 — Otimizar relações

- **Executar:** Atualizar somente conectores afetados e aplicar estratégia para arestas fora da tela.
- **Produzir:** `apps/desktop/src/board/edge-renderer.ts`.
- **Aceitar quando:** Mover um nó não recalcula toda a cena sem necessidade demonstrada.
- **Controle:** Implementação; P1 na versão; depende de SL-S19-04; evidência em `reports/S19/SL-S19-05.md`.

### SL-S19-06 — Criar operações em lote

- **Executar:** Duplicar ocorrências, alinhar e agrupar com uma operação reversível.
- **Produzir:** `crates/board/src/batch.rs`.
- **Aceitar quando:** Cancelar/falhar não deixa lote parcialmente aplicado sem recuperação.
- **Controle:** Implementação; P1 na versão; depende de SL-S19-05; evidência em `reports/S19/SL-S19-06.md`.

### SL-S19-07 — Salvar viewport como preferência

- **Executar:** Separar navegação pessoal da topologia autoral quando apropriado.
- **Produzir:** `crates/board/src/view_state.rs`.
- **Aceitar quando:** Pan sem mudança de conhecimento não gera ruído indevido no histórico autoral.
- **Controle:** Implementação; P1 na versão; depende de SL-S19-06; evidência em `reports/S19/SL-S19-07.md`.

### SL-S19-08 — Medir cenas representativas

- **Executar:** Rodar benchmark de 100/1.000 nós, imagens e arestas densas.
- **Produzir:** `reports/v0.3/frame-times.json`.
- **Aceitar quando:** Perfil documenta p95/frame time, recursos e limitações; regressões bloqueiam a tag.
- **Controle:** Teste; P1 na versão; depende de SL-S19-07; evidência em `reports/S19/SL-S19-08.md`.

### SL-S19-09 — Validar isolamento de workspace

- **Executar:** Abrir vários boards e tentar resolver referências fora do escopo permitido.
- **Produzir:** `tests/board/workspace-scope/`.
- **Aceitar quando:** UI e broker respeitam política mesmo com cache de board anterior ainda existente.
- **Controle:** Teste; P1 na versão; depende de SL-S19-08; evidência em `reports/S19/SL-S19-09.md`.

### SL-S19-10 — Publicar alpha da Mesa

- **Executar:** Gravar demo, limites, guia de teclado e relatório de reconstrução.
- **Produzir:** `release/v0.3.0/`.
- **Aceitar quando:** Produto permite exploração espacial sem modelos, rede ou conta.
- **Controle:** Release; P1 na versão; depende de SL-S19-09; evidência em `reports/S19/SL-S19-10.md`.

### SL-S19-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Navegar entre boards de 100/1.000 nós e reconstruir a cena após reinício.**

Registrar commit, ambiente, testes e pendências em `reports/S19/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

# v0.4.0 — Peça e proveniência

**Maturidade:** Alpha funcional sem IA  
**Entrega acumulada:** Produzir texto editorial a partir da Mesa com fork-on-insert, referências e exportação.

<a id="s20"></a>

## S20 — Construir editor da Peça e layout editorial

- **Objetivo:** Criar produção textual formal sem misturá-la ao estado visual da Mesa.
- **Componente / forma:** BlockSuite / Peça / Integração.
- **Pré-requisitos técnicos:** S15 (Entregar notas locais e homologar editor); S19 (Concluir navegação, LOD e workspaces)
- **Entregável do sprint:** Peça independente com painel editorial e navegação.
- **Demonstração exigida:** Abrir Peça vinculada ao workspace e editar enquanto consulta células.
- **Requisitos:** R04, R10, R13, R31. **Risco de integração:** Alto.

### SL-S20-01 — Definir entidade Peça

- **Executar:** Fixar identidade, revisões, vínculo com workspace e estado editorial.
- **Produzir:** `schemas/piece-v0.json`.
- **Aceitar quando:** Peça não é apenas uma célula com flag; pode ser exportada/reaberta independentemente do layout.
- **Controle:** Contrato; P0 na versão; depende de SL-S15-GATE, SL-S19-GATE; evidência em `reports/S20/SL-S20-01.md`.

### SL-S20-02 — Criar lifecycle da Peça

- **Executar:** Novo, abrir, renomear, arquivar e restaurar pelo Core.
- **Produzir:** `crates/piece/src/lifecycle.rs`.
- **Aceitar quando:** Paths e nomes não alteram identidade nem colidem silenciosamente.
- **Controle:** Implementação; P1 na versão; depende de SL-S20-01; evidência em `reports/S20/SL-S20-02.md`.

### SL-S20-03 — Montar editor editorial

- **Executar:** Reutilizar facade BlockSuite com perfil e comandos apropriados à redação.
- **Produzir:** `apps/desktop/src/piece/editor.ts`.
- **Aceitar quando:** Não se cria outro motor de texto com codec divergente do usado nas notas.
- **Controle:** Implementação; P1 na versão; depende de SL-S20-02; evidência em `reports/S20/SL-S20-03.md`.

### SL-S20-04 — Criar painel dockado

- **Executar:** Abrir/retrair/redimensionar respeitando largura e foco do canvas.
- **Produzir:** `apps/desktop/src/piece/dock.ts`.
- **Aceitar quando:** Redimensionar não altera conteúdo nem dispara eventos espaciais de cartões.
- **Controle:** Implementação; P1 na versão; depende de SL-S20-03; evidência em `reports/S20/SL-S20-04.md`.

### SL-S20-05 — Criar outline e navegação

- **Executar:** Derivar headings da Peça e navegar por blocos estáveis.
- **Produzir:** `apps/desktop/src/piece/outline.ts`.
- **Aceitar quando:** Reordenar headings atualiza navegação sem quebrar IDs de citações preservados.
- **Controle:** Implementação; P1 na versão; depende de SL-S20-04; evidência em `reports/S20/SL-S20-05.md`.

### SL-S20-06 — Integrar autosave editorial

- **Executar:** Usar fila do Core e estado claro de publicação durável.
- **Produzir:** `apps/desktop/src/piece/save.ts`.
- **Aceitar quando:** Fechar painel não destrói edição pendente; conflito externo usa o mesmo protocolo.
- **Controle:** Implementação; P1 na versão; depende de SL-S20-05; evidência em `reports/S20/SL-S20-06.md`.

### SL-S20-07 — Criar estados de produção

- **Executar:** Rascunho/finalizado ou estados aprovados, sem impor workflow excessivo.
- **Produzir:** `crates/piece/src/editorial_state.rs`.
- **Aceitar quando:** Estado é metadado canônico e não depende do índice de busca.
- **Controle:** Implementação; P1 na versão; depende de SL-S20-06; evidência em `reports/S20/SL-S20-07.md`.

### SL-S20-08 — Preparar área de copiloto

- **Executar:** Criar região de propostas desativada sem modelo, sem bloquear editor.
- **Produzir:** `apps/desktop/src/piece/copilot-shell.ts`.
- **Aceitar quando:** Ausência de IA mantém funcionalidade editorial completa.
- **Controle:** Implementação; P1 na versão; depende de SL-S20-07; evidência em `reports/S20/SL-S20-08.md`.

### SL-S20-09 — Testar documentos longos

- **Executar:** Abrir Peças com listas, tabelas e citações além da viewport.
- **Produzir:** `tests/piece/long-documents/`.
- **Aceitar quando:** Salvamento/exportação inclui todo o documento e não só blocos montados.
- **Controle:** Teste; P1 na versão; depende de SL-S20-08; evidência em `reports/S20/SL-S20-09.md`.

### SL-S20-10 — Validar edição simultânea de contextos

- **Executar:** Alternar foco Mesa/Peça e inspecionar revisões.
- **Produzir:** `reports/piece/focus-and-save.md`.
- **Aceitar quando:** Comando de uma área não modifica documento da outra.
- **Controle:** Teste; P1 na versão; depende de SL-S20-09; evidência em `reports/S20/SL-S20-10.md`.

### SL-S20-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Abrir Peça vinculada ao workspace e editar enquanto consulta células.**

Registrar commit, ambiente, testes e pendências em `reports/S20/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s21"></a>

## S21 — Implementar fork-on-insert e cadeia de evidências

- **Objetivo:** Preservar origem sem tornar a Peça dependente de conteúdo vivo de uma célula.
- **Componente / forma:** ProvenanceStore / Peça / Próprio.
- **Pré-requisitos técnicos:** S17 (Persistir topologia, grupos e relações); S20 (Construir editor da Peça e layout editorial)
- **Entregável do sprint:** Inserção independente com referência de revisão e localização.
- **Demonstração exigida:** Arrastar célula, editar a fonte e comprovar que a Peça conserva o trecho inserido.
- **Requisitos:** R02, R05, R13, R37. **Risco de integração:** Crítico.

### SL-S21-01 — Definir evento de inserção

- **Executar:** Registrar célula/revisão, fragmento, evidência e bloco de destino.
- **Produzir:** `schemas/cell-forked-into-piece.json`.
- **Aceitar quando:** Referência não se resume a ID+data e pode apontar ao texto efetivamente inserido.
- **Controle:** Contrato; P0 na versão; depende de SL-S17-GATE, SL-S20-GATE; evidência em `reports/S21/SL-S21-01.md`.

### SL-S21-02 — Capturar snapshot da seleção

- **Executar:** Resolver conteúdo na revisão exata antes de clonar.
- **Produzir:** `crates/provenance/src/snapshot.rs`.
- **Aceitar quando:** Edição concorrente da célula não altera o fragmento entre preview e confirmação.
- **Controle:** Implementação; P0 na versão; depende de SL-S21-01; evidência em `reports/S21/SL-S21-02.md`.

### SL-S21-03 — Clonar blocos com nova identidade

- **Executar:** Gerar IDs de destino e manter origem separada.
- **Produzir:** `crates/piece/src/fork_insert.rs`.
- **Aceitar quando:** Editar Peça não escreve de volta na célula e vice-versa.
- **Controle:** Implementação; P0 na versão; depende de SL-S21-02; evidência em `reports/S21/SL-S21-03.md`.

### SL-S21-04 — Persistir texto citado e hash

- **Executar:** Conservar trecho e localizador correspondente no formato aprovado.
- **Produzir:** `crates/provenance/src/quote.rs`.
- **Aceitar quando:** Texto citado pode ser verificado mesmo depois de renomear arquivo original.
- **Controle:** Implementação; P0 na versão; depende de SL-S21-03; evidência em `reports/S21/SL-S21-04.md`.

### SL-S21-05 — Encadear fonte primária

- **Executar:** Propagar referências de recorte até material/asset original quando existentes.
- **Produzir:** `crates/provenance/src/chain.rs`.
- **Aceitar quando:** Célula derivada não é apresentada como fonte primária independente.
- **Controle:** Implementação; P0 na versão; depende de SL-S21-04; evidência em `reports/S21/SL-S21-05.md`.

### SL-S21-06 — Renderizar citações clicáveis

- **Executar:** Navegar ao trecho/revisão, com estado para fonte ausente.
- **Produzir:** `apps/desktop/src/piece/citations.ts`.
- **Aceitar quando:** Clique não abre path externo sem autorização nem substitui silenciosamente revisão antiga.
- **Controle:** Implementação; P0 na versão; depende de SL-S21-05; evidência em `reports/S21/SL-S21-06.md`.

### SL-S21-07 — Tratar atualização da fonte

- **Executar:** Mostrar divergência e oferecer comparação, sem atualização automática da Peça.
- **Produzir:** `apps/desktop/src/piece/source-diff.ts`.
- **Aceitar quando:** Mudança posterior da fonte não modifica a citação histórica.
- **Controle:** Implementação; P0 na versão; depende de SL-S21-06; evidência em `reports/S21/SL-S21-07.md`.

### SL-S21-08 — Reverter inserção

- **Executar:** Desfazer blocos e vínculos do mesmo evento preservando histórico.
- **Produzir:** `crates/piece/src/undo_insert.rs`.
- **Aceitar quando:** Undo não remove citações/assets ainda usados por outro trecho.
- **Controle:** Implementação; P0 na versão; depende de SL-S21-07; evidência em `reports/S21/SL-S21-08.md`.

### SL-S21-09 — Testar cadeias e duplicações

- **Executar:** Inserir mesma célula várias vezes e citar recortes aninhados.
- **Produzir:** `tests/provenance/fork/`.
- **Aceitar quando:** Cada ocorrência possui destino próprio e origem inequívoca.
- **Controle:** Teste; P0 na versão; depende de SL-S21-08; evidência em `reports/S21/SL-S21-09.md`.

### SL-S21-10 — Verificar referências após reconstrução

- **Executar:** Apagar DB e caches editoriais e resolver todas as citações.
- **Produzir:** `reports/provenance/rebuild.json`.
- **Aceitar quando:** Cadeia é reconstruível exclusivamente dos arquivos e objetos canônicos.
- **Controle:** Teste; P0 na versão; depende de SL-S21-09; evidência em `reports/S21/SL-S21-10.md`.

### SL-S21-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Arrastar célula, editar a fonte e comprovar que a Peça conserva o trecho inserido.**

Registrar commit, ambiente, testes e pendências em `reports/S21/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s22"></a>

## S22 — Entregar produção local, histórico e exportação

- **Objetivo:** Fechar o primeiro ciclo de trabalho completo sem depender de IA ou web.
- **Componente / forma:** Peça / autoria completa / Próprio.
- **Pré-requisitos técnicos:** S20 (Construir editor da Peça e layout editorial); S21 (Implementar fork-on-insert e cadeia de evidências)
- **Entregável do sprint:** v0.4.0: notas → Mesa → Peça, histórico e exportação Markdown.
- **Demonstração exigida:** Produzir ensaio local, recuperar uma revisão e exportar texto com fontes.
- **Requisitos:** R02, R05, R07, R13, R29, R31. **Risco de integração:** Crítico.

### SL-S22-01 — Criar navegação entre Peças

- **Executar:** Listar por workspace e manter contexto de consulta à Mesa.
- **Produzir:** `apps/desktop/src/piece/library.ts`.
- **Aceitar quando:** Uma Peça não some quando seu painel é fechado ou o board é arquivado.
- **Controle:** Implementação; P0 na versão; depende de SL-S20-GATE, SL-S21-GATE; evidência em `reports/S22/SL-S22-01.md`.

### SL-S22-02 — Exibir histórico semântico

- **Executar:** Mostrar edições, inserções e autoria automática/humana.
- **Produzir:** `apps/desktop/src/history/`.
- **Aceitar quando:** Cada entrada resolve revisão válida e não depende exclusivamente de audit_log.db.
- **Controle:** Implementação; P0 na versão; depende de SL-S22-01; evidência em `reports/S22/SL-S22-02.md`.

### SL-S22-03 — Comparar revisões editoriais

- **Executar:** Mostrar mudanças de texto/metadados e permitir restauração como nova revisão.
- **Produzir:** `apps/desktop/src/piece/revision-diff.ts`.
- **Aceitar quando:** Restaurar não apaga a linha temporal nem referências necessárias.
- **Controle:** Implementação; P0 na versão; depende de SL-S22-02; evidência em `reports/S22/SL-S22-03.md`.

### SL-S22-04 — Exportar Markdown portátil

- **Executar:** Resolver caminhos relativos e incluir referências/manifest conforme perfil.
- **Produzir:** `crates/export/src/markdown.rs`.
- **Aceitar quando:** Arquivo exportado abre em editor comum e preserva texto/citações suportados.
- **Controle:** Implementação; P0 na versão; depende de SL-S22-03; evidência em `reports/S22/SL-S22-04.md`.

### SL-S22-05 — Exportar pacote de pesquisa

- **Executar:** Agrupar Peça, fontes autorizadas e mapa de proveniência opcional.
- **Produzir:** `crates/export/src/research_bundle.rs`.
- **Aceitar quando:** Export respeita escopo e não inclui todo o Vault por conveniência.
- **Controle:** Implementação; P0 na versão; depende de SL-S22-04; evidência em `reports/S22/SL-S22-05.md`.

### SL-S22-06 — Criar snapshot de backup local

- **Executar:** Gerar cópia consistente com manifest e hashes sem rotina remota.
- **Produzir:** `crates/backup/src/snapshot.rs`.
- **Aceitar quando:** Snapshot valida após alterações posteriores no Vault ativo.
- **Controle:** Implementação; P0 na versão; depende de SL-S22-05; evidência em `reports/S22/SL-S22-06.md`.

### SL-S22-07 — Criar restauração de ensaio

- **Executar:** Restaurar em diretório novo com confirmação e validação.
- **Produzir:** `crates/backup/src/restore.rs`.
- **Aceitar quando:** Nenhum arquivo existente é sobrescrito silenciosamente.
- **Controle:** Implementação; P0 na versão; depende de SL-S22-06; evidência em `reports/S22/SL-S22-07.md`.

### SL-S22-08 — Validar ciclo integral offline

- **Executar:** Capturar nota manual, organizar, inserir na Peça, revisar e exportar com rede bloqueada.
- **Produzir:** `reports/v0.4/offline-journey.md`.
- **Aceitar quando:** Todos os passos essenciais funcionam sem credenciais/modelos.
- **Controle:** Teste; P0 na versão; depende de SL-S22-07; evidência em `reports/S22/SL-S22-08.md`.

### SL-S22-09 — Executar recuperação de autoria

- **Executar:** Interromper durante inserção e restauração, depois reconstruir caches.
- **Produzir:** `tests/piece/recovery/`.
- **Aceitar quando:** Operação fica anterior ou recuperável, nunca com texto e citação divergentes sem detecção.
- **Controle:** Teste; P0 na versão; depende de SL-S22-08; evidência em `reports/S22/SL-S22-09.md`.

### SL-S22-10 — Publicar alpha funcional local

- **Executar:** Documentar limitações e preparar pacote com evidências.
- **Produzir:** `release/v0.4.0/`.
- **Aceitar quando:** Mantenedor consegue usar o app para trabalho real sem habilitar funcionalidades ainda não seguras.
- **Controle:** Release; P0 na versão; depende de SL-S22-09; evidência em `reports/S22/SL-S22-10.md`.

### SL-S22-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Produzir ensaio local, recuperar uma revisão e exportar texto com fontes.**

Registrar commit, ambiente, testes e pendências em `reports/S22/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

# v0.5.0 — Ingestão segura

**Maturidade:** Alpha de acervo  
**Entrega acumulada:** Capturar arquivos/HTML, extrair texto e OCR em workers confinados e organizar o acervo.

<a id="s23"></a>

## S23 — Construir protocolo e supervisor de workers

- **Objetivo:** Permitir motores poliglotas sem conceder autoridade sobre o Vault.
- **Componente / forma:** JobScheduler / WorkerHost / Próprio.
- **Pré-requisitos técnicos:** S09 (Implementar broker de arquivos e defesa TOCTOU); S11 (Concluir v0.1: recuperação e diagnóstico do Vault); S22 (Entregar produção local, histórico e exportação)
- **Entregável do sprint:** WorkerHost, fila persistível e protocolo versionado.
- **Demonstração exigida:** Worker de teste recebe snapshot autorizado, falha/reinicia e devolve proposta validada.
- **Requisitos:** R03, R05, R06, R08, R09. **Risco de integração:** Crítico.

### SL-S23-01 — Aprovar protocolo e isolamento

- **Executar:** Resolver ADR-009; selecionar pipes/sockets/API local e escopo por tarefa.
- **Produzir:** `docs/adr/009-workers.md`.
- **Aceitar quando:** Processo auxiliar não compartilha implicitamente todos os direitos do Core.
- **Controle:** Decisão; P0 na versão; depende de SL-S09-GATE, SL-S11-GATE, SL-S22-GATE; evidência em `reports/S23/SL-S23-01.md`.

### SL-S23-02 — Definir envelope de job

- **Executar:** Incluir IDs, revisão, input_refs, capacidade, orçamento, deadline e cancelamento.
- **Produzir:** `schemas/job-v0.json`.
- **Aceitar quando:** Job de revisão antiga é distinguível; payload inválido não inicia worker.
- **Controle:** Contrato; P0 na versão; depende de SL-S23-01; evidência em `reports/S23/SL-S23-02.md`.

### SL-S23-03 — Implementar fila e estados

- **Executar:** Planejar pendente, em execução, retry, cancelado, bloqueado e concluído.
- **Produzir:** `crates/jobs/src/state.rs`.
- **Aceitar quando:** Reinício não duplica efeitos nem perde captura associada ao job.
- **Controle:** Implementação; P0 na versão; depende de SL-S23-02; evidência em `reports/S23/SL-S23-03.md`.

### SL-S23-04 — Criar transporte local

- **Executar:** Fazer handshake, versão, limites de mensagem e identificação do processo.
- **Produzir:** `crates/workers/src/transport.rs`.
- **Aceitar quando:** Worker incompatível é recusado antes de receber dados.
- **Controle:** Implementação; P0 na versão; depende de SL-S23-03; evidência em `reports/S23/SL-S23-04.md`.

### SL-S23-05 — Provisionar entradas restritas

- **Executar:** Entregar handles/cópias snapshot e diretório temporário exclusivo.
- **Produzir:** `crates/workers/src/inputs.rs`.
- **Aceitar quando:** Worker não ganha o path do Vault inteiro como atalho.
- **Controle:** Implementação; P0 na versão; depende de SL-S23-04; evidência em `reports/S23/SL-S23-05.md`.

### SL-S23-06 — Validar resultados

- **Executar:** Checar esquema, hashes, revisão e destino antes de aceitar proposta.
- **Produzir:** `crates/workers/src/results.rs`.
- **Aceitar quando:** Resultado atrasado/malformado não publica conteúdo sobre revisão nova.
- **Controle:** Implementação; P0 na versão; depende de SL-S23-05; evidência em `reports/S23/SL-S23-06.md`.

### SL-S23-07 — Implementar cancelamento

- **Executar:** Encerrar tarefa, limpar temporários e controlar descendentes.
- **Produzir:** `crates/workers/src/cancel.rs`.
- **Aceitar quando:** Cancelamento não apaga originais nem deixa processo órfão conhecido.
- **Controle:** Implementação; P0 na versão; depende de SL-S23-06; evidência em `reports/S23/SL-S23-07.md`.

### SL-S23-08 — Implementar quotas

- **Executar:** Aplicar limites de CPU/memória/bytes/tempo por perfil e registrar consumo.
- **Produzir:** `crates/workers/src/budget.rs`.
- **Aceitar quando:** Worker que excede perfil falha de modo recuperável com causa identificada.
- **Controle:** Implementação; P0 na versão; depende de SL-S23-07; evidência em `reports/S23/SL-S23-08.md`.

### SL-S23-09 — Criar backoff/idempotência

- **Executar:** Repetir apenas erros recuperáveis, com limite e chave de tarefa.
- **Produzir:** `crates/jobs/src/retry.rs`.
- **Aceitar quando:** Retry não duplica documentos, cobrança autorizada ou eventos autorais.
- **Controle:** Implementação; P0 na versão; depende de SL-S23-08; evidência em `reports/S23/SL-S23-09.md`.

### SL-S23-10 — Simular worker hostil

- **Executar:** Tentar path extra, resposta excessiva, processo filho e resposta fora de ordem.
- **Produzir:** `tests/workers/protocol/`.
- **Aceitar quando:** Harness demonstra rejeição e nenhuma mutação canônica não autorizada.
- **Controle:** Teste; P0 na versão; depende de SL-S23-09; evidência em `reports/S23/SL-S23-10.md`.

### SL-S23-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Worker de teste recebe snapshot autorizado, falha/reinicia e devolve proposta validada.**

Registrar commit, ambiente, testes e pendências em `reports/S23/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s24"></a>

## S24 — Implementar perfis de sandbox Linux

- **Objetivo:** Conter parsing, inferência e browser por capacidades reais do kernel.
- **Componente / forma:** SandboxManager / Linux / Próprio.
- **Pré-requisitos técnicos:** S23 (Construir protocolo e supervisor de workers)
- **Entregável do sprint:** Perfis Linux e matriz de disponibilidade de segurança.
- **Demonstração exigida:** Worker sem rede não lê sentinela fora do escopo nem cria conexão externa.
- **Requisitos:** R01, R06, R08, R09. **Risco de integração:** Crítico.

### SL-S24-01 — Detectar recursos do kernel

- **Executar:** Verificar Landlock/ABI, seccomp e primitives necessárias antes de lançar job.
- **Produzir:** `crates/sandbox/src/linux/probe.rs`.
- **Aceitar quando:** Ausência de recurso obrigatório bloqueia a tarefa com explicação.
- **Controle:** Implementação; P0 na versão; depende de SL-S23-GATE; evidência em `reports/S24/SL-S24-01.md`.

### SL-S24-02 — Definir perfis mínimos

- **Executar:** Separar parser, inferência CPU/GPU e aquisição web.
- **Produzir:** `sandbox/linux/profiles/`.
- **Aceitar quando:** Um perfil com rede não é reaproveitado implicitamente para leitura de documentos privados.
- **Controle:** Implementação; P0 na versão; depende de SL-S24-01; evidência em `reports/S24/SL-S24-02.md`.

### SL-S24-03 — Aplicar restrição filesystem

- **Executar:** Conceder leitura de entradas/modelos/libs e escrita apenas temporária.
- **Produzir:** `crates/sandbox/src/linux/fs.rs`.
- **Aceitar quando:** Diretório de outro workspace permanece inacessível mesmo com path conhecido.
- **Controle:** Implementação; P0 na versão; depende de SL-S24-02; evidência em `reports/S24/SL-S24-03.md`.

### SL-S24-04 — Aplicar política de syscalls

- **Executar:** Reduzir superfície conforme perfil sem tratar Landlock como substituto de seccomp.
- **Produzir:** `crates/sandbox/src/linux/syscalls.rs`.
- **Aceitar quando:** Testes negativos de execução/processos e positivos das funções homologadas passam.
- **Controle:** Implementação; P0 na versão; depende de SL-S24-03; evidência em `reports/S24/SL-S24-04.md`.

### SL-S24-05 — Conter rede

- **Executar:** Bloquear saída onde exigido e definir fronteira de broker para casos autorizados.
- **Produzir:** `crates/sandbox/src/linux/network.rs`.
- **Aceitar quando:** Worker local não pode contornar EgressPolicy usando sua própria biblioteca HTTP.
- **Controle:** Implementação; P0 na versão; depende de SL-S24-04; evidência em `reports/S24/SL-S24-05.md`.

### SL-S24-06 — Controlar árvore de processos

- **Executar:** Registrar descendentes, limites e encerramento em falha.
- **Produzir:** `crates/sandbox/src/linux/process_tree.rs`.
- **Aceitar quando:** Matar tarefa também encerra filhos autorizados e não concede criação ilimitada.
- **Controle:** Implementação; P0 na versão; depende de SL-S24-05; evidência em `reports/S24/SL-S24-06.md`.

### SL-S24-07 — Tratar GPU explicitamente

- **Executar:** Documentar devices/driver necessários e negar acessos não necessários.
- **Produzir:** `sandbox/linux/gpu-capabilities.yaml`.
- **Aceitar quando:** Perfil CPU funciona sem GPU; perfil GPU não recebe filesystem geral por conveniência.
- **Controle:** Implementação; P0 na versão; depende de SL-S24-06; evidência em `reports/S24/SL-S24-07.md`.

### SL-S24-08 — Exercitar escapes de filesystem

- **Executar:** Cobrir links, mounts disponíveis, descriptors herdados e temporários compartilhados.
- **Produzir:** `tests/sandbox/linux/fs/`.
- **Aceitar quando:** Sentinelas externas não são lidas/escritas; limitações são declaradas e bloqueadoras quando aplicável.
- **Controle:** Teste; P0 na versão; depende de SL-S24-07; evidência em `reports/S24/SL-S24-08.md`.

### SL-S24-09 — Exercitar rede/filhos

- **Executar:** Usar serviços de teste locais e processo descendente tentando sair do perfil.
- **Produzir:** `tests/sandbox/linux/process-network/`.
- **Aceitar quando:** Proibições também valem para descendentes e não só para o PID inicial.
- **Controle:** Teste; P0 na versão; depende de SL-S24-08; evidência em `reports/S24/SL-S24-09.md`.

### SL-S24-10 — Publicar matriz Linux

- **Executar:** Registrar kernel/arquitetura, capacidades e comportamentos desabilitados.
- **Produzir:** `reports/sandbox/linux-matrix.md`.
- **Aceitar quando:** Não se alega proteção idêntica em kernels não testados.
- **Controle:** Documentação; P0 na versão; depende de SL-S24-09; evidência em `reports/S24/SL-S24-10.md`.

### SL-S24-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Worker sem rede não lê sentinela fora do escopo nem cria conexão externa.**

Registrar commit, ambiente, testes e pendências em `reports/S24/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s25"></a>

## S25 — Implementar perfis de sandbox macOS

- **Objetivo:** Validar isolamento macOS sem depender de sandbox-exec como promessa futura.
- **Componente / forma:** SandboxManager / macOS / Próprio.
- **Pré-requisitos técnicos:** S23 (Construir protocolo e supervisor de workers)
- **Entregável do sprint:** Helper/perfis macOS e evidências nativas Intel/Apple Silicon conforme disponibilidade.
- **Demonstração exigida:** Processo auxiliar autorizado lê a entrada e falha ao acessar outro workspace ou rede proibida.
- **Requisitos:** R01, R06, R08, R09, R30. **Risco de integração:** Crítico.

### SL-S25-01 — Escolher mecanismo suportável

- **Executar:** Avaliar App Sandbox/helpers/XPC, assinatura e distribuição; registrar limites.
- **Produzir:** `docs/security/macos-sandbox-design.md`.
- **Aceitar quando:** Decisão não apresenta sandbox-exec deprecated como solução garantida de longo prazo.
- **Controle:** Spike; P0 na versão; depende de SL-S23-GATE; evidência em `reports/S25/SL-S25-01.md`.

### SL-S25-02 — Criar helper restrito

- **Executar:** Separar runtime confiável da comunicação com processos de parsing/inferência.
- **Produzir:** `platform/macos/helper/`.
- **Aceitar quando:** Helper não herda toda a API Tauri nem credenciais do processo principal.
- **Controle:** Implementação; P0 na versão; depende de SL-S25-01; evidência em `reports/S25/SL-S25-02.md`.

### SL-S25-03 — Definir entitlements

- **Executar:** Conceder somente arquivos, comunicação e recursos necessários por perfil.
- **Produzir:** `platform/macos/entitlements/`.
- **Aceitar quando:** Diferenças parser/inferência/web são visíveis e revisadas.
- **Controle:** Implementação; P0 na versão; depende de SL-S25-02; evidência em `reports/S25/SL-S25-03.md`.

### SL-S25-04 — Transferir entradas por capability

- **Executar:** Resolver documentos selecionados e fontes aprovadas por broker.
- **Produzir:** `crates/sandbox/src/macos/inputs.rs`.
- **Aceitar quando:** Escopo do workspace não vira acesso geral ao diretório pessoal.
- **Controle:** Implementação; P0 na versão; depende de SL-S25-03; evidência em `reports/S25/SL-S25-04.md`.

### SL-S25-05 — Restringir saídas temporárias

- **Executar:** Criar diretórios por tarefa e validar resultados no Core.
- **Produzir:** `crates/sandbox/src/macos/output.rs`.
- **Aceitar quando:** Worker não publica diretamente nos arquivos Markdown canônicos.
- **Controle:** Implementação; P0 na versão; depende de SL-S25-04; evidência em `reports/S25/SL-S25-05.md`.

### SL-S25-06 — Controlar rede/IPC

- **Executar:** Separar conexão necessária ao runtime local de saída externa.
- **Produzir:** `platform/macos/policies/`.
- **Aceitar quando:** Modelo local não possui capacidade de egress externo não autorizada.
- **Controle:** Implementação; P0 na versão; depende de SL-S25-05; evidência em `reports/S25/SL-S25-06.md`.

### SL-S25-07 — Integrar ciclo de vida

- **Executar:** Cancelar helpers e backends com cleanup e accounting.
- **Produzir:** `crates/sandbox/src/macos/lifecycle.rs`.
- **Aceitar quando:** Encerramento da tarefa não deixa runtime invisível consumindo recursos.
- **Controle:** Implementação; P0 na versão; depende de SL-S25-06; evidência em `reports/S25/SL-S25-07.md`.

### SL-S25-08 — Executar adversarial nativo

- **Executar:** Testar leitura/gravação externa, redes e propagação a filhos.
- **Produzir:** `tests/sandbox/macos/`.
- **Aceitar quando:** Aprovação requer resultado em macOS real/ambiente válido, não apenas compilação cross-target.
- **Controle:** Teste; P0 na versão; depende de SL-S25-07; evidência em `reports/S25/SL-S25-08.md`.

### SL-S25-09 — Validar empacotamento de helper

- **Executar:** Testar assinatura/permissões e máquina sem ferramentas de desenvolvimento.
- **Produzir:** `reports/sandbox/macos-package.md`.
- **Aceitar quando:** Pacote inicia com privilégios documentados e não exige desativar proteção do sistema silenciosamente.
- **Controle:** Teste; P0 na versão; depende de SL-S25-08; evidência em `reports/S25/SL-S25-09.md`.

### SL-S25-10 — Registrar suporte e bloqueios

- **Executar:** Distinguir capacidades homologadas por arquitetura e perfil.
- **Produzir:** `reports/sandbox/macos-matrix.md`.
- **Aceitar quando:** Função não confinada permanece desabilitada até solução ou revisão explícita de requisito.
- **Controle:** Decisão; P0 na versão; depende de SL-S25-09; evidência em `reports/S25/SL-S25-10.md`.

### SL-S25-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Processo auxiliar autorizado lê a entrada e falha ao acessar outro workspace ou rede proibida.**

Registrar commit, ambiente, testes e pendências em `reports/S25/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s26"></a>

## S26 — Implementar perfis de sandbox Windows

- **Objetivo:** Combinar contenção efetiva com controle de recursos e descendentes.
- **Componente / forma:** SandboxManager / Windows / Próprio.
- **Pré-requisitos técnicos:** S23 (Construir protocolo e supervisor de workers)
- **Entregável do sprint:** Perfis Windows com AppContainer/capabilities ou alternativa auditada.
- **Demonstração exigida:** Worker controlado não acessa paths/rede fora do permitido e toda sua árvore encerra.
- **Requisitos:** R01, R06, R08, R09, R30. **Risco de integração:** Crítico.

### SL-S26-01 — Definir fronteira Windows

- **Executar:** Avaliar AppContainer, tokens, ACLs e broker para motores selecionados.
- **Produzir:** `docs/security/windows-sandbox-design.md`.
- **Aceitar quando:** Job Objects não são apresentados como única barreira de arquivos/rede.
- **Controle:** Spike; P0 na versão; depende de SL-S23-GATE; evidência em `reports/S26/SL-S26-01.md`.

### SL-S26-02 — Criar identidade restrita

- **Executar:** Configurar contexto de segurança por worker sem herdar permissões gerais do usuário.
- **Produzir:** `crates/sandbox/src/windows/identity.rs`.
- **Aceitar quando:** Remover admin não é considerado suficiente; acesso efetivo é testado.
- **Controle:** Implementação; P0 na versão; depende de SL-S26-01; evidência em `reports/S26/SL-S26-02.md`.

### SL-S26-03 — Conceder entradas explicitamente

- **Executar:** Aplicar ACL/capability/handles necessários aos snapshots e modelos.
- **Produzir:** `crates/sandbox/src/windows/inputs.rs`.
- **Aceitar quando:** Outro workspace e segredos do app continuam negados.
- **Controle:** Implementação; P0 na versão; depende de SL-S26-02; evidência em `reports/S26/SL-S26-03.md`.

### SL-S26-04 — Conter filesystem

- **Executar:** Tratar reparse points, paths especiais e temporários dentro do perfil.
- **Produzir:** `crates/sandbox/src/windows/fs.rs`.
- **Aceitar quando:** Reparse point preparado não redireciona escrita para fora do destino autorizado.
- **Controle:** Implementação; P0 na versão; depende de SL-S26-03; evidência em `reports/S26/SL-S26-04.md`.

### SL-S26-05 — Conter rede

- **Executar:** Definir proibições/capacidades efetivas, inclusive comunicação local necessária.
- **Produzir:** `crates/sandbox/src/windows/network.rs`.
- **Aceitar quando:** Worker offline não acessa serviço externo; não há dependência oculta de privilégio admin para cada job.
- **Controle:** Implementação; P0 na versão; depende de SL-S26-04; evidência em `reports/S26/SL-S26-05.md`.

### SL-S26-06 — Usar Job Objects para lifecycle

- **Executar:** Limitar recursos e agrupar descendentes sem impedir indiscriminadamente browser/backends legítimos.
- **Produzir:** `crates/sandbox/src/windows/jobs.rs`.
- **Aceitar quando:** Perfil browser admite somente árvore esperada; cancelamento encerra todos os processos do job.
- **Controle:** Implementação; P0 na versão; depende de SL-S26-05; evidência em `reports/S26/SL-S26-06.md`.

### SL-S26-07 — Controlar DLLs e executáveis

- **Executar:** Resolver binários em diretório gerenciado e verificar origem.
- **Produzir:** `crates/sandbox/src/windows/loader.rs`.
- **Aceitar quando:** PATH e diretório de trabalho hostis não substituem biblioteca/backend autorizado.
- **Controle:** Implementação; P0 na versão; depende de SL-S26-06; evidência em `reports/S26/SL-S26-07.md`.

### SL-S26-08 — Executar corpus adversarial

- **Executar:** Cobrir rede, privilégios, arquivos, junctions e criação de processos.
- **Produzir:** `tests/sandbox/windows/`.
- **Aceitar quando:** Evidência demonstra negação real em Windows, não apenas retorno esperado de mock.
- **Controle:** Teste; P0 na versão; depende de SL-S26-07; evidência em `reports/S26/SL-S26-08.md`.

### SL-S26-09 — Testar instalação sem admin

- **Executar:** Instalar perfil suportado, executar job e desinstalar sem resíduos privilegiados.
- **Produzir:** `reports/sandbox/windows-install.md`.
- **Aceitar quando:** Requisitos de privilégio são explícitos e funções inseguras falham fechadas.
- **Controle:** Teste; P0 na versão; depende de SL-S26-08; evidência em `reports/S26/SL-S26-09.md`.

### SL-S26-10 — Consolidar matriz multiplataforma

- **Executar:** Comparar garantias Linux/macOS/Windows e registrar capacidades desabilitadas.
- **Produzir:** `reports/sandbox/capability-matrix.json`.
- **Aceitar quando:** Jobs futuros só são habilitados onde o perfil necessário foi aprovado.
- **Controle:** Decisão; P0 na versão; depende de SL-S26-09; evidência em `reports/S26/SL-S26-10.md`.

### SL-S26-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Worker controlado não acessa paths/rede fora do permitido e toda sua árvore encerra.**

Registrar commit, ambiente, testes e pendências em `reports/S26/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s27"></a>

## S27 — Desconstruir Docling e integrar extração textual

- **Objetivo:** Reutilizar parsing sem tornar o worker responsável pelo formato ou armazenamento Sandland.
- **Componente / forma:** Docling / ExtractProvider / Desconstrução.
- **Pré-requisitos técnicos:** S23 (Construir protocolo e supervisor de workers); S24 (Implementar perfis de sandbox Linux); S25 (Implementar perfis de sandbox macOS); S26 (Implementar perfis de sandbox Windows)
- **Entregável do sprint:** Worker Docling mínimo para documentos textuais.
- **Demonstração exigida:** Importar PDF/DOCX de fixture e obter blocos com referências ao original.
- **Requisitos:** R14, R15, R30, R33, R34. **Risco de integração:** Crítico.

### SL-S27-01 — Fixar revisão Docling

- **Executar:** Registrar versão, deps, modelos necessários e licenças de cada artefato.
- **Produzir:** `upstream/docling/manifest.json`.
- **Aceitar quando:** Nenhum peso é baixado automaticamente sem passar pelo catálogo aprovado.
- **Controle:** Upstream; P0 na versão; depende de SL-S23-GATE, SL-S24-GATE, SL-S25-GATE, SL-S26-GATE; evidência em `reports/S27/SL-S27-01.md`.

### SL-S27-02 — Reproduzir baseline de parsing

- **Executar:** Converter fixtures textuais upstream antes de criar o adapter.
- **Produzir:** `reports/docling/baseline.md`.
- **Aceitar quando:** Resultado original, versões e limitações estão registrados para comparação diferencial.
- **Controle:** Upstream; P0 na versão; depende de SL-S27-01; evidência em `reports/S27/SL-S27-02.md`.

### SL-S27-03 — Recortar pipelines

- **Executar:** Selecionar parsing textual/tabelas; deixar OCR/VLM e servidor opcional fora do caminho leve.
- **Produzir:** `workers/docling/pipeline-profile.yaml`.
- **Aceitar quando:** Importação simples não carrega todo o catálogo de modelos ou serviços.
- **Controle:** Upstream; P0 na versão; depende de SL-S27-02; evidência em `reports/S27/SL-S27-03.md`.

### SL-S27-04 — Definir NormalizedDocument

- **Executar:** Mapear blocos, hierarquia, páginas, tabelas, assets e diagnósticos.
- **Produzir:** `schemas/normalized-document-v0.json`.
- **Aceitar quando:** Contrato representa extração parcial e localizadores sem fingir cobertura completa.
- **Controle:** Contrato; P0 na versão; depende de SL-S27-03; evidência em `reports/S27/SL-S27-04.md`.

### SL-S27-05 — Criar adapter Docling

- **Executar:** Converter estruturas da revisão fixada ao contrato, sem imports internos espalhados.
- **Produzir:** `workers/docling/adapter.py`.
- **Aceitar quando:** Mudança de versão upstream é confinada ao adapter e detectada por testes.
- **Controle:** Implementação; P0 na versão; depende de SL-S27-04; evidência em `reports/S27/SL-S27-05.md`.

### SL-S27-06 — Preservar fonte e localizadores

- **Executar:** Relacionar conteúdo extraído ao asset/hash/página/bloco.
- **Produzir:** `workers/docling/provenance.py`.
- **Aceitar quando:** Trecho exibido pode ser rastreado ao documento original da revisão correta.
- **Controle:** Implementação; P0 na versão; depende de SL-S27-05; evidência em `reports/S27/SL-S27-06.md`.

### SL-S27-07 — Integrar worker ao scheduler

- **Executar:** Receber entrada autorizada e devolver resultados por IPC.
- **Produzir:** `workers/docling/entrypoint.py`.
- **Aceitar quando:** Worker não escreve no Vault nem cria sua própria base canônica.
- **Controle:** Implementação; P0 na versão; depende de SL-S27-06; evidência em `reports/S27/SL-S27-07.md`.

### SL-S27-08 — Tratar erros documentais

- **Executar:** Classificar formato não suportado, criptografado, corrompido e extração incompleta.
- **Produzir:** `workers/docling/errors.py`.
- **Aceitar quando:** Captura original permanece preservada e pode ser reprocessada.
- **Controle:** Implementação; P0 na versão; depende de SL-S27-07; evidência em `reports/S27/SL-S27-08.md`.

### SL-S27-09 — Comparar baseline e adapter

- **Executar:** Verificar perdas de texto/estrutura e schema em corpus de avaliação.
- **Produzir:** `tests/extract/docling-text/`.
- **Aceitar quando:** Diferenças aceitas são documentadas; não há regressão silenciosa de tabelas/fontes.
- **Controle:** Teste; P0 na versão; depende de SL-S27-08; evidência em `reports/S27/SL-S27-09.md`.

### SL-S27-10 — Medir processo leve

- **Executar:** Medir startup, pico, cancelamento e retorno ao estado sem worker.
- **Produzir:** `reports/docling/text-resources.json`.
- **Aceitar quando:** Orçamento do perfil é observado ou a integração é bloqueada/replanejada explicitamente.
- **Controle:** Teste; P0 na versão; depende de SL-S27-09; evidência em `reports/S27/SL-S27-10.md`.

### SL-S27-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Importar PDF/DOCX de fixture e obter blocos com referências ao original.**

Registrar commit, ambiente, testes e pendências em `reports/S27/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s28"></a>

## S28 — Adicionar OCR e robustez da extração

- **Objetivo:** Expandir o processamento sem prometer que todo OCR cabe no perfil leve.
- **Componente / forma:** Docling / OCR / Adaptação.
- **Pré-requisitos técnicos:** S27 (Desconstruir Docling e integrar extração textual)
- **Entregável do sprint:** Pipeline OCR homologado e processamento recuperável por documento.
- **Demonstração exigida:** Importar PDF escaneado, cancelar, retomar e citar texto com localização.
- **Requisitos:** R09, R14, R15, R30. **Risco de integração:** Crítico.

### SL-S28-01 — Selecionar perfil OCR

- **Executar:** Comparar alternativas disponíveis na revisão fixada, idiomas, memória e licenças.
- **Produzir:** `docs/models/ocr-profile.md`.
- **Aceitar quando:** Perfil aprovado explicita hardware mínimo e não aumenta teto de RAM silenciosamente.
- **Controle:** Decisão; P0 na versão; depende de SL-S27-GATE; evidência em `reports/S28/SL-S28-01.md`.

### SL-S28-02 — Provisionar artefatos OCR

- **Executar:** Instalar por manifest/hash e permitir uso com rede bloqueada depois do provisionamento.
- **Produzir:** `workers/docling/ocr-manifest.json`.
- **Aceitar quando:** Artefato faltante gera tarefa de instalação consentida, não download oculto durante parsing.
- **Controle:** Implementação; P0 na versão; depende de SL-S28-01; evidência em `reports/S28/SL-S28-02.md`.

### SL-S28-03 — Detectar necessidade de OCR

- **Executar:** Distinguir camada textual suficiente, páginas imagem e extração suspeita.
- **Produzir:** `workers/docling/ocr-routing.py`.
- **Aceitar quando:** Documento textual comum evita custo OCR; classificação incorreta pode ser corrigida pelo usuário.
- **Controle:** Implementação; P0 na versão; depende de SL-S28-02; evidência em `reports/S28/SL-S28-03.md`.

### SL-S28-04 — Processar com limites

- **Executar:** Definir unidade/paginação e concorrência permitida pelo motor sem alterar semântica.
- **Produzir:** `workers/docling/ocr-jobs.py`.
- **Aceitar quando:** Documento grande não cresce sem limite até derrubar o Core.
- **Controle:** Implementação; P0 na versão; depende de SL-S28-03; evidência em `reports/S28/SL-S28-04.md`.

### SL-S28-05 — Preservar coordenadas

- **Executar:** Associar OCR e tabelas à página/asset original com status de precisão.
- **Produzir:** `workers/docling/ocr-locators.py`.
- **Aceitar quando:** Texto extraído não inventa coordenada ou evidência inexistente quando o backend não a fornece.
- **Controle:** Implementação; P0 na versão; depende de SL-S28-04; evidência em `reports/S28/SL-S28-05.md`.

### SL-S28-06 — Registrar checkpoints

- **Executar:** Retomar unidades concluídas quando a API/pipeline escolhido permitir.
- **Produzir:** `crates/ingest/src/extraction_checkpoint.rs`.
- **Aceitar quando:** Retry não duplica blocos e fallback de reprocessamento completo é explícito quando necessário.
- **Controle:** Implementação; P0 na versão; depende de SL-S28-05; evidência em `reports/S28/SL-S28-06.md`.

### SL-S28-07 — Expor qualidade/limitações

- **Executar:** Marcar baixa legibilidade, leitura parcial e páginas não processadas.
- **Produzir:** `apps/desktop/src/ingest/extraction-quality.ts`.
- **Aceitar quando:** Resumo futuro consegue distinguir documento completo de conteúdo parcialmente extraído.
- **Controle:** Implementação; P0 na versão; depende de SL-S28-06; evidência em `reports/S28/SL-S28-07.md`.

### SL-S28-08 — Testar corpus visual difícil

- **Executar:** Cobrir colunas, rotação, tabelas, imagens e diferentes idiomas.
- **Produzir:** `tests/extract/ocr/`.
- **Aceitar quando:** Erros conhecidos são mensurados e não mascarados como compreensão visual perfeita.
- **Controle:** Teste; P0 na versão; depende de SL-S28-07; evidência em `reports/S28/SL-S28-08.md`.

### SL-S28-09 — Testar limites e cancelamento

- **Executar:** Simular timeout/OOM e interrupção após algumas páginas.
- **Produzir:** `reports/docling/ocr-failure.json`.
- **Aceitar quando:** App preserva original, encerra worker e fornece reprocessamento controlado.
- **Controle:** Teste; P0 na versão; depende de SL-S28-08; evidência em `reports/S28/SL-S28-09.md`.

### SL-S28-10 — Publicar capacidades por perfil

- **Executar:** Relacionar formatos, idiomas, limites e recursos necessários.
- **Produzir:** `docs/user/document-extraction.md`.
- **Aceitar quando:** Usuário sabe quando OCR está indisponível ou requer pacote adicional.
- **Controle:** Documentação; P0 na versão; depende de SL-S28-09; evidência em `reports/S28/SL-S28-10.md`.

### SL-S28-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Importar PDF escaneado, cancelar, retomar e citar texto com localização.**

Registrar commit, ambiente, testes e pendências em `reports/S28/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s29"></a>

## S29 — Integrar Readability/Turndown para clipping seguro

- **Objetivo:** Normalizar HTML sem renderizar páginas hostis na janela privilegiada.
- **Componente / forma:** Readability / Turndown / Desconstrução.
- **Pré-requisitos técnicos:** S23 (Construir protocolo e supervisor de workers); S24 (Implementar perfis de sandbox Linux); S25 (Implementar perfis de sandbox macOS); S26 (Implementar perfis de sandbox Windows)
- **Entregável do sprint:** Normalizador HTML→documento Sandland e captura manual de HTML.
- **Demonstração exigida:** Processar página de fixture com scripts/trackers sem executar scripts ou acessar rede.
- **Requisitos:** R08, R14, R17, R30. **Risco de integração:** Crítico.

### SL-S29-01 — Fixar bibliotecas e baseline

- **Executar:** Registrar versões/licenças e normalizar corpus de artigos antes de customizar.
- **Produzir:** `upstream/web-normalizer/manifest.json`.
- **Aceitar quando:** Saída upstream e erros conhecidos ficam disponíveis para comparação.
- **Controle:** Upstream; P0 na versão; depende de SL-S23-GATE, SL-S24-GATE, SL-S25-GATE, SL-S26-GATE; evidência em `reports/S29/SL-S29-01.md`.

### SL-S29-02 — Escolher ambiente DOM isolado

- **Executar:** Definir renderer/helper sem IPC privilegiado e sem recursos remotos automáticos.
- **Produzir:** `docs/architecture/html-normalizer.md`.
- **Aceitar quando:** Plano não presume DOM completo em Web Worker comum nem usa janela principal como parser hostil.
- **Controle:** Decisão; P0 na versão; depende de SL-S29-01; evidência em `reports/S29/SL-S29-02.md`.

### SL-S29-03 — Construir entrada inerte

- **Executar:** Aplicar limites de tamanho, encoding e tratamento de URL base.
- **Produzir:** `workers/html-normalizer/input.ts`.
- **Aceitar quando:** HTML malformado não escapa para navegação, scripts ou resolução arbitrária de arquivos.
- **Controle:** Implementação; P0 na versão; depende de SL-S29-02; evidência em `reports/S29/SL-S29-03.md`.

### SL-S29-04 — Integrar Readability

- **Executar:** Extrair conteúdo principal e metadata mantendo diagnóstico de falha.
- **Produzir:** `workers/html-normalizer/readability.ts`.
- **Aceitar quando:** Página não reconhecida não é substituída por texto vazio como se captura tivesse sucesso.
- **Controle:** Implementação; P0 na versão; depende de SL-S29-03; evidência em `reports/S29/SL-S29-04.md`.

### SL-S29-05 — Integrar Turndown

- **Executar:** Aplicar regras aprovadas para títulos, listas, tabelas, código e links.
- **Produzir:** `workers/html-normalizer/turndown.ts`.
- **Aceitar quando:** Conversão preserva conteúdo do perfil e usa Markdown válido em fixtures.
- **Controle:** Implementação; P0 na versão; depende de SL-S29-04; evidência em `reports/S29/SL-S29-05.md`.

### SL-S29-06 — Sanitizar resultado

- **Executar:** Bloquear scripts/handlers/schemes perigosos antes da apresentação.
- **Produzir:** `workers/html-normalizer/sanitize.ts`.
- **Aceitar quando:** Readability não é tratado como sanitizador e payload XSS não executa.
- **Controle:** Implementação; P0 na versão; depende de SL-S29-05; evidência em `reports/S29/SL-S29-06.md`.

### SL-S29-07 — Tratar assets externos

- **Executar:** Devolver lista de recursos candidatos para aquisição separada/autorizada.
- **Produzir:** `workers/html-normalizer/assets.ts`.
- **Aceitar quando:** Parser não baixa imagens/trackers por conta própria.
- **Controle:** Implementação; P0 na versão; depende de SL-S29-06; evidência em `reports/S29/SL-S29-07.md`.

### SL-S29-08 — Normalizar fonte

- **Executar:** Registrar URL de origem, URL final informada, data e hash do HTML recebido.
- **Produzir:** `workers/html-normalizer/provenance.ts`.
- **Aceitar quando:** Snapshot é distinguível de página live e tem cadeia de origem verificável.
- **Controle:** Implementação; P0 na versão; depende de SL-S29-07; evidência em `reports/S29/SL-S29-08.md`.

### SL-S29-09 — Executar corpus hostil

- **Executar:** Cobrir iframes, meta refresh, links locais, CSS remoto e conteúdo oculto.
- **Produzir:** `tests/extract/html-security/`.
- **Aceitar quando:** Zero chamadas de rede inesperadas e nenhum acesso à bridge Tauri.
- **Controle:** Teste; P0 na versão; depende de SL-S29-08; evidência em `reports/S29/SL-S29-09.md`.

### SL-S29-10 — Comparar fidelidade e lifecycle

- **Executar:** Avaliar artigos, páginas não-artigo, idiomas e ciclos de criação/destruição.
- **Produzir:** `reports/clipping/normalizer.md`.
- **Aceitar quando:** Perdas e fallback são explícitos; renderer auxiliar não permanece órfão.
- **Controle:** Teste; P0 na versão; depende de SL-S29-09; evidência em `reports/S29/SL-S29-10.md`.

### SL-S29-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Processar página de fixture com scripts/trackers sem executar scripts ou acessar rede.**

Registrar commit, ambiente, testes e pendências em `reports/S29/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s30"></a>

## S30 — Entregar Ingest sem fricção e leitura do acervo

- **Objetivo:** Unificar captura, extração e estados visíveis sem exigir organização antecipada.
- **Componente / forma:** IngestCoordinator / UI / Integração.
- **Pré-requisitos técnicos:** S27 (Desconstruir Docling e integrar extração textual); S28 (Adicionar OCR e robustez da extração); S29 (Integrar Readability/Turndown para clipping seguro)
- **Entregável do sprint:** v0.5.0: Ingest de arquivos e HTML local com leitura e promoção à Mesa.
- **Demonstração exigida:** Soltar documento, continuar trabalhando e promover um recorte após extração.
- **Requisitos:** R02, R14, R15, R17, R31. **Risco de integração:** Alto.

### SL-S30-01 — Criar entrada drag/drop e seleção

- **Executar:** Aceitar arquivos e texto sem formulário taxonômico obrigatório.
- **Produzir:** `apps/desktop/src/ingest/capture.ts`.
- **Aceitar quando:** Item recebe ID/original persistido antes de qualquer modelo ser chamado.
- **Controle:** Implementação; P1 na versão; depende de SL-S27-GATE, SL-S28-GATE, SL-S29-GATE; evidência em `reports/S30/SL-S30-01.md`.

### SL-S30-02 — Implementar deduplicação

- **Executar:** Usar hash/fonte com política clara para nova captura versus referência existente.
- **Produzir:** `crates/ingest/src/deduplicate.rs`.
- **Aceitar quando:** Capturas legítimas em datas diferentes não são fundidas sem preservar sua origem.
- **Controle:** Implementação; P1 na versão; depende de SL-S30-01; evidência em `reports/S30/SL-S30-02.md`.

### SL-S30-03 — Orquestrar roteamento

- **Executar:** Escolher parser por tipo verificado, não só por extensão do nome.
- **Produzir:** `crates/ingest/src/router.rs`.
- **Aceitar quando:** Tipo enganoso ou não suportado é recusado/registrado sem execução arbitrária.
- **Controle:** Implementação; P1 na versão; depende de SL-S30-02; evidência em `reports/S30/SL-S30-03.md`.

### SL-S30-04 — Criar UI da fila

- **Executar:** Mostrar captura salva, extração pendente, parcial, falhou e reprocessar.
- **Produzir:** `apps/desktop/src/ingest/jobs.ts`.
- **Aceitar quando:** Falha de OCR não faz documento desaparecer do acervo.
- **Controle:** Implementação; P1 na versão; depende de SL-S30-03; evidência em `reports/S30/SL-S30-04.md`.

### SL-S30-05 — Publicar extração pelo Core

- **Executar:** Validar resultado e persistir versão de conteúdo/mapa de fontes.
- **Produzir:** `crates/ingest/src/commit_extraction.rs`.
- **Aceitar quando:** Worker atrasado não sobrescreve correção manual nem resultado de pipeline mais recente.
- **Controle:** Implementação; P1 na versão; depende de SL-S30-04; evidência em `reports/S30/SL-S30-05.md`.

### SL-S30-06 — Criar leitor de material

- **Executar:** Exibir texto, origem, diagnóstico e navegação ao asset por broker.
- **Produzir:** `apps/desktop/src/ingest/reader.ts`.
- **Aceitar quando:** Conteúdo ativo é neutralizado e origem permanece disponível offline.
- **Controle:** Implementação; P1 na versão; depende de SL-S30-05; evidência em `reports/S30/SL-S30-06.md`.

### SL-S30-07 — Criar recorte para célula

- **Executar:** Promover seleção com fonte/revisão preservadas.
- **Produzir:** `crates/ingest/src/promote.rs`.
- **Aceitar quando:** Recorte novo não modifica fonte imutável e conecta-se à proveniência da Peça.
- **Controle:** Implementação; P1 na versão; depende de SL-S30-06; evidência em `reports/S30/SL-S30-07.md`.

### SL-S30-08 — Gerenciar reprocessamento

- **Executar:** Separar nova extração, enriquecimento e alterações autorais aceitas.
- **Produzir:** `crates/ingest/src/reprocess.rs`.
- **Aceitar quando:** Reprocessar não apaga anotação/metadata editada pelo usuário.
- **Controle:** Implementação; P1 na versão; depende de SL-S30-07; evidência em `reports/S30/SL-S30-08.md`.

### SL-S30-09 — Executar ingestão em lote

- **Executar:** Misturar formatos, duplicatas, arquivos inválidos e cancelamentos.
- **Produzir:** `reports/v0.5/batch-ingest.json`.
- **Aceitar quando:** Fila progride com falhas isoladas, sem deadlock nem corrupção global.
- **Controle:** Teste; P1 na versão; depende de SL-S30-08; evidência em `reports/S30/SL-S30-09.md`.

### SL-S30-10 — Publicar alpha de acervo

- **Executar:** Incluir pacote de capacidades por plataforma e instruções sem IA.
- **Produzir:** `release/v0.5.0/`.
- **Aceitar quando:** Usuário captura, lê e organiza materiais sem chave de API.
- **Controle:** Release; P1 na versão; depende de SL-S30-09; evidência em `reports/S30/SL-S30-10.md`.

### SL-S30-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Soltar documento, continuar trabalhando e promover um recorte após extração.**

Registrar commit, ambiente, testes e pendências em `reports/S30/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

# v0.6.0 — Enriquecimento e modelos

**Maturidade:** Alpha assistida  
**Entrega acumulada:** LocalAI gerenciado, resumos, Jev autorizado, fallback local e transcrição homologada.

<a id="s31"></a>

## S31 — Recortar e gerenciar LocalAI

- **Objetivo:** Adotar infraestrutura de inferência sem trazer outro sistema de agentes/memória.
- **Componente / forma:** LocalAI / RuntimeProvider / Desconstrução.
- **Pré-requisitos técnicos:** S23 (Construir protocolo e supervisor de workers); S24 (Implementar perfis de sandbox Linux); S25 (Implementar perfis de sandbox macOS); S26 (Implementar perfis de sandbox Windows); S30 (Entregar Ingest sem fricção e leitura do acervo)
- **Entregável do sprint:** Runtime LocalAI gerenciado e pacote mínimo por alvo de desenvolvimento.
- **Demonstração exigida:** Iniciar runtime autorizado, chamar backend de teste e encerrar sua árvore inteira.
- **Requisitos:** R03, R07, R08, R09, R27, R30. **Risco de integração:** Crítico.

### SL-S31-01 — Fixar LocalAI e backends candidatos

- **Executar:** Registrar código Go, imagens/binários, deps e interfaces usadas.
- **Produzir:** `upstream/localai/manifest.json`.
- **Aceitar quando:** Todos os artefatos têm origem e versão; API compatível não substitui inventário.
- **Controle:** Upstream; P0 na versão; depende de SL-S23-GATE, SL-S24-GATE, SL-S25-GATE, SL-S26-GATE, SL-S30-GATE; evidência em `reports/S31/SL-S31-01.md`.

### SL-S31-02 — Reproduzir inferência mínima

- **Executar:** Executar modelo de teste aprovado fora do app e registrar resultados.
- **Produzir:** `reports/localai/baseline.md`.
- **Aceitar quando:** Configuração upstream verificável existe antes de customizações.
- **Controle:** Upstream; P0 na versão; depende de SL-S31-01; evidência em `reports/S31/SL-S31-02.md`.

### SL-S31-03 — Delimitar superfície

- **Executar:** Excluir do produto agentes, shell, memória/RAG e painel administrativo não necessários.
- **Produzir:** `docs/upstream/localai-boundaries.md`.
- **Aceitar quando:** Nenhuma ferramenta do modelo alcança API administrativa ou execução arbitrária.
- **Controle:** Upstream; P0 na versão; depende de SL-S31-02; evidência em `reports/S31/SL-S31-03.md`.

### SL-S31-04 — Ratificar runtime

- **Executar:** Resolver ADR-010 incluindo distribuição, atualização e supervisão.
- **Produzir:** `docs/adr/010-runtime.md`.
- **Aceitar quando:** Instalação não exige Docker/Python/Go globais de forma implícita.
- **Controle:** Decisão; P0 na versão; depende de SL-S31-03; evidência em `reports/S31/SL-S31-04.md`.

### SL-S31-05 — Criar RuntimeProvider

- **Executar:** Iniciar, verificar saúde, obter capacidades e encerrar runtime por protocolo.
- **Produzir:** `adapters/localai/`.
- **Aceitar quando:** Core não depende de detalhes privados do LocalAI fora do adapter.
- **Controle:** Implementação; P0 na versão; depende de SL-S31-04; evidência em `reports/S31/SL-S31-05.md`.

### SL-S31-06 — Proteger API local

- **Executar:** Bind restrito, autenticação de sessão e acesso apenas via Core.
- **Produzir:** `adapters/localai/security.rs`.
- **Aceitar quando:** Página adquirida ou processo sem token não pode usar a API como ponte de execução.
- **Controle:** Implementação; P0 na versão; depende de SL-S31-05; evidência em `reports/S31/SL-S31-06.md`.

### SL-S31-07 — Aplicar perfis

- **Executar:** Gerar configuração somente para backends aprovados e controlar downloads.
- **Produzir:** `adapters/localai/profiles.rs`.
- **Aceitar quando:** Modelo solicitado pelo agente não instala backend desconhecido.
- **Controle:** Implementação; P0 na versão; depende de SL-S31-06; evidência em `reports/S31/SL-S31-07.md`.

### SL-S31-08 — Integrar lifecycle

- **Executar:** Controlar descendentes, stdout/erros saneados e falha de startup.
- **Produzir:** `crates/workers/src/localai_host.rs`.
- **Aceitar quando:** Falha de modelo não derruba a UI nem deixa processo invisível.
- **Controle:** Implementação; P0 na versão; depende de SL-S31-07; evidência em `reports/S31/SL-S31-08.md`.

### SL-S31-09 — Testar runtime offline

- **Executar:** Provisionar artefatos e bloquear rede durante execução.
- **Produzir:** `tests/runtime/localai-offline/`.
- **Aceitar quando:** Geração local não depende de catálogo remoto, telemetria ou download tardio.
- **Controle:** Teste; P0 na versão; depende de SL-S31-08; evidência em `reports/S31/SL-S31-09.md`.

### SL-S31-10 — Medir footprint sem/com modelo

- **Executar:** Contabilizar runtime e backend filhos separadamente.
- **Produzir:** `reports/localai/resources.json`.
- **Aceitar quando:** Relatório distingue overhead do servidor, pesos e contexto; sem alegação de teto não medido.
- **Controle:** Teste; P0 na versão; depende de SL-S31-09; evidência em `reports/S31/SL-S31-10.md`.

### SL-S31-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Iniciar runtime autorizado, chamar backend de teste e encerrar sua árvore inteira.**

Registrar commit, ambiente, testes e pendências em `reports/S31/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s32"></a>

## S32 — Homologar modelos e construir ModelGateway

- **Objetivo:** Escolher modelos por função, idioma e hardware em vez de por nome fixado sem prova.
- **Componente / forma:** ModelGateway / catálogo / Próprio.
- **Pré-requisitos técnicos:** S31 (Recortar e gerenciar LocalAI)
- **Entregável do sprint:** Catálogo homologado, instalação verificada e gateway de geração/embeddings.
- **Demonstração exigida:** Instalar pacote aprovado, detectar corrupção e obter resposta local cancelável.
- **Requisitos:** R07, R08, R09, R20, R27, R30. **Risco de integração:** Crítico.

### SL-S32-01 — Selecionar candidatos por papel

- **Executar:** Avaliar resumo, embeddings, copiloto e transcrição em português e hardware-alvo.
- **Produzir:** `docs/adr/011-models.md`.
- **Aceitar quando:** Nome do modelo e quantização têm evidência; equivalente local a Jev não é presumido.
- **Controle:** Decisão; P0 na versão; depende de SL-S31-GATE; evidência em `reports/S32/SL-S32-01.md`.

### SL-S32-02 — Criar manifests de modelos

- **Executar:** Registrar hash completo, licença, origem, tokenizer, dimensão e capacidades.
- **Produzir:** `models/manifests/`.
- **Aceitar quando:** Placeholders/truncamentos são rejeitados pelo validador.
- **Controle:** Implementação; P0 na versão; depende de SL-S32-01; evidência em `reports/S32/SL-S32-02.md`.

### SL-S32-03 — Implementar instalação consentida

- **Executar:** Download retomável, verificação e publicação atômica de artefatos.
- **Produzir:** `crates/models/src/install.rs`.
- **Aceitar quando:** Arquivo parcial/corrompido não é carregado; modo offline aceita pacote pré-provisionado.
- **Controle:** Implementação; P0 na versão; depende de SL-S32-02; evidência em `reports/S32/SL-S32-03.md`.

### SL-S32-04 — Resolver diretórios por SO

- **Executar:** Usar pastas apropriadas e cache compartilhado fora de cada Vault.
- **Produzir:** `crates/models/src/paths.rs`.
- **Aceitar quando:** Não há dependência de home/path hardcoded nem duplicação automática por workspace.
- **Controle:** Implementação; P0 na versão; depende de SL-S32-03; evidência em `reports/S32/SL-S32-04.md`.

### SL-S32-05 — Definir capabilities de modelo

- **Executar:** Distinguir generate, embed, tools, schema e transcribe sem assumir paridade.
- **Produzir:** `schemas/model-capabilities.json`.
- **Aceitar quando:** Chamada incompatível falha antes de enviar conteúdo ou instalar recursos.
- **Controle:** Contrato; P0 na versão; depende de SL-S32-04; evidência em `reports/S32/SL-S32-05.md`.

### SL-S32-06 — Criar GenerationProvider

- **Executar:** Normalizar streaming, erros, cancelamento e métricas sem apagar identidade do provedor.
- **Produzir:** `crates/model-gateway/src/generate.rs`.
- **Aceitar quando:** Cancelamento propaga e conteúdo parcial é distinguido de resposta concluída.
- **Controle:** Implementação; P0 na versão; depende de SL-S32-05; evidência em `reports/S32/SL-S32-06.md`.

### SL-S32-07 — Criar EmbeddingProvider

- **Executar:** Fixar preprocessing/pooling/normalização e retornar manifesto junto ao vetor.
- **Produzir:** `crates/model-gateway/src/embed.rs`.
- **Aceitar quando:** Vetor de mesmo tamanho mas outro espaço não é aceito como compatível.
- **Controle:** Implementação; P0 na versão; depende de SL-S32-06; evidência em `reports/S32/SL-S32-07.md`.

### SL-S32-08 — Integrar descarregamento

- **Executar:** Usar limites/watchdog do runtime e verificar tarefas ainda ativas.
- **Produzir:** `crates/models/src/lifecycle.rs`.
- **Aceitar quando:** Ociosidade de cinco minutos libera recursos conforme perfil, sem cortar inferência em andamento.
- **Controle:** Implementação; P0 na versão; depende de SL-S32-07; evidência em `reports/S32/SL-S32-08.md`.

### SL-S32-09 — Testar quotas e starvation

- **Executar:** Concorrer embeddings/resumo com baixo orçamento e prioridades de UI.
- **Produzir:** `tests/runtime/scheduling/`.
- **Aceitar quando:** Fila limita modelos residentes e não provoca alternância patológica sem diagnóstico.
- **Controle:** Teste; P0 na versão; depende de SL-S32-08; evidência em `reports/S32/SL-S32-09.md`.

### SL-S32-10 — Homologar catálogo inicial

- **Executar:** Rodar qualidade, memória e falhas de cada perfil selecionado.
- **Produzir:** `reports/models/qualification.json`.
- **Aceitar quando:** Modelo não aprovado fica indisponível/experimental, sem ser anunciado como padrão homologado.
- **Controle:** Teste; P0 na versão; depende de SL-S32-09; evidência em `reports/S32/SL-S32-10.md`.

### SL-S32-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Instalar pacote aprovado, detectar corrupção e obter resposta local cancelável.**

Registrar commit, ambiente, testes e pendências em `reports/S32/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s33"></a>

## S33 — Implementar resumo executivo e enriquecimento gerativo

- **Objetivo:** Produzir o resumo de 2–5 frases sem confundir síntese com evidência original.
- **Componente / forma:** LocalAI / sumarização / Adaptação.
- **Pré-requisitos técnicos:** S27 (Desconstruir Docling e integrar extração textual); S28 (Adicionar OCR e robustez da extração); S30 (Entregar Ingest sem fricção e leitura do acervo); S32 (Homologar modelos e construir ModelGateway)
- **Entregável do sprint:** SummaryService com pipeline longo, proveniência e revisão humana.
- **Demonstração exigida:** Resumir documento, corrigir o texto e reprocessar sem perder a correção.
- **Requisitos:** R14, R19, R23, R27. **Risco de integração:** Alto.

### SL-S33-01 — Definir resultado de resumo

- **Executar:** Registrar texto, revisão, modelo, prompt, cobertura e fontes relevantes.
- **Produzir:** `schemas/summary-result.json`.
- **Aceitar quando:** Ausência de cobertura completa é representável e não mascarada.
- **Controle:** Contrato; P0 na versão; depende de SL-S27-GATE, SL-S28-GATE, SL-S30-GATE, SL-S32-GATE; evidência em `reports/S33/SL-S33-01.md`.

### SL-S33-02 — Criar prompt/validador curto

- **Executar:** Pedir síntese densa e controlar extensão por idioma sem exigir slogans.
- **Produzir:** `crates/enrichment/src/executive_summary.rs`.
- **Aceitar quando:** Resultado de 2–5 frases é verificável; excesso/insuficiência aciona tratamento explícito.
- **Controle:** Implementação; P1 na versão; depende de SL-S33-01; evidência em `reports/S33/SL-S33-02.md`.

### SL-S33-03 — Implementar rota de documento curto

- **Executar:** Ler conteúdo normalizado autorizado e chamar modelo leve.
- **Produzir:** `crates/enrichment/src/summary_short.rs`.
- **Aceitar quando:** Não usa apenas título ou snippet como se tivesse lido o corpo.
- **Controle:** Implementação; P1 na versão; depende de SL-S33-02; evidência em `reports/S33/SL-S33-03.md`.

### SL-S33-04 — Implementar rota hierárquica

- **Executar:** Resumir seções sob budget e consolidar sem duplicar chamadas desnecessárias.
- **Produzir:** `crates/enrichment/src/summary_long.rs`.
- **Aceitar quando:** Documento além do contexto não é truncado silenciosamente aos primeiros tokens.
- **Controle:** Implementação; P1 na versão; depende de SL-S33-03; evidência em `reports/S33/SL-S33-04.md`.

### SL-S33-05 — Preservar evidências/limitações

- **Executar:** Associar afirmações importantes ou referências disponíveis ao material de entrada.
- **Produzir:** `crates/enrichment/src/summary_provenance.rs`.
- **Aceitar quando:** Resumo não recebe status de fonte primária e não inventa localização ausente.
- **Controle:** Implementação; P1 na versão; depende de SL-S33-04; evidência em `reports/S33/SL-S33-05.md`.

### SL-S33-06 — Persistir gerado versus editado

- **Executar:** Separar proposta automática e versão confirmada pelo usuário.
- **Produzir:** `crates/enrichment/src/summary_state.rs`.
- **Aceitar quando:** Reindexação/reexecução não substitui resumo editado.
- **Controle:** Implementação; P1 na versão; depende de SL-S33-05; evidência em `reports/S33/SL-S33-06.md`.

### SL-S33-07 — Criar UI de revisão

- **Executar:** Exibir origem automática, regenerar, editar, aceitar e restaurar.
- **Produzir:** `apps/desktop/src/ingest/summary.ts`.
- **Aceitar quando:** Ação é reversível e não depende de alterar o original capturado.
- **Controle:** Implementação; P1 na versão; depende de SL-S33-06; evidência em `reports/S33/SL-S33-07.md`.

### SL-S33-08 — Controlar cache por revisão

- **Executar:** Reutilizar somente quando conteúdo/modelo/prompt compatíveis.
- **Produzir:** `crates/enrichment/src/summary_cache.rs`.
- **Aceitar quando:** Alterar o texto invalida resumo; mover cartão não o invalida.
- **Controle:** Implementação; P1 na versão; depende de SL-S33-07; evidência em `reports/S33/SL-S33-08.md`.

### SL-S33-09 — Avaliar factualidade no corpus

- **Executar:** Medir omissões, números errados, idioma e abstenção em extração parcial.
- **Produzir:** `tests/eval/summaries/`.
- **Aceitar quando:** Falhas são registradas contra conjunto reservado, não resolvidas apenas mudando o exemplo de demonstração.
- **Controle:** Teste; P1 na versão; depende de SL-S33-08; evidência em `reports/S33/SL-S33-09.md`.

### SL-S33-10 — Testar ausência de modelo/OOM

- **Executar:** Manter captura e permitir revisão futura ou escrita manual do resumo.
- **Produzir:** `reports/summary/failure.md`.
- **Aceitar quando:** Enriquecimento opcional não torna o acervo indisponível.
- **Controle:** Teste; P1 na versão; depende de SL-S33-09; evidência em `reports/S33/SL-S33-10.md`.

### SL-S33-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Resumir documento, corrigir o texto e reprocessar sem perder a correção.**

Registrar commit, ambiente, testes e pendências em `reports/S33/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s34"></a>

## S34 — Integrar Jev como motor de decisões

- **Objetivo:** Implementar decisões tipadas e egress explícito, sem tentar executar Jev no LocalAI.
- **Componente / forma:** Jev / DecisionProvider / Integração.
- **Pré-requisitos técnicos:** S23 (Construir protocolo e supervisor de workers); S30 (Entregar Ingest sem fricção e leitura do acervo); S32 (Homologar modelos e construir ModelGateway)
- **Entregável do sprint:** Adapter Jev testável offline e chamadas reais estritamente opt-in.
- **Demonstração exigida:** Classificar fixture autorizada e negar a mesma chamada quando a política de rede proíbe.
- **Requisitos:** R07, R08, R18, R33, R34. **Risco de integração:** Crítico.

### SL-S34-01 — Fixar contrato da API

- **Executar:** Registrar SDK/endpoint/modelo, termos, limites e formato de respostas observados.
- **Produzir:** `upstream/jev/contract.json`.
- **Aceitar quando:** Integração identifica oferta hospedada; código aberto do cliente não é confundido com pesos disponíveis.
- **Controle:** Upstream; P0 na versão; depende de SL-S23-GATE, SL-S30-GATE, SL-S32-GATE; evidência em `reports/S34/SL-S34-01.md`.

### SL-S34-02 — Definir DecisionRequest/Result

- **Executar:** Modelar Choice, Score e Noul com campos distintos e proveniência.
- **Produzir:** `schemas/decision-v0.json`.
- **Aceitar quando:** Noul não ganha confidence inventada e score local não é rotulado como Jev.
- **Controle:** Contrato; P0 na versão; depende de SL-S34-01; evidência em `reports/S34/SL-S34-02.md`.

### SL-S34-03 — Implementar armazenamento de chave

- **Executar:** Usar cofre de credenciais do SO e não expor segredo à UI/logs.
- **Produzir:** `adapters/jev/credentials.rs`.
- **Aceitar quando:** Export do Vault e Git não contêm chave nem headers sensíveis.
- **Controle:** Implementação; P0 na versão; depende de SL-S34-02; evidência em `reports/S34/SL-S34-03.md`.

### SL-S34-04 — Aplicar EgressPolicy antes da chamada

- **Executar:** Verificar consentimento, escopo e conteúdo autorizado por tarefa.
- **Produzir:** `adapters/jev/egress.rs`.
- **Aceitar quando:** Classificador não recebe dados para decidir se o próprio envio é permitido.
- **Controle:** Implementação; P0 na versão; depende de SL-S34-03; evidência em `reports/S34/SL-S34-04.md`.

### SL-S34-05 — Criar cliente com limites

- **Executar:** Tratar timeout, rate limit, retries controlados, custo/uso disponível e cancelamento.
- **Produzir:** `adapters/jev/client.rs`.
- **Aceitar quando:** Retry não ocorre ilimitadamente nem ignora revogação de consentimento.
- **Controle:** Implementação; P0 na versão; depende de SL-S34-04; evidência em `reports/S34/SL-S34-05.md`.

### SL-S34-06 — Mapear categoria Choice

- **Executar:** Incluir candidatos e opção nenhuma/insuficiente quando pertinente.
- **Produzir:** `adapters/jev/category.rs`.
- **Aceitar quando:** Categoria fora dos candidatos não é inventada por parsing de texto livre.
- **Controle:** Implementação; P0 na versão; depende de SL-S34-05; evidência em `reports/S34/SL-S34-06.md`.

### SL-S34-07 — Mapear tags Noul

- **Executar:** Avaliar pertencimento independente por candidato em lote quando suportado.
- **Produzir:** `adapters/jev/tags.rs`.
- **Aceitar quando:** Tags não se tornam mutuamente exclusivas e probability não é tratada como verdade garantida.
- **Controle:** Implementação; P0 na versão; depende de SL-S34-06; evidência em `reports/S34/SL-S34-07.md`.

### SL-S34-08 — Mapear rubricas Score

- **Executar:** Versionar níveis e preservar distribuições/identidade do critério.
- **Produzir:** `adapters/jev/scores.rs`.
- **Aceitar quando:** Mudar rubrica invalida comparação indevida com scores antigos.
- **Controle:** Implementação; P0 na versão; depende de SL-S34-07; evidência em `reports/S34/SL-S34-08.md`.

### SL-S34-09 — Criar transporte fake e testes de falha

- **Executar:** Simular respostas válidas, recusa, timeout, segredo ausente e payload inválido.
- **Produzir:** `tests/decision/jev-contract/`.
- **Aceitar quando:** CI passa sem chave e sem chamada faturável; falhas não aplicam decisões parciais indevidas.
- **Controle:** Teste; P0 na versão; depende de SL-S34-08; evidência em `reports/S34/SL-S34-09.md`.

### SL-S34-10 — Executar smoke real consentido

- **Executar:** Usar corpus não sensível e orçamento aprovado; registrar versão e métricas.
- **Produzir:** `reports/jev/live-smoke.md`.
- **Aceitar quando:** Teste não executado permanece pendente; resultado real não é fabricado a partir do mock.
- **Controle:** Teste; P0 na versão; depende de SL-S34-09; evidência em `reports/S34/SL-S34-10.md`.

### SL-S34-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Classificar fixture autorizada e negar a mesma chamada quando a política de rede proíbe.**

Registrar commit, ambiente, testes e pendências em `reports/S34/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s35"></a>

## S35 — Implementar taxonomia e fallback de classificação

- **Objetivo:** Colocar políticas de classificação no produto, e não dentro de prompts opacos.
- **Componente / forma:** TaxonomyService / Jev / Próprio.
- **Pré-requisitos técnicos:** S32 (Homologar modelos e construir ModelGateway); S34 (Integrar Jev como motor de decisões)
- **Entregável do sprint:** Taxonomia canônica, revisão/aliases e fallback offline.
- **Demonstração exigida:** Aplicar sugestão, desfazer e manter classificação utilizável sem a API.
- **Requisitos:** R02, R07, R18, R20, R28. **Risco de integração:** Médio.

### SL-S35-01 — Persistir taxonomia por IDs

- **Executar:** Guardar termos, idioma, descrições, aliases e origem das decisões.
- **Produzir:** `crates/taxonomy/src/store.rs`.
- **Aceitar quando:** Apagar DB não perde termos/aliases aprovados.
- **Controle:** Implementação; P1 na versão; depende de SL-S32-GATE, SL-S34-GATE; evidência em `reports/S35/SL-S35-01.md`.

### SL-S35-02 — Recuperar candidatos

- **Executar:** Combinar léxico e embeddings aprovados sem misturar espaços incompatíveis.
- **Produzir:** `crates/taxonomy/src/candidates.rs`.
- **Aceitar quando:** Categoria correta fora do top-k pode levar a expansão/abstenção, não escolha forçada silenciosa.
- **Controle:** Implementação; P1 na versão; depende de SL-S35-01; evidência em `reports/S35/SL-S35-02.md`.

### SL-S35-03 — Calibrar política de aceitação

- **Executar:** Resolver ADR-012 com conjunto separado de avaliação e tolerância a erro.
- **Produzir:** `docs/adr/012-taxonomy-policy.md`.
- **Aceitar quando:** Thresholds não são copiados cegamente do RFC nem tratados como universais.
- **Controle:** Decisão; P0 na versão; depende de SL-S35-02; evidência em `reports/S35/SL-S35-03.md`.

### SL-S35-04 — Aplicar decisões reversíveis

- **Executar:** Diferenciar sugerido, aceito automaticamente, confirmado e rejeitado.
- **Produzir:** `crates/taxonomy/src/decisions.rs`.
- **Aceitar quando:** Cada mutação tem origem e pode ser revertida sem apagar decisão humana posterior.
- **Controle:** Implementação; P1 na versão; depende de SL-S35-03; evidência em `reports/S35/SL-S35-04.md`.

### SL-S35-05 — Propor novos termos

- **Executar:** Usar extração/modelo generativo com validação; Jev avalia candidatos quando autorizado.
- **Produzir:** `crates/taxonomy/src/new_terms.rs`.
- **Aceitar quando:** Jev não é chamado para gerar string livre inexistente em suas primitivas.
- **Controle:** Implementação; P1 na versão; depende de SL-S35-04; evidência em `reports/S35/SL-S35-05.md`.

### SL-S35-06 — Proteger merges/aliases

- **Executar:** Transformar distância/similaridade em sugestão revisável, não fusão silenciosa.
- **Produzir:** `crates/taxonomy/src/aliases.rs`.
- **Aceitar quando:** Termos parecidos mas diferentes permanecem distintos sem aprovação.
- **Controle:** Implementação; P1 na versão; depende de SL-S35-05; evidência em `reports/S35/SL-S35-06.md`.

### SL-S35-07 — Implementar fallback local

- **Executar:** Usar modelo homologado com schema e registrar ausência de calibração equivalente.
- **Produzir:** `crates/taxonomy/src/local_fallback.rs`.
- **Aceitar quando:** Probabilidades/confidence não são inventadas para imitar resultado Jev.
- **Controle:** Implementação; P1 na versão; depende de SL-S35-06; evidência em `reports/S35/SL-S35-07.md`.

### SL-S35-08 — Implementar fallback sem modelo

- **Executar:** Regras léxicas ou pendência/revisão manual sem precisar carregar embeddings por OOM.
- **Produzir:** `crates/taxonomy/src/heuristic_fallback.rs`.
- **Aceitar quando:** Máquina sem RAM/rede continua capturando material e mostra estado correto.
- **Controle:** Implementação; P1 na versão; depende de SL-S35-07; evidência em `reports/S35/SL-S35-08.md`.

### SL-S35-09 — Criar UI de revisão taxonômica

- **Executar:** Filtrar sugestões, editar, confirmar e desfazer em lote com preview.
- **Produzir:** `apps/desktop/src/taxonomy/`.
- **Aceitar quando:** Bulk action não altera itens fora do conjunto aprovado nem esconde baixa confiança.
- **Controle:** Implementação; P1 na versão; depende de SL-S35-08; evidência em `reports/S35/SL-S35-09.md`.

### SL-S35-10 — Avaliar português e casos ambíguos

- **Executar:** Medir categoria, multilabel, abstention e merges indevidos.
- **Produzir:** `reports/taxonomy/evaluation.json`.
- **Aceitar quando:** Regressões em corpus reservado impedem habilitar automação mais agressiva.
- **Controle:** Teste; P1 na versão; depende de SL-S35-09; evidência em `reports/S35/SL-S35-10.md`.

### SL-S35-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Aplicar sugestão, desfazer e manter classificação utilizável sem a API.**

Registrar commit, ambiente, testes e pendências em `reports/S35/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s36"></a>

## S36 — Transcrever mídia e concluir enriquecimento do Ingest

- **Objetivo:** Fechar o conjunto de enriquecimentos com áudio/vídeo e estados independentes.
- **Componente / forma:** Transcrição / Ingest integrado / Integração.
- **Pré-requisitos técnicos:** S30 (Entregar Ingest sem fricção e leitura do acervo); S32 (Homologar modelos e construir ModelGateway); S33 (Implementar resumo executivo e enriquecimento gerativo); S35 (Implementar taxonomia e fallback de classificação)
- **Entregável do sprint:** v0.6.0 com resumo, classificação e transcrição local homologada.
- **Demonstração exigida:** Importar áudio e documento, continuar editando e acompanhar cada resultado separadamente.
- **Requisitos:** R09, R14, R16, R18, R19, R27. **Risco de integração:** Alto.

### SL-S36-01 — Fixar backend de transcrição

- **Executar:** Selecionar rota LocalAI/Whisper homologada e dependências de decodificação.
- **Produzir:** `upstream/transcription/manifest.json`.
- **Aceitar quando:** Licenças e binários auxiliares estão inventariados; suporte não é presumido por extensão.
- **Controle:** Upstream; P1 na versão; depende de SL-S30-GATE, SL-S32-GATE, SL-S33-GATE, SL-S35-GATE; evidência em `reports/S36/SL-S36-01.md`.

### SL-S36-02 — Definir transcript e timestamps

- **Executar:** Modelar segmentos, idioma, revisão da fonte e cobertura parcial.
- **Produzir:** `schemas/transcript-v0.json`.
- **Aceitar quando:** Segmento cita tempo e arquivo/hash corretos; diarização não é prometida sem implementação.
- **Controle:** Contrato; P0 na versão; depende de SL-S36-01; evidência em `reports/S36/SL-S36-02.md`.

### SL-S36-03 — Preparar áudio com limites

- **Executar:** Decodificar/normalizar em worker autorizado e controlar arquivos temporários.
- **Produzir:** `workers/transcription/input/`.
- **Aceitar quando:** Arquivo corrompido ou excessivo não trava Core nem executa comando montado com entrada não confiável.
- **Controle:** Implementação; P1 na versão; depende de SL-S36-02; evidência em `reports/S36/SL-S36-03.md`.

### SL-S36-04 — Executar transcrição

- **Executar:** Integrar backend por jobs e orçamento, com cancelamento real.
- **Produzir:** `workers/transcription/adapter/`.
- **Aceitar quando:** Tarefa interrompida libera recursos e conserva original/segmentos aproveitáveis conforme contrato.
- **Controle:** Implementação; P1 na versão; depende de SL-S36-03; evidência em `reports/S36/SL-S36-04.md`.

### SL-S36-05 — Persistir texto corrigível

- **Executar:** Manter transcrição gerada e revisão humana separadas.
- **Produzir:** `crates/ingest/src/transcripts.rs`.
- **Aceitar quando:** Reprocessamento não apaga correções manuais nem perde timestamps originais.
- **Controle:** Implementação; P1 na versão; depende de SL-S36-04; evidência em `reports/S36/SL-S36-05.md`.

### SL-S36-06 — Unificar estados de enriquecimento

- **Executar:** Acompanhar extração, resumo, classificação e embeddings independentemente.
- **Produzir:** `crates/ingest/src/enrichment_state.rs`.
- **Aceitar quando:** Falha em uma etapa não falsifica sucesso das demais ou bloqueia leitura do item.
- **Controle:** Implementação; P1 na versão; depende de SL-S36-05; evidência em `reports/S36/SL-S36-06.md`.

### SL-S36-07 — Evitar competição de modelos

- **Executar:** Priorizar UI e reagendar OCR/STT/geração conforme perfil.
- **Produzir:** `crates/jobs/src/model_scheduling.rs`.
- **Aceitar quando:** Workloads simultâneos não ultrapassam política aprovada por serem jobs independentes.
- **Controle:** Implementação; P1 na versão; depende de SL-S36-06; evidência em `reports/S36/SL-S36-07.md`.

### SL-S36-08 — Avaliar transcrição portuguesa

- **Executar:** Medir erros e alinhamento temporal no corpus reservado.
- **Produzir:** `reports/transcription/qualification.json`.
- **Aceitar quando:** Limitações de idioma/ruído são expostas; transcrição não é tratada como citação perfeita sem revisão.
- **Controle:** Teste; P1 na versão; depende de SL-S36-07; evidência em `reports/S36/SL-S36-08.md`.

### SL-S36-09 — Executar jornada sem chave/rede

- **Executar:** Classificar por fallback, resumir localmente e transcrever com artefatos instalados.
- **Produzir:** `reports/v0.6/offline-enrichment.md`.
- **Aceitar quando:** Ausência Jev/BYOK não provoca requisição escondida ou perda de captura.
- **Controle:** Teste; P1 na versão; depende de SL-S36-08; evidência em `reports/S36/SL-S36-09.md`.

### SL-S36-10 — Publicar alpha assistida

- **Executar:** Empacotar perfis aprovados, diagnóstico de modelo e documentação de consentimento.
- **Produzir:** `release/v0.6.0/`.
- **Aceitar quando:** Capacidade só é anunciada nos alvos/perfis em que foi testada.
- **Controle:** Release; P1 na versão; depende de SL-S36-09; evidência em `reports/S36/SL-S36-10.md`.

### SL-S36-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Importar áudio e documento, continuar editando e acompanhar cada resultado separadamente.**

Registrar commit, ambiente, testes e pendências em `reports/S36/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

# v0.7.0 — Busca híbrida

**Maturidade:** Alpha integrada  
**Entrega acumulada:** Escolher Qdrant OU HelixDB e recuperar materiais/Peças com índices descartáveis e revisões válidas.

<a id="s37"></a>

## S37 — Definir contrato e bancada de busca híbrida

- **Objetivo:** Comparar motores sob o mesmo contrato antes de escolhê-los.
- **Componente / forma:** SearchService / avaliação / Próprio.
- **Pré-requisitos técnicos:** S05 (Construir CI, corpus e protocolo de medição); S10 (Reconciliar alterações externas e reconstruir índices); S22 (Entregar produção local, histórico e exportação); S32 (Homologar modelos e construir ModelGateway); S36 (Transcrever mídia e concluir enriquecimento do Ingest)
- **Entregável do sprint:** Contrato de busca, indexação e protocolo comparativo.
- **Demonstração exigida:** Mesma fixture/consulta roda em providers fake e produz ranking/evidências verificáveis.
- **Requisitos:** R02, R20, R21, R34. **Risco de integração:** Médio.

### SL-S37-01 — Definir SearchDocument

- **Executar:** Identificar item, tipo, escopo, revisão, bloco, localizador e estado editorial.
- **Produzir:** `schemas/search-document-v0.json`.
- **Aceitar quando:** Peça derivada e fonte externa podem ser filtradas separadamente.
- **Controle:** Contrato; P0 na versão; depende de SL-S05-GATE, SL-S10-GATE, SL-S22-GATE, SL-S32-GATE, SL-S36-GATE; evidência em `reports/S37/SL-S37-01.md`.

### SL-S37-02 — Definir SearchQuery/Hit

- **Executar:** Incluir filtros, top-k, budget, score/origem e referência de evidência.
- **Produzir:** `schemas/search-query-v0.json`.
- **Aceitar quando:** Resultado não entrega path arbitrário para ferramenta abrir.
- **Controle:** Contrato; P0 na versão; depende de SL-S37-01; evidência em `reports/S37/SL-S37-02.md`.

### SL-S37-03 — Criar interface SearchProvider

- **Executar:** Delimitar upsert, delete, query, health, rebuild e capabilities.
- **Produzir:** `crates/search/src/provider.rs`.
- **Aceitar quando:** Domínio compila contra interface sem SDK privado de Qdrant/Helix espalhado.
- **Controle:** Implementação; P1 na versão; depende de SL-S37-02; evidência em `reports/S37/SL-S37-03.md`.

### SL-S37-04 — Definir unidades de indexação

- **Executar:** Usar seções/blocos com vínculo ao item; resumo é representação adicional.
- **Produzir:** `crates/search/src/units.rs`.
- **Aceitar quando:** Recuperação não depende só do resumo de 2–5 frases nem trata canvas como chunks cegos.
- **Controle:** Implementação; P1 na versão; depende de SL-S37-03; evidência em `reports/S37/SL-S37-04.md`.

### SL-S37-05 — Integrar embeddings versionados

- **Executar:** Associar manifesto e rejeitar dimensão/espaço divergente.
- **Produzir:** `crates/search/src/vector_manifest.rs`.
- **Aceitar quando:** Troca de modelo exige reindexação ou coleção distinta, sem mesclar vetores.
- **Controle:** Implementação; P1 na versão; depende de SL-S37-04; evidência em `reports/S37/SL-S37-05.md`.

### SL-S37-06 — Construir baseline léxico

- **Executar:** Usar FTS5 ou baseline equivalente para comparar ganhos reais.
- **Produzir:** `experiments/search/lexical-baseline/`.
- **Aceitar quando:** Busca semântica não é declarada superior sem comparação com léxico.
- **Controle:** Implementação; P1 na versão; depende de SL-S37-05; evidência em `reports/S37/SL-S37-06.md`.

### SL-S37-07 — Fixar protocolo de avaliação

- **Executar:** Definir recall/ranking, filtros, exclusão, latência, memória e cold start.
- **Produzir:** `experiments/search/protocol.md`.
- **Aceitar quando:** Métricas, corpus reservado e método são idênticos para os dois candidatos.
- **Controle:** Teste; P1 na versão; depende de SL-S37-06; evidência em `reports/S37/SL-S37-07.md`.

### SL-S37-08 — Criar suíte de conformidade

- **Executar:** Testar IDs, revision update, tombstone, filtros e reconstrução.
- **Produzir:** `tests/contracts/search-provider/`.
- **Aceitar quando:** Provider fake malicioso falha ao retornar item de outro escopo ou revisão obsoleta.
- **Controle:** Teste; P1 na versão; depende de SL-S37-07; evidência em `reports/S37/SL-S37-08.md`.

### SL-S37-09 — Preparar escalas de corpus

- **Executar:** Medir conjuntos pequeno/médio/grande escolhidos antes dos resultados.
- **Produzir:** `experiments/search/datasets.yaml`.
- **Aceitar quando:** Tamanho/semântica não mudam para favorecer um candidato depois da medição.
- **Controle:** Teste; P1 na versão; depende de SL-S37-08; evidência em `reports/S37/SL-S37-09.md`.

### SL-S37-10 — Fixar regra de escolha

- **Executar:** Priorizar correção, distribuição/licença, recursos e depois desempenho.
- **Produzir:** `experiments/search/decision-rubric.md`.
- **Aceitar quando:** Empate ou inviabilidade possui regra de replanejamento, não escolha por preferência não registrada.
- **Controle:** Decisão; P0 na versão; depende de SL-S37-09; evidência em `reports/S37/SL-S37-10.md`.

### SL-S37-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Mesma fixture/consulta roda em providers fake e produz ranking/evidências verificáveis.**

Registrar commit, ambiente, testes e pendências em `reports/S37/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s38"></a>

## S38 — Avaliar Qdrant sem incorporar servidor completo

- **Objetivo:** Testar Edge/Server adequados ao desktop com recorte experimental descartável.
- **Componente / forma:** Qdrant / candidato A / Spike de motor.
- **Pré-requisitos técnicos:** S37 (Definir contrato e bancada de busca híbrida)
- **Entregável do sprint:** Relatório Qdrant e provider experimental.
- **Demonstração exigida:** Executar conformidade e benchmarks iguais ao candidato B, com conclusão viável/inviável.
- **Requisitos:** R01, R20, R21, R30, R33. **Risco de integração:** Crítico.

> Este sprint fecha uma investigação, não aprova automaticamente Qdrant para produção. Conclusão negativa fundamentada é uma saída válida do spike.

### SL-S38-01 — Fixar revisão/artefato Qdrant

- **Executar:** Auditar Edge e Server, APIs disponíveis, licença e dependências.
- **Produzir:** `experiments/search/qdrant/manifest.json`.
- **Aceitar quando:** Não se presume paridade Edge/Server ou licença apenas pela marca.
- **Controle:** Upstream; P0 na versão; depende de SL-S37-GATE; evidência em `reports/S38/SL-S38-01.md`.

### SL-S38-02 — Escolher modo de prova

- **Executar:** Comparar exigências embutido versus processo local para o conjunto requerido.
- **Produzir:** `experiments/search/qdrant/mode.md`.
- **Aceitar quando:** Motivo do modo e recursos indisponíveis ficam explícitos.
- **Controle:** Spike; P0 na versão; depende de SL-S38-01; evidência em `reports/S38/SL-S38-02.md`.

### SL-S38-03 — Montar harness mínimo

- **Executar:** Abrir engine/serviço em diretório temporário e expor contrato experimental.
- **Produzir:** `experiments/search/qdrant/adapter/`.
- **Aceitar quando:** Nenhuma UI do produto depende deste candidato antes da decisão.
- **Controle:** Implementação; P0 na versão; depende de SL-S38-02; evidência em `reports/S38/SL-S38-03.md`.

### SL-S38-04 — Exercitar CRUD e revisões

- **Executar:** Inserir, atualizar, excluir e reiniciar com IDs do contrato.
- **Produzir:** `reports/search/qdrant/crud.json`.
- **Aceitar quando:** Evidência registra resultado de cada caso; falha funcional torna candidato rejeitado, não teste omitido.
- **Controle:** Teste; P0 na versão; depende de SL-S38-03; evidência em `reports/S38/SL-S38-04.md`.

### SL-S38-05 — Exercitar filtros de escopo

- **Executar:** Consultar tipos, workspaces, revisões e estados editoriais.
- **Produzir:** `reports/search/qdrant/filters.json`.
- **Aceitar quando:** Não há hits fora do escopo nos casos válidos; incompatibilidade é conclusiva.
- **Controle:** Teste; P0 na versão; depende de SL-S38-04; evidência em `reports/S38/SL-S38-05.md`.

### SL-S38-06 — Exercitar caminho híbrido

- **Executar:** Combinar dense/sparse ou léxico externo conforme capabilities reais.
- **Produzir:** `reports/search/qdrant/hybrid.json`.
- **Aceitar quando:** Fusões e normalizações são documentadas e comparáveis ao baseline.
- **Controle:** Teste; P0 na versão; depende de SL-S38-05; evidência em `reports/S38/SL-S38-06.md`.

### SL-S38-07 — Medir busca/indexação

- **Executar:** Executar protocolo reservado e guardar resultados brutos.
- **Produzir:** `reports/search/qdrant/performance.json`.
- **Aceitar quando:** Memória/latência incluem modo de execução usado, não números promocionais.
- **Controle:** Teste; P0 na versão; depende de SL-S38-06; evidência em `reports/S38/SL-S38-07.md`.

### SL-S38-08 — Exercitar falha/rebuild

- **Executar:** Interromper índice, apagar cache e reconstruir do corpus canônico.
- **Produzir:** `reports/search/qdrant/recovery.json`.
- **Aceitar quando:** Dados autorais não dependem do candidato; recuperação e limitações são observadas.
- **Controle:** Teste; P0 na versão; depende de SL-S38-07; evidência em `reports/S38/SL-S38-08.md`.

### SL-S38-09 — Verificar distribuição

- **Executar:** Testar empacotamento e dependências nos alvos prioritários e mapear pendências dos demais.
- **Produzir:** `reports/search/qdrant/packaging.md`.
- **Aceitar quando:** Suporte não observado permanece pendente; cenário experimental não vira homologação.
- **Controle:** Spike; P0 na versão; depende de SL-S38-08; evidência em `reports/S38/SL-S38-09.md`.

### SL-S38-10 — Consolidar conclusão A

- **Executar:** Marcar capabilities aprovadas, rejeitadas e custos de adaptação.
- **Produzir:** `reports/search/qdrant/conclusion.md`.
- **Aceitar quando:** Relatório pode concluir inviabilidade; deve completar o protocolo em vez de mascarar falhas.
- **Controle:** Decisão; P0 na versão; depende de SL-S38-09; evidência em `reports/S38/SL-S38-10.md`.

### SL-S38-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Executar conformidade e benchmarks iguais ao candidato B, com conclusão viável/inviável.**

Registrar commit, ambiente, testes e pendências em `reports/S38/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s39"></a>

## S39 — Avaliar HelixDB sob o mesmo contrato

- **Objetivo:** Medir integração embutida e busca/grafo sem tornar HelixDB fonte da verdade.
- **Componente / forma:** HelixDB / candidato B / Spike de motor.
- **Pré-requisitos técnicos:** S37 (Definir contrato e bancada de busca híbrida)
- **Entregável do sprint:** Relatório HelixDB e provider experimental.
- **Demonstração exigida:** Rodar o mesmo corpus e confrontar resultados com o candidato A.
- **Requisitos:** R01, R20, R21, R30, R33. **Risco de integração:** Crítico.

> Concluir o spike não implica adotar HelixDB. Os dois protótipos ficam em experiments e não são distribuídos juntos por padrão.

### SL-S39-01 — Fixar revisão HelixDB

- **Executar:** Auditar SDK/engine embutido, licença e requisitos de storage/cache.
- **Produzir:** `experiments/search/helix/manifest.json`.
- **Aceitar quando:** Modo local/cloud/embedded e geração de API não são misturados.
- **Controle:** Upstream; P0 na versão; depende de SL-S37-GATE; evidência em `reports/S39/SL-S39-01.md`.

### SL-S39-02 — Escolher storage de prova

- **Executar:** Usar disco local e perfil explícito de caches sem exigir object storage remoto.
- **Produzir:** `experiments/search/helix/mode.md`.
- **Aceitar quando:** Modo atende independência de nuvem ou é classificado incompatível.
- **Controle:** Spike; P0 na versão; depende de SL-S39-01; evidência em `reports/S39/SL-S39-02.md`.

### SL-S39-03 — Montar harness mínimo

- **Executar:** Traduzir contrato SearchProvider para SDK sem acoplamento da UI.
- **Produzir:** `experiments/search/helix/adapter/`.
- **Aceitar quando:** Código é experimental e não instala um segundo backend de produção.
- **Controle:** Implementação; P0 na versão; depende de SL-S39-02; evidência em `reports/S39/SL-S39-03.md`.

### SL-S39-04 — Exercitar CRUD e revisão

- **Executar:** Repetir inserção, exclusão, atualização e reabertura do protocolo.
- **Produzir:** `reports/search/helix/crud.json`.
- **Aceitar quando:** Mesmos critérios e IDs do candidato A; falhas preservadas no relatório.
- **Controle:** Teste; P0 na versão; depende de SL-S39-03; evidência em `reports/S39/SL-S39-04.md`.

### SL-S39-05 — Exercitar filtros e BM25

- **Executar:** Verificar busca textual/vetorial e isolamento de escopos.
- **Produzir:** `reports/search/helix/hybrid.json`.
- **Aceitar quando:** Operação integrada não mistura conteúdo de workspace não autorizado.
- **Controle:** Teste; P0 na versão; depende de SL-S39-04; evidência em `reports/S39/SL-S39-05.md`.

### SL-S39-06 — Exercitar relações derivadas

- **Executar:** Projetar pequeno grafo canônico e executar consultas úteis ao produto.
- **Produzir:** `reports/search/helix/graph.json`.
- **Aceitar quando:** Ganho relacional demonstrado por caso real; grafo no banco não substitui topologia em arquivos.
- **Controle:** Teste; P0 na versão; depende de SL-S39-05; evidência em `reports/S39/SL-S39-06.md`.

### SL-S39-07 — Medir recursos e consultas

- **Executar:** Aplicar mesmo corpus, warm/cold, filtros e hardware da prova A.
- **Produzir:** `reports/search/helix/performance.json`.
- **Aceitar quando:** Caches limitados não são apresentados como limite rígido de RSS total.
- **Controle:** Teste; P0 na versão; depende de SL-S39-06; evidência em `reports/S39/SL-S39-07.md`.

### SL-S39-08 — Exercitar falha/reconstrução

- **Executar:** Apagar ou interromper índice e reconstruir a projeção.
- **Produzir:** `reports/search/helix/recovery.json`.
- **Aceitar quando:** Nenhuma autoria só pode ser recuperada do banco.
- **Controle:** Teste; P0 na versão; depende de SL-S39-07; evidência em `reports/S39/SL-S39-08.md`.

### SL-S39-09 — Verificar distribuição

- **Executar:** Inspecionar SDK/runtime por alvo e dependências nativas.
- **Produzir:** `reports/search/helix/packaging.md`.
- **Aceitar quando:** Lacunas de plataforma são fator da decisão, não escondidas como detalhe futuro.
- **Controle:** Spike; P0 na versão; depende de SL-S39-08; evidência em `reports/S39/SL-S39-09.md`.

### SL-S39-10 — Consolidar conclusão B

- **Executar:** Registrar viabilidade, lacunas e custo incremental versus Qdrant.
- **Produzir:** `reports/search/helix/conclusion.md`.
- **Aceitar quando:** Protocolo termina com evidências comparáveis mesmo quando motor é rejeitado.
- **Controle:** Decisão; P0 na versão; depende de SL-S39-09; evidência em `reports/S39/SL-S39-10.md`.

### SL-S39-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Rodar o mesmo corpus e confrontar resultados com o candidato A.**

Registrar commit, ambiente, testes e pendências em `reports/S39/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s40"></a>

## S40 — Escolher e integrar um backend de busca

- **Objetivo:** Converter evidência comparativa em uma implementação de produto, não em dual-stack.
- **Componente / forma:** Qdrant OU HelixDB / produção / Integração.
- **Pré-requisitos técnicos:** S38 (Avaliar Qdrant sem incorporar servidor completo); S39 (Avaliar HelixDB sob o mesmo contrato)
- **Entregável do sprint:** SearchProvider de produção e ADR de seleção.
- **Demonstração exigida:** Executar busca híbrida no app com apenas o backend escolhido instalado.
- **Requisitos:** R02, R20, R21, R30, R32. **Risco de integração:** Crítico.

### SL-S40-01 — Aprovar seleção

- **Executar:** Confrontar rubrica e relatórios; resolver ADR-013 com veto para requisitos essenciais não atendidos.
- **Produzir:** `docs/adr/013-search-backend.md`.
- **Aceitar quando:** Um candidato escolhido explicitamente; se ambos falham, sprint bloqueia e plano é revisto.
- **Controle:** Decisão; P0 na versão; depende de SL-S38-GATE, SL-S39-GATE; evidência em `reports/S40/SL-S40-01.md`.

### SL-S40-02 — Promover adapter selecionado

- **Executar:** Mover somente código necessário do experimento à camada de produção.
- **Produzir:** `adapters/search-selected/`.
- **Aceitar quando:** Segundo candidato não é dependência de runtime nem inicia processos.
- **Controle:** Implementação; P0 na versão; depende de SL-S40-01; evidência em `reports/S40/SL-S40-02.md`.

### SL-S40-03 — Integrar índice e manifest

- **Executar:** Versionar schema, engine, embeddings e configuração de armazenamento.
- **Produzir:** `crates/search/src/index_manifest.rs`.
- **Aceitar quando:** Índice incompatível é reconstruído/isolado e não lido como se fosse atual.
- **Controle:** Implementação; P0 na versão; depende de SL-S40-02; evidência em `reports/S40/SL-S40-03.md`.

### SL-S40-04 — Integrar backend léxico

- **Executar:** Manter FTS5 ou adotar BM25 do motor conforme ADR, evitando duplicação injustificada.
- **Produzir:** `crates/search/src/lexical.rs`.
- **Aceitar quando:** Caminho léxico funciona sem gerar embedding e possui contrato equivalente.
- **Controle:** Implementação; P0 na versão; depende de SL-S40-03; evidência em `reports/S40/SL-S40-04.md`.

### SL-S40-05 — Implementar fusão RRF

- **Executar:** Combinar rankings no coordenador quando motores forem separados.
- **Produzir:** `crates/search/src/fusion.rs`.
- **Aceitar quando:** Fórmula/testes não dependem da antiga query SQL única com sqlite-vec.
- **Controle:** Implementação; P0 na versão; depende de SL-S40-04; evidência em `reports/S40/SL-S40-05.md`.

### SL-S40-06 — Aplicar autorização a hits

- **Executar:** Filtrar/validar escopo e revisão antes de expor conteúdo.
- **Produzir:** `crates/search/src/authorization.rs`.
- **Aceitar quando:** Resultado inesperado do backend não contorna o ToolBroker.
- **Controle:** Implementação; P0 na versão; depende de SL-S40-05; evidência em `reports/S40/SL-S40-06.md`.

### SL-S40-07 — Integrar geração de consultas vetoriais

- **Executar:** Usar mesmo espaço aprovado e fallback léxico em indisponibilidade.
- **Produzir:** `crates/search/src/query_vectors.rs`.
- **Aceitar quando:** Modelo descarregado pode causar latência explícita, não envio externo automático.
- **Controle:** Implementação; P0 na versão; depende de SL-S40-06; evidência em `reports/S40/SL-S40-07.md`.

### SL-S40-08 — Conectar lifecycle e métricas

- **Executar:** Abrir/fechar engine, gerenciar recursos e expor diagnóstico.
- **Produzir:** `crates/search/src/lifecycle.rs`.
- **Aceitar quando:** Engine não vira processo órfão e modo offline é observado.
- **Controle:** Implementação; P0 na versão; depende de SL-S40-07; evidência em `reports/S40/SL-S40-08.md`.

### SL-S40-09 — Executar conformidade de produção

- **Executar:** Rodar suite comum no adapter promovido, não apenas no protótipo.
- **Produzir:** `reports/search/production-contract.json`.
- **Aceitar quando:** Todos os requisitos essenciais aprovados na decisão continuam satisfeitos após integração.
- **Controle:** Teste; P0 na versão; depende de SL-S40-08; evidência em `reports/S40/SL-S40-09.md`.

### SL-S40-10 — Remover dependências não escolhidas

- **Executar:** Atualizar SBOM, pacotes e documentação de instalação.
- **Produzir:** `compliance/search-runtime.json`.
- **Aceitar quando:** Pacote final contém somente o caminho adotado e suas licenças reais.
- **Controle:** Licença; P0 na versão; depende de SL-S40-09; evidência em `reports/S40/SL-S40-10.md`.

### SL-S40-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Executar busca híbrida no app com apenas o backend escolhido instalado.**

Registrar commit, ambiente, testes e pendências em `reports/S40/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s41"></a>

## S41 — Entregar busca incremental no acervo e nas Peças

- **Objetivo:** Fazer recuperação refletir revisões, exclusões e diferentes tipos de autoria.
- **Componente / forma:** SearchService / UI / Integração.
- **Pré-requisitos técnicos:** S35 (Implementar taxonomia e fallback de classificação); S40 (Escolher e integrar um backend de busca)
- **Entregável do sprint:** v0.7.0 com busca híbrida e reconstrução assistida.
- **Demonstração exigida:** Editar/excluir material e verificar atualização da busca sem perder as fontes canônicas.
- **Requisitos:** R02, R05, R13, R20, R21, R31. **Risco de integração:** Crítico.

### SL-S41-01 — Conectar eventos à indexação

- **Executar:** Consumir captura, extração, edição da Peça e tombstones por revisão.
- **Produzir:** `crates/search/src/index_jobs.rs`.
- **Aceitar quando:** Evento repetido não duplica unidades e edição antiga não substitui a mais nova.
- **Controle:** Implementação; P0 na versão; depende de SL-S35-GATE, SL-S40-GATE; evidência em `reports/S41/SL-S41-01.md`.

### SL-S41-02 — Fazer reindexação incremental

- **Executar:** Reprocessar apenas unidades afetadas e propagar mudanças de manifesto.
- **Produzir:** `crates/search/src/incremental.rs`.
- **Aceitar quando:** Alteração de layout não dispara indexação textual novamente; troca de embedding inicia rebuild compatível.
- **Controle:** Implementação; P0 na versão; depende de SL-S41-01; evidência em `reports/S41/SL-S41-02.md`.

### SL-S41-03 — Publicar rebuild atomicamente

- **Executar:** Construir nova geração e trocar quando validada.
- **Produzir:** `crates/search/src/rebuild.rs`.
- **Aceitar quando:** Interrupção mantém busca anterior ou degradação explícita, não índice parcialmente anunciado pronto.
- **Controle:** Implementação; P0 na versão; depende de SL-S41-02; evidência em `reports/S41/SL-S41-03.md`.

### SL-S41-04 — Expor progresso e saúde

- **Executar:** Mostrar pendente, desatualizado, reconstruindo e falha por tipo de índice.
- **Produzir:** `apps/desktop/src/search/health.ts`.
- **Aceitar quando:** Usuário não confunde ausência de resultado com material não indexado.
- **Controle:** Implementação; P0 na versão; depende de SL-S41-03; evidência em `reports/S41/SL-S41-04.md`.

### SL-S41-05 — Criar UI de busca híbrida

- **Executar:** Incluir filtros de tipo, workspace, origem e estado editorial.
- **Produzir:** `apps/desktop/src/search/`.
- **Aceitar quando:** Peças próprias podem ser excluídas quando se busca evidência externa.
- **Controle:** Implementação; P0 na versão; depende de SL-S41-04; evidência em `reports/S41/SL-S41-05.md`.

### SL-S41-06 — Navegar ao trecho

- **Executar:** Resolver hit por referência/revisão no leitor e na Peça.
- **Produzir:** `apps/desktop/src/search/open-hit.ts`.
- **Aceitar quando:** Hit antigo é detectado e não abre o trecho errado da revisão atual.
- **Controle:** Implementação; P0 na versão; depende de SL-S41-05; evidência em `reports/S41/SL-S41-06.md`.

### SL-S41-07 — Integrar taxonomia ao provider

- **Executar:** Substituir lookup experimental de candidatos quando útil sem trocar política de decisão.
- **Produzir:** `crates/taxonomy/src/search_adapter.rs`.
- **Aceitar quando:** Modelo/índice de termos permanece identificado e não se mistura com embeddings de outro espaço.
- **Controle:** Implementação; P0 na versão; depende de SL-S41-06; evidência em `reports/S41/SL-S41-07.md`.

### SL-S41-08 — Validar relevância no corpus reservado

- **Executar:** Medir léxico, semântico e híbrido com rótulos estáveis.
- **Produzir:** `reports/v0.7/relevance.json`.
- **Aceitar quando:** Ganho/perda por modalidade é registrado, sem declarar vencedor universal.
- **Controle:** Teste; P0 na versão; depende de SL-S41-07; evidência em `reports/S41/SL-S41-08.md`.

### SL-S41-09 — Executar exclusão e isolamento

- **Executar:** Remover material, editar Peça e consultar em outro workspace.
- **Produzir:** `tests/search/end-to-end/`.
- **Aceitar quando:** Nenhum conteúdo excluído/desautorizado reaparece por cache ou job atrasado.
- **Controle:** Teste; P0 na versão; depende de SL-S41-08; evidência em `reports/S41/SL-S41-09.md`.

### SL-S41-10 — Publicar alpha de recuperação

- **Executar:** Documentar backend escolhido, limites e comando de reconstrução.
- **Produzir:** `release/v0.7.0/`.
- **Aceitar quando:** Apagar índices vetoriais/léxicos não perde materiais ou texto editorial.
- **Controle:** Release; P0 na versão; depende de SL-S41-09; evidência em `reports/S41/SL-S41-10.md`.

### SL-S41-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Editar/excluir material e verificar atualização da busca sem perder as fontes canônicas.**

Registrar commit, ambiente, testes e pendências em `reports/S41/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

# v0.8.0 — Contexto estrutural

**Maturidade:** Alpha de raciocínio  
**Entrega acumulada:** PageIndex adaptado às células, Skeleton Map e recuperação relacional limitada por escopo e orçamento.

<a id="s42"></a>

## S42 — Auditar e recortar PageIndex para células

- **Objetivo:** Identificar código reaproveitável e pontos que precisam perder acoplamento documental/cloud.
- **Componente / forma:** PageIndex / upstream / Desconstrução.
- **Pré-requisitos técnicos:** S03 (Inventariar upstreams e definir recortes); S23 (Construir protocolo e supervisor de workers); S32 (Homologar modelos e construir ModelGateway); S41 (Entregar busca incremental no acervo e nas Peças)
- **Entregável do sprint:** Pacote experimental de contexto e ADR do recorte PageIndex.
- **Demonstração exigida:** Construir índice de Markdown de fixture sem PDF e sem store canônico do SDK.
- **Requisitos:** R03, R22, R23, R30, R33. **Risco de integração:** Crítico.

### SL-S42-01 — Fixar revisão PageIndex

- **Executar:** Registrar licença, dependências Python, APIs públicas e módulos internos usados.
- **Produzir:** `upstream/pageindex/manifest.json`.
- **Aceitar quando:** Paths e versões são verificados na revisão escolhida, não assumidos a partir de documentação móvel.
- **Controle:** Upstream; P0 na versão; depende de SL-S03-GATE, SL-S23-GATE, SL-S32-GATE, SL-S41-GATE; evidência em `reports/S42/SL-S42-01.md`.

### SL-S42-02 — Reproduzir baseline Markdown

- **Executar:** Exercitar page_index_md ou equivalente real com fixture e modelo controlado.
- **Produzir:** `reports/pageindex/upstream-baseline.md`.
- **Aceitar quando:** Entrada/saída e comportamento de IDs/headings estão documentados.
- **Controle:** Upstream; P0 na versão; depende de SL-S42-01; evidência em `reports/S42/SL-S42-02.md`.

### SL-S42-03 — Mapear acoplamentos

- **Executar:** Identificar store, nomes de documento, páginas, provedores, prompts e frameworks de agentes.
- **Produzir:** `docs/upstream/pageindex-couplings.md`.
- **Aceitar quando:** Cloud e operações de gestão de biblioteca não entram implicitamente no adapter de células.
- **Controle:** Upstream; P0 na versão; depende de SL-S42-02; evidência em `reports/S42/SL-S42-03.md`.

### SL-S42-04 — Delimitar módulos úteis

- **Executar:** Selecionar construção/navegação/resumos e utilitários necessários por grafo de dependências.
- **Produzir:** `upstream/pageindex/slice.json`.
- **Aceitar quando:** Não se copia função isolada com dependências wildcard desconhecidas.
- **Controle:** Upstream; P0 na versão; depende de SL-S42-03; evidência em `reports/S42/SL-S42-04.md`.

### SL-S42-05 — Criar fachada ContextProvider

- **Executar:** Expor contratos Sandland sem vazar nomes de documento/página como identidade.
- **Produzir:** `adapters/pageindex/`.
- **Aceitar quando:** App não depende de DocStore nem de IDs sequenciais internos para autoria.
- **Controle:** Implementação; P0 na versão; depende de SL-S42-04; evidência em `reports/S42/SL-S42-05.md`.

### SL-S42-06 — Criar porta de modelos

- **Executar:** Redirecionar chamadas de resumo/navegação ao ModelGateway autorizado.
- **Produzir:** `adapters/pageindex/model_port.py`.
- **Aceitar quando:** Configuração padrão não envia células a provedor remoto do upstream.
- **Controle:** Implementação; P0 na versão; depende de SL-S42-05; evidência em `reports/S42/SL-S42-06.md`.

### SL-S42-07 — Criar porta de evidências

- **Executar:** Intermediar toda leitura pelo broker e pelas referências autorizadas.
- **Produzir:** `adapters/pageindex/evidence_port.py`.
- **Aceitar quando:** Prompt de contexto não é a única barreira de acesso à biblioteca.
- **Controle:** Implementação; P0 na versão; depende de SL-S42-06; evidência em `reports/S42/SL-S42-07.md`.

### SL-S42-08 — Criar patch set testável

- **Executar:** Isolar mudanças do núcleo e conservar avisos/origem.
- **Produzir:** `upstream/pageindex/patches/`.
- **Aceitar quando:** Diferenças são reproduzíveis e possuem fixtures; não há fork integral sem necessidade.
- **Controle:** Implementação; P0 na versão; depende de SL-S42-07; evidência em `reports/S42/SL-S42-08.md`.

### SL-S42-09 — Ratificar ADR de contexto

- **Executar:** Resolver ADR-014, manter árvore como projeção e escolher responsabilidades do adapter.
- **Produzir:** `docs/adr/014-context-engine.md`.
- **Aceitar quando:** Grafo completo continua canônico fora da árvore; nome RLM é definido sem ambiguidade.
- **Controle:** Decisão; P0 na versão; depende de SL-S42-08; evidência em `reports/S42/SL-S42-09.md`.

### SL-S42-10 — Validar isolamento inicial

- **Executar:** Executar contexto fake sem SDK cloud, filesystem geral ou dependências não necessárias.
- **Produzir:** `reports/pageindex/isolation.json`.
- **Aceitar quando:** Uso offline e perímetro de leitura são demonstrados antes de ligar a UI.
- **Controle:** Teste; P0 na versão; depende de SL-S42-09; evidência em `reports/S42/SL-S42-10.md`.

### SL-S42-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Construir índice de Markdown de fixture sem PDF e sem store canônico do SDK.**

Registrar commit, ambiente, testes e pendências em `reports/S42/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s43"></a>

## S43 — Projetar a topologia em índice estrutural estável

- **Objetivo:** Transformar workspace/células em hierarquia navegável sem perder o grafo.
- **Componente / forma:** PageIndex / StructureAdapter / Adaptação.
- **Pré-requisitos técnicos:** S17 (Persistir topologia, grupos e relações); S42 (Auditar e recortar PageIndex para células)
- **Entregável do sprint:** WorkspaceSnapshot e índice estrutural por revisão.
- **Demonstração exigida:** Renomear notas, mover cartões e reconstruir mantendo IDs e referências.
- **Requisitos:** R02, R04, R11, R22, R32. **Risco de integração:** Alto.

### SL-S43-01 — Definir WorkspaceSnapshot

- **Executar:** Combinar hierarquia, relações, layout, fontes e revisões com schema explícito.
- **Produzir:** `schemas/workspace-snapshot-v0.json`.
- **Aceitar quando:** Snapshot representa ciclos/arestas cruzadas fora da árvore de navegação.
- **Controle:** Contrato; P0 na versão; depende de SL-S17-GATE, SL-S42-GATE; evidência em `reports/S43/SL-S43-01.md`.

### SL-S43-02 — Construir snapshot coerente

- **Executar:** Ler revisão consistente de board e conteúdos pelo Core.
- **Produzir:** `crates/context/src/snapshot.rs`.
- **Aceitar quando:** Não combina geometria nova com texto antigo sem registrar as revisões usadas.
- **Controle:** Implementação; P1 na versão; depende de SL-S43-01; evidência em `reports/S43/SL-S43-02.md`.

### SL-S43-03 — Projetar grupos/células/blocos

- **Executar:** Usar estrutura existente do produto em vez de inferi-la novamente por LLM.
- **Produzir:** `adapters/pageindex/structure_adapter.py`.
- **Aceitar quando:** Célula não é convertida em PDF e grupo manual não é substituído por cluster inventado.
- **Controle:** Implementação; P1 na versão; depende de SL-S43-02; evidência em `reports/S43/SL-S43-03.md`.

### SL-S43-04 — Mapear IDs estáveis

- **Executar:** Traduzir node_id interno para identidade Sandland com revision key.
- **Produzir:** `adapters/pageindex/identity.py`.
- **Aceitar quando:** Inserir um nó não renumera todas as evidências existentes.
- **Controle:** Implementação; P1 na versão; depende de SL-S43-03; evidência em `reports/S43/SL-S43-04.md`.

### SL-S43-05 — Preservar referências cruzadas

- **Executar:** Guardar arestas tipadas/direção e ocorrências múltiplas em estrutura auxiliar.
- **Produzir:** `crates/context/src/relations.rs`.
- **Aceitar quando:** Tree thinning não elimina células/arestas autorais.
- **Controle:** Implementação; P1 na versão; depende de SL-S43-04; evidência em `reports/S43/SL-S43-05.md`.

### SL-S43-06 — Representar geometria relevante

- **Executar:** Conservar bounds/grupos e distinguir posição observada de relação semântica.
- **Produzir:** `crates/context/src/layout.rs`.
- **Aceitar quando:** Proximidade não é convertida automaticamente em apoio causal ou factual.
- **Controle:** Implementação; P1 na versão; depende de SL-S43-05; evidência em `reports/S43/SL-S43-06.md`.

### SL-S43-07 — Mapear blocos/fontes

- **Executar:** Associar folhas a conteúdo e evidências com localizadores corretos.
- **Produzir:** `adapters/pageindex/source_mapping.py`.
- **Aceitar quando:** Leitura de folha resolve a revisão exata e não uma página fictícia.
- **Controle:** Implementação; P1 na versão; depende de SL-S43-06; evidência em `reports/S43/SL-S43-07.md`.

### SL-S43-08 — Persistir cache derivado

- **Executar:** Versionar schema, pipeline e hashes sem duplicar autoridade canônica.
- **Produzir:** `crates/context/src/cache.rs`.
- **Aceitar quando:** Apagar árvore/summary cache permite reconstrução dos arquivos.
- **Controle:** Implementação; P1 na versão; depende de SL-S43-07; evidência em `reports/S43/SL-S43-08.md`.

### SL-S43-09 — Testar casos topológicos

- **Executar:** Cobrir ciclos, grupos sobrepostos admitidos, notas repetidas e órfãos.
- **Produzir:** `tests/context/structure/`.
- **Aceitar quando:** Dados não representáveis na árvore continuam recuperáveis pelas relações.
- **Controle:** Teste; P1 na versão; depende de SL-S43-08; evidência em `reports/S43/SL-S43-09.md`.

### SL-S43-10 — Testar estabilidade incremental

- **Executar:** Reordenar/renomear/mover mantendo conjunto de evidências.
- **Produzir:** `reports/context/identity-stability.json`.
- **Aceitar quando:** Apenas revisões realmente afetadas mudam; nenhuma referência é redirecionada por índice posicional.
- **Controle:** Teste; P1 na versão; depende de SL-S43-09; evidência em `reports/S43/SL-S43-10.md`.

### SL-S43-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Renomear notas, mover cartões e reconstruir mantendo IDs e referências.**

Registrar commit, ambiente, testes e pendências em `reports/S43/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s44"></a>

## S44 — Criar ferramentas de contexto com escopo efetivo

- **Objetivo:** Permitir exploração pelo modelo sem lhe entregar o Vault ou a biblioteca global.
- **Componente / forma:** ToolBroker / PageIndex / Adaptação.
- **Pré-requisitos técnicos:** S09 (Implementar broker de arquivos e defesa TOCTOU); S26 (Implementar perfis de sandbox Windows); S43 (Projetar a topologia em índice estrutural estável)
- **Entregável do sprint:** Ferramentas de overview, grupo, célula, vizinhos e fonte.
- **Demonstração exigida:** Modelo recebe ID fora do escopo e o broker nega mesmo com instrução em documento.
- **Requisitos:** R06, R08, R22, R23. **Risco de integração:** Crítico.

### SL-S44-01 — Definir catálogo de ferramentas

- **Executar:** Especificar args, resultados, leitura, limites e erros de cada operação.
- **Produzir:** `schemas/context-tools-v0.json`.
- **Aceitar quando:** Nenhuma ferramenta aceita path arbitrário como substituto de EvidenceRef.
- **Controle:** Contrato; P0 na versão; depende de SL-S09-GATE, SL-S26-GATE, SL-S43-GATE; evidência em `reports/S44/SL-S44-01.md`.

### SL-S44-02 — Implementar overview limitado

- **Executar:** Retornar mapa global resumido por orçamento e snapshot.
- **Produzir:** `crates/context-tools/src/overview.rs`.
- **Aceitar quando:** Board grande não injeta automaticamente todas as células no prompt.
- **Controle:** Implementação; P0 na versão; depende de SL-S44-01; evidência em `reports/S44/SL-S44-02.md`.

### SL-S44-03 — Implementar leitura de grupo

- **Executar:** Paginar membros/relações e manter revisão coerente.
- **Produzir:** `crates/context-tools/src/group.rs`.
- **Aceitar quando:** Limites são impostos em código, não dependem de modelo obedecer instrução.
- **Controle:** Implementação; P0 na versão; depende de SL-S44-02; evidência em `reports/S44/SL-S44-03.md`.

### SL-S44-04 — Implementar leitura de célula

- **Executar:** Permitir ranges/blocos e devolução de referência de origem.
- **Produzir:** `crates/context-tools/src/cell.rs`.
- **Aceitar quando:** Leitura recusa célula de outro workspace não autorizada.
- **Controle:** Implementação; P0 na versão; depende de SL-S44-03; evidência em `reports/S44/SL-S44-04.md`.

### SL-S44-05 — Implementar vizinhos/arestas

- **Executar:** Filtrar relação, direção e profundidade; controlar ciclos.
- **Produzir:** `crates/context-tools/src/neighbors.rs`.
- **Aceitar quando:** Traversal termina no limite e não perde tipo apoia/contradiz.
- **Controle:** Implementação; P0 na versão; depende de SL-S44-04; evidência em `reports/S44/SL-S44-05.md`.

### SL-S44-06 — Implementar leitura de fonte

- **Executar:** Resolver asset/revisão/trecho autorizado por broker.
- **Produzir:** `crates/context-tools/src/source.rs`.
- **Aceitar quando:** Modelo não transforma URL/filename em permissão de filesystem.
- **Controle:** Implementação; P0 na versão; depende de SL-S44-05; evidência em `reports/S44/SL-S44-06.md`.

### SL-S44-07 — Validar capability por chamada

- **Executar:** Checar expiração, revisão e budget acumulado antes de executar.
- **Produzir:** `crates/context-tools/src/authorize.rs`.
- **Aceitar quando:** Reutilizar ferramenta com token de outra tarefa é negado.
- **Controle:** Implementação; P0 na versão; depende de SL-S44-06; evidência em `reports/S44/SL-S44-07.md`.

### SL-S44-08 — Limitar resultados e logs

- **Executar:** Cortar por contrato com paginação explícita sem vazar corpo em logs.
- **Produzir:** `crates/context-tools/src/limits.rs`.
- **Aceitar quando:** Truncamento é sinalizado e não se confunde com evidência completa.
- **Controle:** Implementação; P0 na versão; depende de SL-S44-07; evidência em `reports/S44/SL-S44-08.md`.

### SL-S44-09 — Testar injeção de prompt

- **Executar:** Colocar instruções maliciosas em células, PDFs e resultados de ferramentas.
- **Produzir:** `tests/context/tool-injection/`.
- **Aceitar quando:** Conteúdo não concede novas ferramentas, credenciais ou acesso a outro projeto.
- **Controle:** Teste; P0 na versão; depende de SL-S44-08; evidência em `reports/S44/SL-S44-09.md`.

### SL-S44-10 — Auditar ferramentas de gestão

- **Executar:** Garantir ausência de remove/upload/shell e outros comandos não aprovados no agente.
- **Produzir:** `reports/context/tool-surface.md`.
- **Aceitar quando:** Só o catálogo read-only aprovado é exposto nesta fase.
- **Controle:** Teste; P0 na versão; depende de SL-S44-09; evidência em `reports/S44/SL-S44-10.md`.

### SL-S44-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Modelo recebe ID fora do escopo e o broker nega mesmo com instrução em documento.**

Registrar commit, ambiente, testes e pendências em `reports/S44/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s45"></a>

## S45 — Implementar summaries incrementais e Skeleton Map

- **Objetivo:** Usar síntese estrutural dentro do contexto dos modelos locais e com invalidação precisa.
- **Componente / forma:** ContextService / LocalAI / Próprio.
- **Pré-requisitos técnicos:** S33 (Implementar resumo executivo e enriquecimento gerativo); S43 (Projetar a topologia em índice estrutural estável); S44 (Criar ferramentas de contexto com escopo efetivo)
- **Entregável do sprint:** Skeleton Map com budget e cache incremental de resumos.
- **Demonstração exigida:** Editar uma célula e recomputar apenas summaries dependentes, sem perder evidências.
- **Requisitos:** R09, R19, R22, R23. **Risco de integração:** Alto.

### SL-S45-01 — Definir orçamento por camada

- **Executar:** Reservar tokens para sistema, pergunta, evidências e resposta.
- **Produzir:** `docs/contracts/context-budget.md`.
- **Aceitar quando:** Limite do modelo é verificado antes da chamada e não após erro de contexto.
- **Controle:** Contrato; P0 na versão; depende de SL-S33-GATE, SL-S43-GATE, SL-S44-GATE; evidência em `reports/S45/SL-S45-01.md`.

### SL-S45-02 — Gerar resumos de célula

- **Executar:** Reutilizar resumo válido ou produzir síntese por conteúdo/revisão.
- **Produzir:** `crates/context/src/cell_summary.rs`.
- **Aceitar quando:** Resumo automático nunca substitui conteúdo nem comentário autoral editado.
- **Controle:** Implementação; P1 na versão; depende de SL-S45-01; evidência em `reports/S45/SL-S45-02.md`.

### SL-S45-03 — Gerar resumos de grupo

- **Executar:** Agregar filhos com references e relações relevantes, preservando divergências.
- **Produzir:** `crates/context/src/group_summary.rs`.
- **Aceitar quando:** Grupo não apaga evidência contraditória apenas para reduzir texto.
- **Controle:** Implementação; P1 na versão; depende de SL-S45-02; evidência em `reports/S45/SL-S45-03.md`.

### SL-S45-04 — Criar grafo de dependências

- **Executar:** Associar summaries a revisões de conteúdo, grupo, modelo e prompt.
- **Produzir:** `crates/context/src/summary_dependencies.rs`.
- **Aceitar quando:** Alteração local invalida somente ancestrais/dependências pertinentes.
- **Controle:** Implementação; P1 na versão; depende de SL-S45-03; evidência em `reports/S45/SL-S45-04.md`.

### SL-S45-05 — Criar Skeleton Map

- **Executar:** Selecionar visão geral útil sem concatenar 100 palavras de cada nó.
- **Produzir:** `crates/context/src/skeleton.rs`.
- **Aceitar quando:** Cena de 1.000 nós respeita budget e oferece mecanismo de expansão.
- **Controle:** Implementação; P1 na versão; depende de SL-S45-04; evidência em `reports/S45/SL-S45-05.md`.

### SL-S45-06 — Integrar scheduler de summaries

- **Executar:** Limitar concorrência e reagendar conforme hardware/pergunta ativa.
- **Produzir:** `crates/context/src/summary_jobs.rs`.
- **Aceitar quando:** Chamadas em massa do upstream não carregam múltiplos modelos sem controle.
- **Controle:** Implementação; P1 na versão; depende de SL-S45-05; evidência em `reports/S45/SL-S45-06.md`.

### SL-S45-07 — Marcar cache desatualizado

- **Executar:** Expor revisão do contexto à UI e reconstruir antes de resposta crítica.
- **Produzir:** `apps/desktop/src/context/status.ts`.
- **Aceitar quando:** Usuário não recebe contexto antigo apresentado como versão atual sem indicação.
- **Controle:** Implementação; P1 na versão; depende de SL-S45-06; evidência em `reports/S45/SL-S45-07.md`.

### SL-S45-08 — Evitar trabalho em mudança visual

- **Executar:** Distinguir posição, relação e corpo do texto na invalidação.
- **Produzir:** `crates/context/src/invalidation.rs`.
- **Aceitar quando:** Mover cartão não regenera resumo textual; mudar aresta atualiza mapa relacional.
- **Controle:** Implementação; P1 na versão; depende de SL-S45-07; evidência em `reports/S45/SL-S45-08.md`.

### SL-S45-09 — Medir custo e cobertura

- **Executar:** Comparar full-context viável com navegação progressiva em corpus fixo.
- **Produzir:** `reports/context/budget-eval.json`.
- **Aceitar quando:** Redução de tokens é medida e não alegada como 80% universal.
- **Controle:** Teste; P1 na versão; depende de SL-S45-08; evidência em `reports/S45/SL-S45-09.md`.

### SL-S45-10 — Exercitar offline/OOM

- **Executar:** Testar interrupção no meio de resumo de grupo e reinício.
- **Produzir:** `tests/context/summary-recovery/`.
- **Aceitar quando:** Cache parcial não vira resultado válido e a Mesa permanece editável.
- **Controle:** Teste; P1 na versão; depende de SL-S45-09; evidência em `reports/S45/SL-S45-10.md`.

### SL-S45-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Editar uma célula e recomputar apenas summaries dependentes, sem perder evidências.**

Registrar commit, ambiente, testes e pendências em `reports/S45/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s46"></a>

## S46 — Entregar recuperação estrutural e síntese progressiva

- **Objetivo:** Fechar o motor da Mesa com evidências, abstenção e limites de execução.
- **Componente / forma:** Context Engine / orquestração / Integração.
- **Pré-requisitos técnicos:** S44 (Criar ferramentas de contexto com escopo efetivo); S45 (Implementar summaries incrementais e Skeleton Map)
- **Entregável do sprint:** v0.8.0: consulta estrutural local sobre células e fontes.
- **Demonstração exigida:** Perguntar sobre grupos que se contradizem e receber resposta apoiada nas revisões corretas.
- **Requisitos:** R07, R13, R22, R23, R27. **Risco de integração:** Alto.

### SL-S46-01 — Criar planejador de exploração

- **Executar:** Selecionar grupos a partir do Skeleton Map e pergunta autorizada.
- **Produzir:** `crates/context-agent/src/planner.rs`.
- **Aceitar quando:** Plano não é interpretado como autorização para ferramenta fora do catálogo.
- **Controle:** Implementação; P1 na versão; depende de SL-S44-GATE, SL-S45-GATE; evidência em `reports/S46/SL-S46-01.md`.

### SL-S46-02 — Implementar expansão limitada

- **Executar:** Navegar árvore e relações com limites de hops/chamadas/tempo.
- **Produzir:** `crates/context-agent/src/explorer.rs`.
- **Aceitar quando:** Ciclo no grafo não causa execução sem fim nem consumo ilimitado.
- **Controle:** Implementação; P1 na versão; depende de SL-S46-01; evidência em `reports/S46/SL-S46-02.md`.

### SL-S46-03 — Executar subconsultas

- **Executar:** Compartilhar snapshot e serializar inferência quando o perfil exigir.
- **Produzir:** `crates/context-agent/src/subqueries.rs`.
- **Aceitar quando:** Paralelismo lógico não implica carregar vários modelos em RAM.
- **Controle:** Implementação; P1 na versão; depende de SL-S46-02; evidência em `reports/S46/SL-S46-03.md`.

### SL-S46-04 — Consolidar evidências

- **Executar:** Manter quotes/referências junto às micro-sínteses e detectar incompatibilidades.
- **Produzir:** `crates/context-agent/src/evidence.rs`.
- **Aceitar quando:** Resumos intermediários não são a única fonte disponível para a resposta.
- **Controle:** Implementação; P1 na versão; depende de SL-S46-03; evidência em `reports/S46/SL-S46-04.md`.

### SL-S46-05 — Gerar resposta com referências

- **Executar:** Validar IDs/localizadores e separar inferência de afirmação citada.
- **Produzir:** `crates/context-agent/src/answer.rs`.
- **Aceitar quando:** Citação inexistente é rejeitada/sinalizada, não renderizada como prova válida.
- **Controle:** Implementação; P1 na versão; depende de SL-S46-04; evidência em `reports/S46/SL-S46-05.md`.

### SL-S46-06 — Implementar abstenção

- **Executar:** Reconhecer evidência insuficiente, acesso negado e budget esgotado.
- **Produzir:** `crates/context-agent/src/abstain.rs`.
- **Aceitar quando:** Sistema não inventa resposta para ocultar ausência de fonte.
- **Controle:** Implementação; P1 na versão; depende de SL-S46-05; evidência em `reports/S46/SL-S46-06.md`.

### SL-S46-07 — Integrar cancelamento e revisão

- **Executar:** Descartar/apresentar como antiga resposta construída sobre snapshot alterado.
- **Produzir:** `crates/context-agent/src/session.rs`.
- **Aceitar quando:** Resultado atrasado não modifica texto da Peça nem se anuncia como atual.
- **Controle:** Implementação; P1 na versão; depende de SL-S46-06; evidência em `reports/S46/SL-S46-07.md`.

### SL-S46-08 — Avaliar relações e layout

- **Executar:** Perguntas contrastivas com mesmas palavras, mas grupos/arestas diferentes.
- **Produzir:** `tests/eval/context-topology/`.
- **Aceitar quando:** Motor usa estrutura relevante em vez de responder só por similaridade textual.
- **Controle:** Teste; P1 na versão; depende de SL-S46-07; evidência em `reports/S46/SL-S46-08.md`.

### SL-S46-09 — Executar avaliação de citações

- **Executar:** Verificar fontes, revisões, ausência de resposta e leaks entre workspaces.
- **Produzir:** `reports/v0.8/context-quality.json`.
- **Aceitar quando:** Precisão de referência e escopo são gates, não somente fluência do texto.
- **Controle:** Teste; P1 na versão; depende de SL-S46-08; evidência em `reports/S46/SL-S46-09.md`.

### SL-S46-10 — Publicar alpha de raciocínio

- **Executar:** Entregar modo local, diagnóstico de contexto e limites conhecidos.
- **Produzir:** `release/v0.8.0/`.
- **Aceitar quando:** PageIndex adaptado não assume autoridade canônica nem acesso global às células.
- **Controle:** Release; P1 na versão; depende de SL-S46-09; evidência em `reports/S46/SL-S46-10.md`.

### SL-S46-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Perguntar sobre grupos que se contradizem e receber resposta apoiada nas revisões corretas.**

Registrar commit, ambiente, testes e pendências em `reports/S46/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

# v0.9.0 — Ciclo analítico completo

**Maturidade:** Alpha de ciclo completo  
**Entrega acumulada:** Pesquisa web, widget, copiloto, chat e intenções conectam Captura → Mesa → Peça.

<a id="s47"></a>

## S47 — Integrar SearxNG como serviço de descoberta

- **Objetivo:** Usar metabusca sem incorporar sua UI ou criar dependência pública implícita.
- **Componente / forma:** SearxNG / WebDiscoveryProvider / Desconstrução.
- **Pré-requisitos técnicos:** S23 (Construir protocolo e supervisor de workers); S26 (Implementar perfis de sandbox Windows); S34 (Integrar Jev como motor de decisões); S46 (Entregar recuperação estrutural e síntese progressiva)
- **Entregável do sprint:** Provider SearxNG com configuração, consentimento e testes de resposta.
- **Demonstração exigida:** Pesquisar por consulta autorizada e apresentar resultados sem enviar o board inteiro.
- **Requisitos:** R08, R24, R30, R33. **Risco de integração:** Crítico.

### SL-S47-01 — Fixar origem SearxNG

- **Executar:** Registrar revisão, runtime Python, engines habilitados e obrigações AGPL.
- **Produzir:** `upstream/searxng/manifest.json`.
- **Aceitar quando:** Fonte correspondente e configuração distribuída são identificáveis.
- **Controle:** Upstream; P0 na versão; depende de SL-S23-GATE, SL-S26-GATE, SL-S34-GATE, SL-S46-GATE; evidência em `reports/S47/SL-S47-01.md`.

### SL-S47-02 — Escolher instância e lifecycle

- **Executar:** Resolver ADR-015: local gerenciada ou instância pessoal explícita.
- **Produzir:** `docs/adr/015-web-discovery.md`.
- **Aceitar quando:** Instância pública não é fallback silencioso nem dependência obrigatória para abrir o app.
- **Controle:** Decisão; P0 na versão; depende de SL-S47-01; evidência em `reports/S47/SL-S47-02.md`.

### SL-S47-03 — Criar configuração mínima

- **Executar:** Habilitar API JSON e apenas engines necessários; separar serviço de sua interface web.
- **Produzir:** `workers/searxng/settings.yml`.
- **Aceitar quando:** Formato não habilitado gera diagnóstico em vez de assumir que toda instância responde JSON.
- **Controle:** Implementação; P0 na versão; depende de SL-S47-02; evidência em `reports/S47/SL-S47-03.md`.

### SL-S47-04 — Integrar supervisor

- **Executar:** Iniciar/verificar/encerrar serviço local quando essa modalidade for escolhida.
- **Produzir:** `adapters/searxng/lifecycle.rs`.
- **Aceitar quando:** Serviço não inicia no startup da autoria sem necessidade.
- **Controle:** Implementação; P0 na versão; depende de SL-S47-03; evidência em `reports/S47/SL-S47-04.md`.

### SL-S47-05 — Definir resultado de descoberta

- **Executar:** Normalizar título, URL, snippet, engine, consulta e timestamp sem chamar isso de evidência lida.
- **Produzir:** `schemas/web-search-result.json`.
- **Aceitar quando:** Resultado preserva origem e não inventa conteúdo da página completa.
- **Controle:** Contrato; P0 na versão; depende de SL-S47-04; evidência em `reports/S47/SL-S47-05.md`.

### SL-S47-06 — Implementar cliente

- **Executar:** Tratar idioma, filtros, timeout, cancelamento e falhas parciais de engines.
- **Produzir:** `adapters/searxng/client.rs`.
- **Aceitar quando:** Uma engine bloqueada não faz loop infinito nem retorna sucesso vazio sem contexto.
- **Controle:** Implementação; P0 na versão; depende de SL-S47-05; evidência em `reports/S47/SL-S47-06.md`.

### SL-S47-07 — Aplicar política de consulta

- **Executar:** Autorizar texto enviado, registrar finalidade e bloquear contexto sensível não aprovado.
- **Produzir:** `crates/web/src/query_policy.rs`.
- **Aceitar quando:** Consulta gerada pela IA é inspecionável; a mesa inteira não é enviada automaticamente.
- **Controle:** Implementação; P0 na versão; depende de SL-S47-06; evidência em `reports/S47/SL-S47-07.md`.

### SL-S47-08 — Deduplicar resultados

- **Executar:** Canonicalizar URLs com cuidado, preservando versões/origens relevantes.
- **Produzir:** `crates/web/src/result_merge.rs`.
- **Aceitar quando:** Parâmetros semanticamente relevantes não são removidos indiscriminadamente.
- **Controle:** Implementação; P0 na versão; depende de SL-S47-07; evidência em `reports/S47/SL-S47-08.md`.

### SL-S47-09 — Criar API fake e smoke autorizado

- **Executar:** Cobrir 403 JSON, indisponibilidade, respostas parciais e uma pesquisa real consentida.
- **Produzir:** `tests/web/searxng/`.
- **Aceitar quando:** CI determinística não depende de buscadores públicos; teste real tem estado separado.
- **Controle:** Teste; P0 na versão; depende de SL-S47-08; evidência em `reports/S47/SL-S47-09.md`.

### SL-S47-10 — Preparar distribuição e avisos

- **Executar:** Incluir fontes/configuração e documentação da divulgação de consultas externas.
- **Produzir:** `compliance/searxng/`.
- **Aceitar quando:** Auto-hospedagem não é anunciada como busca offline ou anonimato absoluto.
- **Controle:** Licença; P0 na versão; depende de SL-S47-09; evidência em `reports/S47/SL-S47-10.md`.

### SL-S47-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Pesquisar por consulta autorizada e apresentar resultados sem enviar o board inteiro.**

Registrar commit, ambiente, testes e pendências em `reports/S47/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s48"></a>

## S48 — Integrar Scrapling no caminho HTTP

- **Objetivo:** Adquirir páginas com limites e origem preservada antes de habilitar um browser.
- **Componente / forma:** Scrapling / WebFetchProvider / Desconstrução.
- **Pré-requisitos técnicos:** S23 (Construir protocolo e supervisor de workers); S26 (Implementar perfis de sandbox Windows); S29 (Integrar Readability/Turndown para clipping seguro); S47 (Integrar SearxNG como serviço de descoberta)
- **Entregável do sprint:** Worker HTTP Scrapling e política de destinos.
- **Demonstração exigida:** Baixar página permitida e negar URL interna/redirect indevido em harness controlado.
- **Requisitos:** R06, R08, R25, R30. **Risco de integração:** Crítico.

### SL-S48-01 — Fixar Scrapling e dependências

- **Executar:** Registrar fetchers/parser necessários e excluir inicialmente extras de browser/crawl.
- **Produzir:** `upstream/scrapling/manifest.json`.
- **Aceitar quando:** Pacote HTTP não instala navegador por simples import sem política aprovada.
- **Controle:** Upstream; P0 na versão; depende de SL-S23-GATE, SL-S26-GATE, SL-S29-GATE, SL-S47-GATE; evidência em `reports/S48/SL-S48-01.md`.

### SL-S48-02 — Reproduzir baseline HTTP

- **Executar:** Exercitar requests, redirecionamentos e HTML de fixture sem customização.
- **Produzir:** `reports/scrapling/http-baseline.md`.
- **Aceitar quando:** Mudança de comportamento no adapter pode ser comparada à revisão original.
- **Controle:** Upstream; P0 na versão; depende de SL-S48-01; evidência em `reports/S48/SL-S48-02.md`.

### SL-S48-03 — Definir FetchRequest/Snapshot

- **Executar:** Incluir URL autorizada, limites, headers permitidos, cadeia de redirect e bytes/hash.
- **Produzir:** `schemas/web-fetch-v0.json`.
- **Aceitar quando:** Snapshot diferencia URL solicitada, final e data de aquisição.
- **Controle:** Contrato; P0 na versão; depende de SL-S48-02; evidência em `reports/S48/SL-S48-03.md`.

### SL-S48-04 — Implementar política SSRF

- **Executar:** Restringir schemes, destinos, DNS e redes privadas conforme autorização.
- **Produzir:** `crates/web/src/url_policy.rs`.
- **Aceitar quando:** localhost, metadados, file URLs e destinos não permitidos são rejeitados.
- **Controle:** Implementação; P0 na versão; depende de SL-S48-03; evidência em `reports/S48/SL-S48-04.md`.

### SL-S48-05 — Validar cada redirecionamento

- **Executar:** Reaplicar política e tratamento de resolução antes de seguir destino.
- **Produzir:** `workers/scrapling/redirect_policy.py`.
- **Aceitar quando:** URL pública que redireciona ao runtime local não contorna o bloqueio.
- **Controle:** Implementação; P0 na versão; depende de SL-S48-04; evidência em `reports/S48/SL-S48-05.md`.

### SL-S48-06 — Aplicar limites de resposta

- **Executar:** Controlar bytes descomprimidos, tempo, encoding e tipo detectado.
- **Produzir:** `workers/scrapling/http_fetch.py`.
- **Aceitar quando:** Resposta infinita/comprimida maliciosa é interrompida sem esgotar recursos do Core.
- **Controle:** Implementação; P0 na versão; depende de SL-S48-05; evidência em `reports/S48/SL-S48-06.md`.

### SL-S48-07 — Integrar sessões restritas

- **Executar:** Isolar cookies/credenciais por contexto e não herdar perfil pessoal do navegador.
- **Produzir:** `workers/scrapling/sessions.py`.
- **Aceitar quando:** Pesquisa pública não ganha cookies de outros workspaces ou da sessão do usuário.
- **Controle:** Implementação; P0 na versão; depende de SL-S48-06; evidência em `reports/S48/SL-S48-07.md`.

### SL-S48-08 — Encaminhar conteúdo adquirido

- **Executar:** Enviar HTML ao normalizador e arquivos ao Ingest por propostas validadas.
- **Produzir:** `crates/web/src/fetch_pipeline.rs`.
- **Aceitar quando:** Worker não grava snapshots diretamente na área canônica.
- **Controle:** Implementação; P0 na versão; depende de SL-S48-07; evidência em `reports/S48/SL-S48-08.md`.

### SL-S48-09 — Exercitar corpus de rede hostil

- **Executar:** Simular rebinding/redirects, tipos enganosos, timeout e resposta excessiva.
- **Produzir:** `tests/web/http-security/`.
- **Aceitar quando:** Todas as exceções de destino são explicitamente autorizadas e auditáveis.
- **Controle:** Teste; P0 na versão; depende de SL-S48-08; evidência em `reports/S48/SL-S48-09.md`.

### SL-S48-10 — Medir HTTP-first

- **Executar:** Comparar sucesso/falha/custo no corpus permitido sem prometer taxa universal.
- **Produzir:** `reports/scrapling/http-eval.json`.
- **Aceitar quando:** Falhas são exibidas; não se declara resolver 85% da web sem evidência correspondente.
- **Controle:** Teste; P0 na versão; depende de SL-S48-09; evidência em `reports/S48/SL-S48-10.md`.

### SL-S48-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Baixar página permitida e negar URL interna/redirect indevido em harness controlado.**

Registrar commit, ambiente, testes e pendências em `reports/S48/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s49"></a>

## S49 — Adicionar aquisição dinâmica com browser confinado

- **Objetivo:** Habilitar páginas dinâmicas sem transformar o app em um navegador irrestrito.
- **Componente / forma:** Scrapling / browser / Adaptação.
- **Pré-requisitos técnicos:** S24 (Implementar perfis de sandbox Linux); S25 (Implementar perfis de sandbox macOS); S26 (Implementar perfis de sandbox Windows); S48 (Integrar Scrapling no caminho HTTP)
- **Entregável do sprint:** Browser worker lazy com política de rede e lifecycle.
- **Demonstração exigida:** Renderizar SPA de fixture, capturar DOM e encerrar todos os processos após a tarefa.
- **Requisitos:** R01, R06, R08, R09, R25, R30. **Risco de integração:** Crítico.

### SL-S49-01 — Definir critérios de escalada

- **Executar:** Escalar HTTP→browser por falha/tipo real, com consentimento e limites.
- **Produzir:** `docs/web/browser-policy.md`.
- **Aceitar quando:** Bloqueio não dispara downloads ou tentativas ilimitadas de bypass.
- **Controle:** Decisão; P0 na versão; depende de SL-S24-GATE, SL-S25-GATE, SL-S26-GATE, SL-S48-GATE; evidência em `reports/S49/SL-S49-01.md`.

### SL-S49-02 — Provisionar browser homologado

- **Executar:** Fixar versão compatível com motor, assinatura/hash e artefatos por SO.
- **Produzir:** `browser/manifests/`.
- **Aceitar quando:** Instalação é consentida e verificável; browser não é baixado de URL fornecida pelo modelo.
- **Controle:** Implementação; P0 na versão; depende de SL-S49-01; evidência em `reports/S49/SL-S49-02.md`.

### SL-S49-03 — Criar sessão efêmera

- **Executar:** Isolar profile, cache, cookies e downloads por tarefa ou escopo aprovado.
- **Produzir:** `workers/scrapling/browser_session.py`.
- **Aceitar quando:** Perfil pessoal e segredos locais não são compartilhados.
- **Controle:** Implementação; P0 na versão; depende de SL-S49-02; evidência em `reports/S49/SL-S49-03.md`.

### SL-S49-04 — Aplicar contenção real

- **Executar:** Conectar perfis por SO à árvore multiprocesso do navegador.
- **Produzir:** `workers/scrapling/browser_sandbox.py`.
- **Aceitar quando:** Não se usa no-sandbox como correção silenciosa; capacidade indisponível permanece bloqueada.
- **Controle:** Implementação; P0 na versão; depende de SL-S49-03; evidência em `reports/S49/SL-S49-04.md`.

### SL-S49-05 — Controlar requisições do browser

- **Executar:** Abranger frames, subresources, WebSockets e redirecionamentos relevantes.
- **Produzir:** `workers/scrapling/browser_network.py`.
- **Aceitar quando:** JavaScript da página não alcança LocalAI/SearxNG privados nem redes proibidas por caminho alternativo.
- **Controle:** Implementação; P0 na versão; depende de SL-S49-04; evidência em `reports/S49/SL-S49-05.md`.

### SL-S49-06 — Capturar DOM e estado

- **Executar:** Definir condição de prontidão, timeout e resultado parcial.
- **Produzir:** `workers/scrapling/dynamic_fetch.py`.
- **Aceitar quando:** Espera por network idle não pode bloquear indefinidamente em página que mantém conexões.
- **Controle:** Implementação; P0 na versão; depende de SL-S49-05; evidência em `reports/S49/SL-S49-06.md`.

### SL-S49-07 — Controlar downloads/ações

- **Executar:** Permitir somente aquisição aprovada, sem upload, shell ou automação de contas.
- **Produzir:** `workers/scrapling/browser_actions.py`.
- **Aceitar quando:** Conteúdo remoto não solicita acesso ao Vault ou credenciais por tool call.
- **Controle:** Implementação; P0 na versão; depende de SL-S49-06; evidência em `reports/S49/SL-S49-07.md`.

### SL-S49-08 — Compartilhar pool limitado

- **Executar:** Reusar recursos quando seguro e fechar sessões ociosas.
- **Produzir:** `crates/web/src/browser_pool.rs`.
- **Aceitar quando:** Número de widgets não cria número igual de browsers persistentes.
- **Controle:** Implementação; P0 na versão; depende de SL-S49-07; evidência em `reports/S49/SL-S49-08.md`.

### SL-S49-09 — Testar páginas hostis e crash

- **Executar:** Simular popup, arquivo de download, loop JS, processo filho e rede interna.
- **Produzir:** `tests/web/browser-security/`.
- **Aceitar quando:** Cancelar mata a árvore autorizada e preserva somente resultados validados.
- **Controle:** Teste; P0 na versão; depende de SL-S49-08; evidência em `reports/S49/SL-S49-09.md`.

### SL-S49-10 — Homologar consumo por plataforma

- **Executar:** Medir pico/idle e suporte das capacidades em alvos reais.
- **Produzir:** `reports/browser/qualification.json`.
- **Aceitar quando:** Capacidade só é liberada onde sandbox e budgets do perfil foram aprovados.
- **Controle:** Teste; P0 na versão; depende de SL-S49-09; evidência em `reports/S49/SL-S49-10.md`.

### SL-S49-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Renderizar SPA de fixture, capturar DOM e encerrar todos os processos após a tarefa.**

Registrar commit, ambiente, testes e pendências em `reports/S49/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s50"></a>

## S50 — Construir widget de pesquisa e snapshots

- **Objetivo:** Conectar descoberta à Mesa sem confundir página live, snapshot e síntese autoral.
- **Componente / forma:** Whiteboard / pesquisa / Próprio.
- **Pré-requisitos técnicos:** S19 (Concluir navegação, LOD e workspaces); S30 (Entregar Ingest sem fricção e leitura do acervo); S47 (Integrar SearxNG como serviço de descoberta); S48 (Integrar Scrapling no caminho HTTP); S49 (Adicionar aquisição dinâmica com browser confinado)
- **Entregável do sprint:** Widget pesquisável, snapshots e promoção de resultados a células.
- **Demonstração exigida:** Pesquisar, selecionar fontes, capturar, editar síntese e atualizar sem perda autoral.
- **Requisitos:** R11, R14, R24, R25, R26, R31. **Risco de integração:** Médio.

### SL-S50-01 — Definir estado canônico do widget

- **Executar:** Persistir consulta, filtros, resultados escolhidos, execuções e revisão de síntese.
- **Produzir:** `schemas/research-widget-v0.json`.
- **Aceitar quando:** Estado pode ser reconstruído sem browser aberto ou cache do serviço.
- **Controle:** Contrato; P0 na versão; depende de SL-S19-GATE, SL-S30-GATE, SL-S47-GATE, SL-S48-GATE, SL-S49-GATE; evidência em `reports/S50/SL-S50-01.md`.

### SL-S50-02 — Criar UI de consulta

- **Executar:** Exibir destino/consentimento, filtros e execução cancelável.
- **Produzir:** `apps/desktop/src/widgets/research/query.ts`.
- **Aceitar quando:** Consulta privada não é enviada ao digitar sem ação/política explícita.
- **Controle:** Implementação; P1 na versão; depende de SL-S50-01; evidência em `reports/S50/SL-S50-02.md`.

### SL-S50-03 — Renderizar resultados seguros

- **Executar:** Mostrar títulos/snippets saneados e origem sem executar HTML.
- **Produzir:** `apps/desktop/src/widgets/research/results.ts`.
- **Aceitar quando:** Resultado de buscador não recebe aparência de texto completo já verificado.
- **Controle:** Implementação; P1 na versão; depende de SL-S50-02; evidência em `reports/S50/SL-S50-03.md`.

### SL-S50-04 — Selecionar e adquirir fontes

- **Executar:** Disparar fetch de itens aprovados e mostrar status individual.
- **Produzir:** `crates/widgets/src/research/acquire.rs`.
- **Aceitar quando:** Falha de uma fonte não perde seleção nem captura das demais.
- **Controle:** Implementação; P1 na versão; depende de SL-S50-03; evidência em `reports/S50/SL-S50-04.md`.

### SL-S50-05 — Persistir snapshots

- **Executar:** Salvar HTML/texto/assets autorizados com URL final, data, hash e mapa de fontes.
- **Produzir:** `crates/widgets/src/research/snapshot.rs`.
- **Aceitar quando:** Consulta posterior à web não altera o snapshot citado.
- **Controle:** Implementação; P1 na versão; depende de SL-S50-04; evidência em `reports/S50/SL-S50-05.md`.

### SL-S50-06 — Promover a Ingest/células

- **Executar:** Criar recorte ou referência de material sem duplicação destrutiva.
- **Produzir:** `crates/widgets/src/research/promote.rs`.
- **Aceitar quando:** Conteúdo e origem chegam ao mesmo protocolo de proveniência usado na Peça.
- **Controle:** Implementação; P1 na versão; depende de SL-S50-05; evidência em `reports/S50/SL-S50-06.md`.

### SL-S50-07 — Integrar síntese opcional

- **Executar:** Usar fontes adquiridas e Context/GenerationProvider, não apenas snippets.
- **Produzir:** `crates/widgets/src/research/summarize.rs`.
- **Aceitar quando:** Afirmações indicam quais fontes foram efetivamente lidas e cobertura parcial.
- **Controle:** Implementação; P1 na versão; depende de SL-S50-06; evidência em `reports/S50/SL-S50-07.md`.

### SL-S50-08 — Tratar atualização

- **Executar:** Comparar resultados/snapshots novos com síntese existente.
- **Produzir:** `apps/desktop/src/widgets/research/refresh.ts`.
- **Aceitar quando:** Atualização não sobrescreve texto editado nem citações anteriores silenciosamente.
- **Controle:** Implementação; P1 na versão; depende de SL-S50-07; evidência em `reports/S50/SL-S50-08.md`.

### SL-S50-09 — Validar uso offline

- **Executar:** Abrir widget salvo e navegar pelas fontes preservadas com rede bloqueada.
- **Produzir:** `tests/widgets/research-offline/`.
- **Aceitar quando:** Pesquisa nova fica indisponível com explicação; leitura de snapshots continua funcional.
- **Controle:** Teste; P1 na versão; depende de SL-S50-08; evidência em `reports/S50/SL-S50-09.md`.

### SL-S50-10 — Executar jornada de pesquisa

- **Executar:** Buscar→capturar→recortar→conectar→inserir na Peça e verificar fonte/revisão.
- **Produzir:** `reports/widget/research-journey.md`.
- **Aceitar quando:** Cadeia integral permanece válida após reinício e limpeza de caches.
- **Controle:** Teste; P1 na versão; depende de SL-S50-09; evidência em `reports/S50/SL-S50-10.md`.

### SL-S50-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Pesquisar, selecionar fontes, capturar, editar síntese e atualizar sem perda autoral.**

Registrar commit, ambiente, testes e pendências em `reports/S50/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s51"></a>

## S51 — Integrar copiloto editorial e BYOK

- **Objetivo:** Gerar propostas de escrita contextualizadas sem conceder edição autônoma irrestrita.
- **Componente / forma:** Copiloto / GenerationProvider / Integração.
- **Pré-requisitos técnicos:** S21 (Implementar fork-on-insert e cadeia de evidências); S32 (Homologar modelos e construir ModelGateway); S46 (Entregar recuperação estrutural e síntese progressiva); S50 (Construir widget de pesquisa e snapshots)
- **Entregável do sprint:** Copiloto local/BYOK com diff, aceite e rollback.
- **Demonstração exigida:** Solicitar revisão de parágrafo, comparar proposta e aceitar somente blocos selecionados.
- **Requisitos:** R08, R13, R23, R27, R31. **Risco de integração:** Crítico.

### SL-S51-01 — Definir ações editoriais

- **Executar:** Reescrever, resumir, criticar e expandir como propostas com revisão-base.
- **Produzir:** `schemas/editorial-proposal-v0.json`.
- **Aceitar quando:** Modelo não recebe operação genérica de substituir arquivo ou executar comando.
- **Controle:** Contrato; P0 na versão; depende de SL-S21-GATE, SL-S32-GATE, SL-S46-GATE, SL-S50-GATE; evidência em `reports/S51/SL-S51-01.md`.

### SL-S51-02 — Criar adapters BYOK

- **Executar:** Integrar protocolos dos provedores aprovados preservando capabilities distintas.
- **Produzir:** `adapters/generation/remote/`.
- **Aceitar quando:** Compatibilidade OpenAI não é presumida para toda API; ausência de tools/schema é explícita.
- **Controle:** Implementação; P0 na versão; depende de SL-S51-01; evidência em `reports/S51/SL-S51-02.md`.

### SL-S51-03 — Gerenciar segredos e consentimento

- **Executar:** Aplicar escopo da Peça/fontes e armazenar chave no SO.
- **Produzir:** `crates/model-gateway/src/remote_policy.rs`.
- **Aceitar quando:** Trocar para remoto não envia automaticamente o contexto inteiro usado no modo local.
- **Controle:** Implementação; P0 na versão; depende de SL-S51-02; evidência em `reports/S51/SL-S51-03.md`.

### SL-S51-04 — Montar contexto editorial

- **Executar:** Combinar seleção, objetivos e evidências autorizadas sob budget.
- **Produzir:** `crates/copilot/src/context.rs`.
- **Aceitar quando:** Conteúdo fora do escopo não entra por link/citação indireta sem autorização.
- **Controle:** Implementação; P0 na versão; depende de SL-S51-03; evidência em `reports/S51/SL-S51-04.md`.

### SL-S51-05 — Executar geração cancelável

- **Executar:** Exibir streaming e estado parcial sem aplicar texto durante geração.
- **Produzir:** `crates/copilot/src/session.rs`.
- **Aceitar quando:** Cancelar mantém a Peça original e não registra proposta parcial como aceita.
- **Controle:** Implementação; P0 na versão; depende de SL-S51-04; evidência em `reports/S51/SL-S51-05.md`.

### SL-S51-06 — Criar diff por blocos

- **Executar:** Mostrar adições/remoções e referências alteradas.
- **Produzir:** `apps/desktop/src/copilot/diff.ts`.
- **Aceitar quando:** Usuário distingue mudança de argumento de simples formatação.
- **Controle:** Implementação; P0 na versão; depende de SL-S51-05; evidência em `reports/S51/SL-S51-06.md`.

### SL-S51-07 — Aplicar proposta atomicamente

- **Executar:** Validar base_revision e permitir aceite parcial conforme contrato.
- **Produzir:** `crates/copilot/src/apply.rs`.
- **Aceitar quando:** Peça editada durante geração gera conflito/rebase explícito, não overwrite.
- **Controle:** Implementação; P0 na versão; depende de SL-S51-06; evidência em `reports/S51/SL-S51-07.md`.

### SL-S51-08 — Registrar proveniência de IA

- **Executar:** Gravar provedor/modelo e evento aceito sem expor segredos.
- **Produzir:** `crates/copilot/src/provenance.rs`.
- **Aceitar quando:** IA é marcada como origem da proposta, não fonte independente do conteúdo factual.
- **Controle:** Implementação; P0 na versão; depende de SL-S51-07; evidência em `reports/S51/SL-S51-08.md`.

### SL-S51-09 — Testar falhas e injeção

- **Executar:** Cobrir resposta sem referência, patch inválido, prompt hostil e provedor offline.
- **Produzir:** `tests/copilot/`.
- **Aceitar quando:** Nenhuma falha concede escrita fora do bloco/escopo autorizado.
- **Controle:** Teste; P0 na versão; depende de SL-S51-08; evidência em `reports/S51/SL-S51-09.md`.

### SL-S51-10 — Validar revisão e rollback

- **Executar:** Aceitar parte, desfazer e reconstruir histórico em modo offline.
- **Produzir:** `reports/copilot/editorial-acceptance.md`.
- **Aceitar quando:** Ação autoral é recuperável e o resultado não depende de conversa hospedada externamente.
- **Controle:** Teste; P0 na versão; depende de SL-S51-09; evidência em `reports/S51/SL-S51-10.md`.

### SL-S51-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Solicitar revisão de parágrafo, comparar proposta e aceitar somente blocos selecionados.**

Registrar commit, ambiente, testes e pendências em `reports/S51/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s52"></a>

## S52 — Implementar intenções, chat global e sugestões

- **Objetivo:** Conectar metas ao acervo sem ampliar o escopo de agentes implicitamente.
- **Componente / forma:** IntentService / chat / Próprio.
- **Pré-requisitos técnicos:** S35 (Implementar taxonomia e fallback de classificação); S41 (Entregar busca incremental no acervo e nas Peças); S46 (Entregar recuperação estrutural e síntese progressiva); S51 (Integrar copiloto editorial e BYOK)
- **Entregável do sprint:** Intenções canônicas, chat contextual e sugestões revisáveis.
- **Demonstração exigida:** Material novo relevante produz sugestão explicável que só cria/altera board após confirmação.
- **Requisitos:** R08, R18, R21, R23, R28. **Risco de integração:** Crítico.

### SL-S52-01 — Definir intenções

- **Executar:** Persistir objetivos, hipóteses, temas, estado e limites de escopo.
- **Produzir:** `schemas/intention-v0.json`.
- **Aceitar quando:** Intenção possui identidade/revisão e não existe apenas em memória de chat.
- **Controle:** Contrato; P0 na versão; depende de SL-S35-GATE, SL-S41-GATE, SL-S46-GATE, SL-S51-GATE; evidência em `reports/S52/SL-S52-01.md`.

### SL-S52-02 — Criar UI de metas

- **Executar:** Editar, pausar e arquivar intenções com linguagem clara.
- **Produzir:** `apps/desktop/src/intentions/`.
- **Aceitar quando:** Pausar intenção interrompe novos jobs proativos sem apagar seus dados.
- **Controle:** Implementação; P0 na versão; depende de SL-S52-01; evidência em `reports/S52/SL-S52-02.md`.

### SL-S52-03 — Recuperar afinidades

- **Executar:** Usar busca aprovada para selecionar materiais candidatos por intenção.
- **Produzir:** `crates/intentions/src/candidates.rs`.
- **Aceitar quando:** Threshold antigo de 0,85 não é aplicado sem calibração para o modelo escolhido.
- **Controle:** Implementação; P0 na versão; depende de SL-S52-02; evidência em `reports/S52/SL-S52-03.md`.

### SL-S52-04 — Avaliar relevância opcional

- **Executar:** Usar Jev ou alternativa local conforme política e registrar critério.
- **Produzir:** `crates/intentions/src/evaluate.rs`.
- **Aceitar quando:** Intenção sensível não é enviada remotamente apenas porque Jev está configurado globalmente.
- **Controle:** Implementação; P0 na versão; depende de SL-S52-03; evidência em `reports/S52/SL-S52-04.md`.

### SL-S52-05 — Deduplicar notificações

- **Executar:** Aplicar cooldown, motivo e limite de volume sem gerar spam.
- **Produzir:** `crates/intentions/src/notifications.rs`.
- **Aceitar quando:** Mesmo item reindexado não dispara sugestões duplicadas incessantemente.
- **Controle:** Implementação; P0 na versão; depende de SL-S52-04; evidência em `reports/S52/SL-S52-05.md`.

### SL-S52-06 — Criar proposta de board

- **Executar:** Apresentar candidatos, relações sugeridas e fontes para confirmação.
- **Produzir:** `crates/intentions/src/board_proposal.rs`.
- **Aceitar quando:** Arestas propostas por IA não aparecem como desenhadas pelo usuário antes do aceite.
- **Controle:** Implementação; P0 na versão; depende de SL-S52-05; evidência em `reports/S52/SL-S52-06.md`.

### SL-S52-07 — Criar chat global com escopo

- **Executar:** Selecionar workspace/acervo permitido e mostrar o escopo ativo.
- **Produzir:** `apps/desktop/src/chat/`.
- **Aceitar quando:** Global não significa acesso irrestrito automático a todos os arquivos do computador.
- **Controle:** Implementação; P0 na versão; depende de SL-S52-06; evidência em `reports/S52/SL-S52-07.md`.

### SL-S52-08 — Persistir conversas/propostas

- **Executar:** Definir o que é histórico autoral versus execução efêmera.
- **Produzir:** `crates/chat/src/history.rs`.
- **Aceitar quando:** Conversa importante pode ser exportada; logs internos sensíveis não viram conteúdo do Vault por acidente.
- **Controle:** Implementação; P0 na versão; depende de SL-S52-07; evidência em `reports/S52/SL-S52-08.md`.

### SL-S52-09 — Testar mudança de intenção/escopo

- **Executar:** Alterar meta e revogar acesso enquanto job está em execução.
- **Produzir:** `tests/intentions/scope/`.
- **Aceitar quando:** Resultado antigo é rejeitado/reclassificado e não publica sugestão desautorizada.
- **Controle:** Teste; P0 na versão; depende de SL-S52-08; evidência em `reports/S52/SL-S52-09.md`.

### SL-S52-10 — Avaliar utilidade e explicação

- **Executar:** Usar corpus de intenções com positivos/negativos e feedback do usuário.
- **Produzir:** `reports/intentions/evaluation.md`.
- **Aceitar quando:** Sugestões exibem motivo/evidências e podem ser rejeitadas sem reorganização forçada do acervo.
- **Controle:** Teste; P0 na versão; depende de SL-S52-09; evidência em `reports/S52/SL-S52-10.md`.

### SL-S52-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Material novo relevante produz sugestão explicável que só cria/altera board após confirmação.**

Registrar commit, ambiente, testes e pendências em `reports/S52/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s53"></a>

## S53 — Integrar o ciclo completo e congelar funcionalidades

- **Objetivo:** Consolidar os motores numa experiência única antes da fase de homologação.
- **Componente / forma:** Produto / alpha integrada / Integração.
- **Pré-requisitos técnicos:** S30 (Entregar Ingest sem fricção e leitura do acervo); S36 (Transcrever mídia e concluir enriquecimento do Ingest); S41 (Entregar busca incremental no acervo e nas Peças); S46 (Entregar recuperação estrutural e síntese progressiva); S50 (Construir widget de pesquisa e snapshots); S51 (Integrar copiloto editorial e BYOK); S52 (Implementar intenções, chat global e sugestões)
- **Entregável do sprint:** v0.9.0 alpha do ciclo analítico completo e inventário de lacunas.
- **Demonstração exigida:** Capturar fonte, pesquisar, conectar, produzir Peça e comprovar origem e reconstrução.
- **Requisitos:** R02, R07, R11, R13, R14, R26, R28, R34. **Risco de integração:** Alto.

### SL-S53-01 — Unificar IDs e navegação

- **Executar:** Resolver links entre Ingest, widget, célula, Peça e histórico por contrato comum.
- **Produzir:** `crates/app-core/src/navigation.rs`.
- **Aceitar quando:** Links não dependem de nomes de tela ou paths absolutos frágeis.
- **Controle:** Implementação; P1 na versão; depende de SL-S30-GATE, SL-S36-GATE, SL-S41-GATE, SL-S46-GATE, SL-S50-GATE, SL-S51-GATE, SL-S52-GATE; evidência em `reports/S53/SL-S53-01.md`.

### SL-S53-02 — Harmonizar estados de operação

- **Executar:** Padronizar fila, erro, offline, revisão antiga e solicitação de permissão.
- **Produzir:** `apps/desktop/src/status-system/`.
- **Aceitar quando:** Mesmo estado não recebe significados contraditórios em fases diferentes.
- **Controle:** Implementação; P1 na versão; depende de SL-S53-01; evidência em `reports/S53/SL-S53-02.md`.

### SL-S53-03 — Revisar fluxos de promoção

- **Executar:** Tornar clara a diferença entre referência, recorte, cópia e fork-on-insert.
- **Produzir:** `apps/desktop/src/flows/promote/`.
- **Aceitar quando:** Usuário consegue prever quais conteúdos serão independentes depois de cada ação.
- **Controle:** Implementação; P1 na versão; depende de SL-S53-02; evidência em `reports/S53/SL-S53-03.md`.

### SL-S53-04 — Unificar painel de recursos

- **Executar:** Mostrar runtime/modelos/jobs e permitir pausar tarefas de fundo.
- **Produzir:** `apps/desktop/src/settings/resources/`.
- **Aceitar quando:** Diagnóstico inclui workers e browser, não apenas contador do frontend.
- **Controle:** Implementação; P1 na versão; depende de SL-S53-03; evidência em `reports/S53/SL-S53-04.md`.

### SL-S53-05 — Rodar jornada integral local

- **Executar:** Usar pacote de dados/modelos provisionado e rede bloqueada.
- **Produzir:** `reports/v0.9/local-journey.md`.
- **Aceitar quando:** Autoria, busca e contexto locais funcionam; web/Jev/BYOK mostram indisponibilidade sem fallback oculto.
- **Controle:** Teste; P1 na versão; depende de SL-S53-04; evidência em `reports/S53/SL-S53-05.md`.

### SL-S53-06 — Rodar jornada conectada consentida

- **Executar:** Habilitar pesquisa/Jev/BYOK separadamente e registrar egress autorizado.
- **Produzir:** `reports/v0.9/connected-journey.md`.
- **Aceitar quando:** Ativar um provedor não autoriza todos os demais automaticamente.
- **Controle:** Teste; P1 na versão; depende de SL-S53-05; evidência em `reports/S53/SL-S53-06.md`.

### SL-S53-07 — Apagar caches durante ensaio seguro

- **Executar:** Reabrir, reconstruir e comparar autoria/proveniência.
- **Produzir:** `reports/v0.9/reconstruction.md`.
- **Aceitar quando:** Nenhum motor contém a única cópia de conhecimento aceito.
- **Controle:** Teste; P1 na versão; depende de SL-S53-06; evidência em `reports/S53/SL-S53-07.md`.

### SL-S53-08 — Congelar funcionalidades v1

- **Executar:** Inventariar requisitos ainda pendentes de homologação/portabilidade.
- **Produzir:** `docs/release/v1-freeze.md`.
- **Aceitar quando:** Lacunas não são renomeadas como feature pronta; escopo novo vai para proposta pós-v1.
- **Controle:** Governança; P1 na versão; depende de SL-S53-07; evidência em `reports/S53/SL-S53-08.md`.

### SL-S53-09 — Triar defeitos

- **Executar:** Classificar integridade/segurança, função essencial, recursos e polimento.
- **Produzir:** `reports/v0.9/defect-register.csv`.
- **Aceitar quando:** Defeito crítico tem owner e bloqueio; não é adiado a v2 para concluir v1 artificialmente.
- **Controle:** Governança; P1 na versão; depende de SL-S53-08; evidência em `reports/S53/SL-S53-09.md`.

### SL-S53-10 — Publicar alpha integrada

- **Executar:** Distribuir somente capacidades testadas e pedir feedback com diagnóstico saneado.
- **Produzir:** `release/v0.9.0/`.
- **Aceitar quando:** Alpha não é anunciada como release estável ou como prova de todas as metas do RFC.
- **Controle:** Release; P1 na versão; depende de SL-S53-09; evidência em `reports/S53/SL-S53-10.md`.

### SL-S53-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Capturar fonte, pesquisar, conectar, produzir Peça e comprovar origem e reconstrução.**

Registrar commit, ambiente, testes e pendências em `reports/S53/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

# v1.0.0-beta.1 — Homologação multiplataforma

**Maturidade:** Beta  
**Entrega acumulada:** Segurança, performance e pacotes validados nos cinco alvos, sem presumir paridade de aceleração.

<a id="s54"></a>

## S54 — Executar hardening e auditoria adversarial

- **Objetivo:** Testar composição real, não apenas segurança isolada de cada adapter.
- **Componente / forma:** Segurança / todos os motores / Próprio.
- **Pré-requisitos técnicos:** S53 (Integrar o ciclo completo e congelar funcionalidades)
- **Entregável do sprint:** Relatório de segurança com correções bloqueadoras concluídas.
- **Demonstração exigida:** Ataques de arquivo, web, prompt e processo falham sem afetar outros workspaces.
- **Requisitos:** R05, R06, R08, R30, R34. **Risco de integração:** Crítico.

### SL-S54-01 — Revalidar modelo de ameaças

- **Executar:** Atualizar superfícies após todas as integrações e revisar privilégios efetivos.
- **Produzir:** `docs/security/threat-model-v1.md`.
- **Aceitar quando:** Nenhum runtime/endpoint novo fica fora do inventário de fronteiras.
- **Controle:** Teste; P0 na versão; depende de SL-S53-GATE; evidência em `reports/S54/SL-S54-01.md`.

### SL-S54-02 — Fuzzar codecs/protocolos

- **Executar:** Cobrir Markdown, YAML, JSON, MPK, respostas de workers e patches editoriais.
- **Produzir:** `tests/fuzz/`.
- **Aceitar quando:** Crashes e consumo excessivo reproduzíveis viram regressões, não arquivos descartados sem análise.
- **Controle:** Teste; P0 na versão; depende de SL-S54-01; evidência em `reports/S54/SL-S54-02.md`.

### SL-S54-03 — Repetir ataques de filesystem

- **Executar:** Executar TOCTOU/junction/symlink nas operações reais dos motores.
- **Produzir:** `reports/security/fs-integration.json`.
- **Aceitar quando:** Proteção do Core não é contornada por parser/browser com acesso direto amplo.
- **Controle:** Teste; P0 na versão; depende de SL-S54-02; evidência em `reports/S54/SL-S54-03.md`.

### SL-S54-04 — Repetir ataques de rede

- **Executar:** Testar SSRF, serviços locais, redirects e subrequests de browser.
- **Produzir:** `reports/security/network-integration.json`.
- **Aceitar quando:** LocalAI, SearxNG e segredos não são alcançados por conteúdo adquirido sem autorização.
- **Controle:** Teste; P0 na versão; depende de SL-S54-03; evidência em `reports/S54/SL-S54-04.md`.

### SL-S54-05 — Red-team de prompt injection

- **Executar:** Misturar instruções hostis com documentos/citações e propostas de geração.
- **Produzir:** `reports/security/prompt-injection.json`.
- **Aceitar quando:** Ataque pode afetar texto gerado, mas não altera permissões nem executa ações proibidas.
- **Controle:** Teste; P0 na versão; depende de SL-S54-04; evidência em `reports/S54/SL-S54-05.md`.

### SL-S54-06 — Auditar credenciais/logs

- **Executar:** Procurar chaves, conteúdos privados e metadados sensíveis em exports e diagnóstico.
- **Produzir:** `reports/security/secrets.json`.
- **Aceitar quando:** Dados-semente não vazam; logging detalhado exige opt-in e saneamento.
- **Controle:** Teste; P0 na versão; depende de SL-S54-05; evidência em `reports/S54/SL-S54-06.md`.

### SL-S54-07 — Corrigir vulnerabilidades encontradas

- **Executar:** Priorizar causa raiz e adicionar teste que falha antes do patch.
- **Produzir:** `security/fixes/`.
- **Aceitar quando:** Achado crítico não fica apenas documentado como risco aceito para publicar estável.
- **Controle:** Implementação; P0 na versão; depende de SL-S54-06; evidência em `reports/S54/SL-S54-07.md`.

### SL-S54-08 — Auditar dependências e patches

- **Executar:** Conferir avisos, CVEs relevantes e necessidade de atualização controlada.
- **Produzir:** `compliance/security-review-v1.json`.
- **Aceitar quando:** Atualização passa regressões; ausência de CVE não é tratada como garantia de segurança.
- **Controle:** Upstream; P0 na versão; depende de SL-S54-07; evidência em `reports/S54/SL-S54-08.md`.

### SL-S54-09 — Validar fail-closed

- **Executar:** Remover mecanismo obrigatório de sandbox e tentar iniciar recurso dependente.
- **Produzir:** `tests/security/missing-capability/`.
- **Aceitar quando:** Função é bloqueada claramente em vez de rodar sem contenção.
- **Controle:** Teste; P0 na versão; depende de SL-S54-08; evidência em `reports/S54/SL-S54-09.md`.

### SL-S54-10 — Fechar gate de segurança

- **Executar:** Revisar evidências manualmente e registrar limites residuais não críticos.
- **Produzir:** `reports/security/v1-signoff.md`.
- **Aceitar quando:** Aprovação identifica exatamente versões/perfis testados e não afirma segurança absoluta.
- **Controle:** Governança; P0 na versão; depende de SL-S54-09; evidência em `reports/S54/SL-S54-10.md`.

### SL-S54-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Ataques de arquivo, web, prompt e processo falham sem afetar outros workspaces.**

Registrar commit, ambiente, testes e pendências em `reports/S54/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s55"></a>

## S55 — Homologar performance e degradação por hardware

- **Objetivo:** Medir os budgets do RFC no aplicativo completo e corrigir gargalos com evidência.
- **Componente / forma:** ResourceSupervisor / benchmarks / Próprio.
- **Pré-requisitos técnicos:** S54 (Executar hardening e auditoria adversarial)
- **Entregável do sprint:** Perfis de desempenho homologados e política de degradação.
- **Demonstração exigida:** Repetir benchmarks com dados brutos e observar retorno real a idle após tarefas pesadas.
- **Requisitos:** R09, R12, R20, R23, R34. **Risco de integração:** Alto.

### SL-S55-01 — Fixar ambiente de homologação

- **Executar:** Registrar hardware, SO, drivers, armazenamento, dataset e versões dos modelos.
- **Produzir:** `bench/profiles/v1.yaml`.
- **Aceitar quando:** Resultado de VM/CI não é confundido com perfil de desktop real.
- **Controle:** Teste; P1 na versão; depende de SL-S54-GATE; evidência em `reports/S55/SL-S55-01.md`.

### SL-S55-02 — Medir idle agregado

- **Executar:** Contar Core, WebView, GPU/auxiliares pertinentes e serviços sob gestão.
- **Produzir:** `reports/perf/idle-v1.json`.
- **Aceitar quando:** Meta de 350 MB é avaliada no perfil aprovado, sem omitir processos pesados.
- **Controle:** Teste; P1 na versão; depende de SL-S55-01; evidência em `reports/S55/SL-S55-02.md`.

### SL-S55-03 — Medir startup e ack

- **Executar:** Distinguir primeiro uso, caches quentes/frios e aceitação versus conclusão de comando.
- **Produzir:** `reports/perf/startup-ipc-v1.json`.
- **Aceitar quando:** Meta de 2,2 s/150 ms possui definição e dados por percentil.
- **Controle:** Teste; P1 na versão; depende de SL-S55-02; evidência em `reports/S55/SL-S55-03.md`.

### SL-S55-04 — Medir indexação leve

- **Executar:** Validar 650 MB no perfil definido e registrar OCR/STT separadamente.
- **Produzir:** `reports/perf/indexing-v1.json`.
- **Aceitar quando:** Pipelines pesados não recebem exceção implícita ao requisito.
- **Controle:** Teste; P1 na versão; depende de SL-S55-03; evidência em `reports/S55/SL-S55-04.md`.

### SL-S55-05 — Medir inferência e unload

- **Executar:** Avaliar 2,8 GB, contexto, simultaneidade e liberação após ociosidade.
- **Produzir:** `reports/perf/inference-v1.json`.
- **Aceitar quando:** Pesos, KV/cache e runtime são contabilizados; não há claim baseado só no tamanho GGUF.
- **Controle:** Teste; P1 na versão; depende de SL-S55-04; evidência em `reports/S55/SL-S55-05.md`.

### SL-S55-06 — Medir cena complexa

- **Executar:** Coletar frame time em 100/1.000 nós com textos/imagens/arestas.
- **Produzir:** `reports/perf/canvas-v1.json`.
- **Aceitar quando:** LOD não ganha desempenho apagando dados ou desativando interação sem aviso.
- **Controle:** Teste; P1 na versão; depende de SL-S55-05; evidência em `reports/S55/SL-S55-06.md`.

### SL-S55-07 — Corrigir gargalos demonstrados

- **Executar:** Atacar hotspots de startup, renderer, indexador e serialização identificados no profiler.
- **Produzir:** `perf/fixes-v1/`.
- **Aceitar quando:** Otimização mantém suítes funcionais e mostra comparação antes/depois.
- **Controle:** Implementação; P1 na versão; depende de SL-S55-06; evidência em `reports/S55/SL-S55-07.md`.

### SL-S55-08 — Aplicar backpressure

- **Executar:** Pausar/reduzir jobs quando recursos se aproximam do perfil autorizado.
- **Produzir:** `crates/resources/src/backpressure.rs`.
- **Aceitar quando:** UI continua utilizável e cancelamento tem efeito; não há thrashing contínuo de modelos.
- **Controle:** Implementação; P1 na versão; depende de SL-S55-07; evidência em `reports/S55/SL-S55-08.md`.

### SL-S55-09 — Explicar degradação

- **Executar:** Informar capacidade indisponível, pacote menor ou tarefa pendente sem fallback externo.
- **Produzir:** `apps/desktop/src/resources/degradation.ts`.
- **Aceitar quando:** Hardware insuficiente não dispara nuvem nem altera conteúdo por truncamento oculto.
- **Controle:** Implementação; P1 na versão; depende de SL-S55-08; evidência em `reports/S55/SL-S55-09.md`.

### SL-S55-10 — Ratificar perfil final

- **Executar:** Corrigir até cumprir ou reabrir ADR-007 explicitamente antes do release.
- **Produzir:** `reports/perf/v1-signoff.md`.
- **Aceitar quando:** Meta não atingida não recebe selo verde; desvios não são escondidos no changelog.
- **Controle:** Decisão; P0 na versão; depende de SL-S55-09; evidência em `reports/S55/SL-S55-10.md`.

### SL-S55-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Repetir benchmarks com dados brutos e observar retorno real a idle após tarefas pesadas.**

Registrar commit, ambiente, testes e pendências em `reports/S55/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s56"></a>

## S56 — Empacotar e validar Linux e Windows

- **Objetivo:** Produzir pacotes instaláveis sem ambientes de desenvolvimento globais.
- **Componente / forma:** Distribuição / Linux / Windows / Integração.
- **Pré-requisitos técnicos:** S54 (Executar hardening e auditoria adversarial); S55 (Homologar performance e degradação por hardware)
- **Entregável do sprint:** Pacotes Linux x86_64/aarch64 e Windows x86_64 com capacidades declaradas.
- **Demonstração exigida:** Instalar em máquina limpa, provisionar perfil permitido, usar e remover o app.
- **Requisitos:** R01, R06, R09, R30, R33. **Risco de integração:** Crítico.

### SL-S56-01 — Definir layout de pacotes

- **Executar:** Separar app, runtimes, modelos e pacotes opcionais por arquitetura.
- **Produzir:** `packaging/layout-v1.json`.
- **Aceitar quando:** Pacote não depende de arquivos presentes apenas na máquina de build.
- **Controle:** Implementação; P0 na versão; depende de SL-S54-GATE, SL-S55-GATE; evidência em `reports/S56/SL-S56-01.md`.

### SL-S56-02 — Construir Linux x86_64

- **Executar:** Empacotar Core/UI e workers homologados com dependências nativas.
- **Produzir:** `packaging/linux/x86_64/`.
- **Aceitar quando:** Instalação limpa roda autoria e capacidades declaradas sem toolchains globais.
- **Controle:** Implementação; P0 na versão; depende de SL-S56-01; evidência em `reports/S56/SL-S56-02.md`.

### SL-S56-03 — Construir Linux aarch64

- **Executar:** Ajustar bibliotecas/backends e não reutilizar binários x86 por engano.
- **Produzir:** `packaging/linux/aarch64/`.
- **Aceitar quando:** Smoke nativo e verificação de arquitetura passam nos componentes distribuídos.
- **Controle:** Implementação; P0 na versão; depende de SL-S56-02; evidência em `reports/S56/SL-S56-03.md`.

### SL-S56-04 — Construir Windows x86_64

- **Executar:** Empacotar dependências, helpers e WebView conforme política aprovada.
- **Produzir:** `packaging/windows/x86_64/`.
- **Aceitar quando:** Não exige privilégio ou runtime inesperado sem documentação/instalação consentida.
- **Controle:** Implementação; P0 na versão; depende de SL-S56-03; evidência em `reports/S56/SL-S56-04.md`.

### SL-S56-05 — Validar isolamento instalado

- **Executar:** Repetir testes essenciais de sandbox no pacote, não só no ambiente dev.
- **Produzir:** `reports/packaging/linux-windows-security.json`.
- **Aceitar quando:** Paths finais, ACLs e helpers preservam as garantias aprovadas.
- **Controle:** Teste; P0 na versão; depende de SL-S56-04; evidência em `reports/S56/SL-S56-05.md`.

### SL-S56-06 — Validar instalação/atualização/remoção

- **Executar:** Testar paths Unicode, usuário sem admin e reinstalação.
- **Produzir:** `tests/packaging/linux-windows/`.
- **Aceitar quando:** Remoção não apaga Vaults pessoais sem confirmação explícita.
- **Controle:** Teste; P0 na versão; depende de SL-S56-05; evidência em `reports/S56/SL-S56-06.md`.

### SL-S56-07 — Validar pacotes opcionais offline

- **Executar:** Instalar por arquivos/manifestos locais com rede bloqueada.
- **Produzir:** `reports/packaging/offline-linux-windows.json`.
- **Aceitar quando:** Capacidades funcionam sem download tardio oculto.
- **Controle:** Teste; P0 na versão; depende de SL-S56-06; evidência em `reports/S56/SL-S56-07.md`.

### SL-S56-08 — Gerar SBOM e avisos

- **Executar:** Inventariar binários, Python privado, browser, DLLs e modelos incluídos.
- **Produzir:** `compliance/packages/linux-windows/`.
- **Aceitar quando:** Licenças/fontes correspondentes acompanham os artefatos efetivamente distribuídos.
- **Controle:** Licença; P0 na versão; depende de SL-S56-07; evidência em `reports/S56/SL-S56-08.md`.

### SL-S56-09 — Rodar jornadas nativas

- **Executar:** Repetir Captura→Mesa→Peça, busca/contexto e web nos perfis habilitados.
- **Produzir:** `reports/targets/linux-windows-v1.json`.
- **Aceitar quando:** Build verde não substitui execução real nos três alvos.
- **Controle:** Teste; P0 na versão; depende de SL-S56-08; evidência em `reports/S56/SL-S56-09.md`.

### SL-S56-10 — Produzir artefatos beta

- **Executar:** Assinar/verificar conforme política e registrar hashes de cada pacote.
- **Produzir:** `release/v1.0.0-beta.1/linux-windows/`.
- **Aceitar quando:** Usuário consegue verificar origem; limitações de assinatura são explícitas, não bypass silencioso.
- **Controle:** Release; P0 na versão; depende de SL-S56-09; evidência em `reports/S56/SL-S56-10.md`.

### SL-S56-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Instalar em máquina limpa, provisionar perfil permitido, usar e remover o app.**

Registrar commit, ambiente, testes e pendências em `reports/S56/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s57"></a>

## S57 — Empacotar macOS e fechar beta multiplataforma

- **Objetivo:** Validar Apple Silicon/Intel e consolidar a matriz real de capacidades.
- **Componente / forma:** Distribuição / macOS / Integração.
- **Pré-requisitos técnicos:** S54 (Executar hardening e auditoria adversarial); S55 (Homologar performance e degradação por hardware); S56 (Empacotar e validar Linux e Windows)
- **Entregável do sprint:** v1.0.0-beta.1 para os cinco alvos previstos.
- **Demonstração exigida:** Instalação limpa em macOS preserva helpers, permissões, modelos e dados do usuário.
- **Requisitos:** R01, R06, R09, R30, R33. **Risco de integração:** Crítico.

### SL-S57-01 — Construir Apple Silicon

- **Executar:** Empacotar binários/model backends compatíveis e paths de recursos.
- **Produzir:** `packaging/macos/arm64/`.
- **Aceitar quando:** Nenhum backend x86 é carregado implicitamente para mascarar falta de suporte.
- **Controle:** Implementação; P0 na versão; depende de SL-S54-GATE, SL-S55-GATE, SL-S56-GATE; evidência em `reports/S57/SL-S57-01.md`.

### SL-S57-02 — Construir macOS Intel

- **Executar:** Definir modelos/backends CPU/GPU realmente disponíveis.
- **Produzir:** `packaging/macos/x86_64/`.
- **Aceitar quando:** Aceleração indisponível não é prometida por paridade com Apple Silicon.
- **Controle:** Implementação; P0 na versão; depende de SL-S57-01; evidência em `reports/S57/SL-S57-02.md`.

### SL-S57-03 — Configurar helpers/assinatura

- **Executar:** Aplicar entitlements e política de distribuição aprovada a todos os executáveis.
- **Produzir:** `packaging/macos/signing/`.
- **Aceitar quando:** Helper não perde contenção ao ser movido para o bundle final.
- **Controle:** Implementação; P0 na versão; depende de SL-S57-02; evidência em `reports/S57/SL-S57-03.md`.

### SL-S57-04 — Validar instalação limpa

- **Executar:** Executar sem ambientes Python/Go/Rust de desenvolvimento.
- **Produzir:** `tests/packaging/macos-install/`.
- **Aceitar quando:** Ausência de dependência é detectada antes do uso, sem instrução opaca para desativar segurança.
- **Controle:** Teste; P0 na versão; depende de SL-S57-03; evidência em `reports/S57/SL-S57-04.md`.

### SL-S57-05 — Repetir sandbox instalado

- **Executar:** Tentar acesso a dados privados, rede e filhos não autorizados.
- **Produzir:** `reports/packaging/macos-security.json`.
- **Aceitar quando:** Permissões reais do bundle correspondem aos perfis aprovados.
- **Controle:** Teste; P0 na versão; depende de SL-S57-04; evidência em `reports/S57/SL-S57-05.md`.

### SL-S57-06 — Validar modelos/pacotes offline

- **Executar:** Provisionar pelo fluxo documentado e bloquear rede.
- **Produzir:** `reports/packaging/macos-offline.json`.
- **Aceitar quando:** Geração/embeddings homologados funcionam sem chamada externa.
- **Controle:** Teste; P0 na versão; depende de SL-S57-05; evidência em `reports/S57/SL-S57-06.md`.

### SL-S57-07 — Rodar jornadas por arquitetura

- **Executar:** Exercitar editor, Mesa, ingest, busca, contexto e web habilitada.
- **Produzir:** `reports/targets/macos-v1.json`.
- **Aceitar quando:** Teste de uma arquitetura não vale como aprovação da outra.
- **Controle:** Teste; P0 na versão; depende de SL-S57-06; evidência em `reports/S57/SL-S57-07.md`.

### SL-S57-08 — Consolidar matriz dos cinco alvos

- **Executar:** Comparar capacidades essenciais e opcionais com evidências.
- **Produzir:** `reports/targets/v1-capabilities.json`.
- **Aceitar quando:** Qualquer lacuna essencial bloqueia stable até correção ou replanejamento formal de escopo.
- **Controle:** Teste; P0 na versão; depende de SL-S57-07; evidência em `reports/S57/SL-S57-08.md`.

### SL-S57-09 — Publicar guia beta

- **Executar:** Explicar instalação, requisitos, limites e coleta de diagnóstico local saneado.
- **Produzir:** `docs/releases/v1-beta.md`.
- **Aceitar quando:** Usuário identifica facilmente qual pacote/modelo pode usar em seu hardware.
- **Controle:** Documentação; P0 na versão; depende de SL-S57-08; evidência em `reports/S57/SL-S57-09.md`.

### SL-S57-10 — Publicar beta multiplataforma

- **Executar:** Empacotar notas, SBOM, hashes e fontes aplicáveis.
- **Produzir:** `release/v1.0.0-beta.1/`.
- **Aceitar quando:** Beta mantém estado experimental explícito e não oculta falhas conhecidas.
- **Controle:** Release; P0 na versão; depende de SL-S57-09; evidência em `reports/S57/SL-S57-10.md`.

### SL-S57-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Instalação limpa em macOS preserva helpers, permissões, modelos e dados do usuário.**

Registrar commit, ambiente, testes e pendências em `reports/S57/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

# v1.0.0-rc.1 — Candidata à estabilidade

**Maturidade:** RC  
**Entrega acumulada:** Migrações, backup/Git, documentação, acessibilidade e instalação limpa avaliados.

<a id="s58"></a>

## S58 — Implementar migrações, backup e Git opcional

- **Objetivo:** Tornar evolução e cópia dos dados seguras antes do primeiro stable.
- **Componente / forma:** Compatibilidade / portabilidade / Próprio.
- **Pré-requisitos técnicos:** S22 (Entregar produção local, histórico e exportação); S57 (Empacotar macOS e fechar beta multiplataforma)
- **Entregável do sprint:** Migração transacional, backup restaurável e integração Git consentida.
- **Demonstração exigida:** Migrar cópia de Vault antigo, restaurar backup e fazer commit/push apenas por ação autorizada.
- **Requisitos:** R02, R05, R29, R32, R37. **Risco de integração:** Crítico.

### SL-S58-01 — Ratificar retenção/Git/auditoria

- **Executar:** Resolver ADR-016 incluindo arquivos canônicos, caches excluídos e limites de anchoring.
- **Produzir:** `docs/adr/016-history-portability.md`.
- **Aceitar quando:** Git opcional não é chamado de backup remoto sem configuração real.
- **Controle:** Decisão; P0 na versão; depende de SL-S22-GATE, SL-S57-GATE; evidência em `reports/S58/SL-S58-01.md`.

### SL-S58-02 — Criar planejador de migração

- **Executar:** Detectar versão e mostrar plano antes de executar mudanças.
- **Produzir:** `crates/migrations/src/plan.rs`.
- **Aceitar quando:** Versão desconhecida abre read-only ou é recusada sem regravação destrutiva.
- **Controle:** Implementação; P0 na versão; depende de SL-S58-01; evidência em `reports/S58/SL-S58-02.md`.

### SL-S58-03 — Executar migração recuperável

- **Executar:** Criar ponto de restauração e publicar migração com protocolo durável.
- **Produzir:** `crates/migrations/src/run.rs`.
- **Aceitar quando:** Interrupção permite retomar/voltar; formatos intermediários não são anunciados como prontos.
- **Controle:** Implementação; P0 na versão; depende de SL-S58-02; evidência em `reports/S58/SL-S58-03.md`.

### SL-S58-04 — Versionar caches separadamente

- **Executar:** Reconstruir índices/MPK sem obrigar migração autoral.
- **Produzir:** `crates/migrations/src/derived.rs`.
- **Aceitar quando:** Atualização de engine vetorial não muda o Markdown sem necessidade.
- **Controle:** Implementação; P0 na versão; depende de SL-S58-03; evidência em `reports/S58/SL-S58-04.md`.

### SL-S58-05 — Concluir backup/restauração

- **Executar:** Validar escopo, manifest, integridade e dry-run do restauro.
- **Produzir:** `crates/backup/`.
- **Aceitar quando:** Backup de dados confirmados reconstrói autoria e histórico aprovado numa raiz limpa.
- **Controle:** Implementação; P0 na versão; depende de SL-S58-04; evidência em `reports/S58/SL-S58-05.md`.

### SL-S58-06 — Integrar Git local

- **Executar:** Commitar arquivos autorizados com hooks/filters e execução de comandos sob política segura.
- **Produzir:** `adapters/git/local.rs`.
- **Aceitar quando:** Repositório importado não executa hook/filtro arbitrário por simples abertura.
- **Controle:** Implementação; P0 na versão; depende de SL-S58-05; evidência em `reports/S58/SL-S58-06.md`.

### SL-S58-07 — Integrar remotos opt-in

- **Executar:** Configurar credenciais do SO, preview de conteúdo e confirmação de push/pull.
- **Produzir:** `adapters/git/remote.rs`.
- **Aceitar quando:** Nenhum remoto é criado ou sincronizado automaticamente; conflitos são explícitos.
- **Controle:** Implementação; P0 na versão; depende de SL-S58-06; evidência em `reports/S58/SL-S58-07.md`.

### SL-S58-08 — Definir retenção e purga

- **Executar:** Separar limpar cache de apagar histórico/assets não referenciados.
- **Produzir:** `crates/history/src/retention.rs`.
- **Aceitar quando:** Purga com efeito autoral exige confirmação e explica perda de rollback/auditoria.
- **Controle:** Implementação; P0 na versão; depende de SL-S58-07; evidência em `reports/S58/SL-S58-08.md`.

### SL-S58-09 — Testar matriz de versões

- **Executar:** Migrar fixtures de versões anteriores e simular downgrade incompatível.
- **Produzir:** `tests/migrations/v1/`.
- **Aceitar quando:** Dados não são silenciosamente perdidos; impossibilidade de downgrade tem alternativa de export/restore.
- **Controle:** Teste; P0 na versão; depende de SL-S58-08; evidência em `reports/S58/SL-S58-09.md`.

### SL-S58-10 — Restaurar em outro sistema

- **Executar:** Levar backup entre SOs e verificar IDs, fontes, paths e referências.
- **Produzir:** `reports/portability/v1-restore.md`.
- **Aceitar quando:** Portabilidade não depende de índices ou diretórios específicos da máquina de origem.
- **Controle:** Teste; P0 na versão; depende de SL-S58-09; evidência em `reports/S58/SL-S58-10.md`.

### SL-S58-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Migrar cópia de Vault antigo, restaurar backup e fazer commit/push apenas por ação autorizada.**

Registrar commit, ambiente, testes e pendências em `reports/S58/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s59"></a>

## S59 — Concluir acessibilidade, documentação e piloto

- **Objetivo:** Validar que o produto pode ser utilizado e mantido sem conhecimento de seus motores internos.
- **Componente / forma:** Produto / qualidade de uso / Próprio.
- **Pré-requisitos técnicos:** S57 (Empacotar macOS e fechar beta multiplataforma); S58 (Implementar migrações, backup e Git opcional)
- **Entregável do sprint:** v1.0.0-rc.1 com documentação e evidências de piloto.
- **Demonstração exigida:** Usuário novo instala, captura, produz Peça e recupera erro seguindo somente os guias.
- **Requisitos:** R01, R07, R08, R31, R33, R34. **Risco de integração:** Crítico.

### SL-S59-01 — Auditar teclado/foco

- **Executar:** Cobrir Ingest, canvas alternativo acessível, editor, diálogo e copiloto.
- **Produzir:** `reports/a11y/keyboard-v1.md`.
- **Aceitar quando:** Jornada essencial não contém armadilha de foco ou ação somente por gesto espacial.
- **Controle:** Teste; P0 na versão; depende de SL-S57-GATE, SL-S58-GATE; evidência em `reports/S59/SL-S59-01.md`.

### SL-S59-02 — Auditar semântica/contraste

- **Executar:** Verificar nomes acessíveis, status, escalas e alternativas aos sinais de cor.
- **Produzir:** `reports/a11y/interface-v1.md`.
- **Aceitar quando:** Erros e relações não dependem exclusivamente de cor ou imagem.
- **Controle:** Teste; P0 na versão; depende de SL-S59-01; evidência em `reports/S59/SL-S59-02.md`.

### SL-S59-03 — Corrigir bloqueios de uso

- **Executar:** Resolver defeitos críticos de navegação, estados vazios e mensagens de recuperação.
- **Produzir:** `ux/fixes-v1/`.
- **Aceitar quando:** Erro comum oferece ação concreta sem expor jargão interno obrigatório.
- **Controle:** Implementação; P0 na versão; depende de SL-S59-02; evidência em `reports/S59/SL-S59-03.md`.

### SL-S59-04 — Escrever guia de primeiros passos

- **Executar:** Explicar modos offline/conectado e ciclo Captura→Mesa→Peça.
- **Produzir:** `docs/user/getting-started.md`.
- **Aceitar quando:** Guia não exige familiaridade com PageIndex, LocalAI, SearxNG ou banco vetorial.
- **Controle:** Documentação; P0 na versão; depende de SL-S59-03; evidência em `reports/S59/SL-S59-04.md`.

### SL-S59-05 — Escrever guia de dados/recuperação

- **Executar:** Documentar formatos, backup, conflito, migração e reconstrução.
- **Produzir:** `docs/user/data-and-recovery.md`.
- **Aceitar quando:** Usuário distingue cache descartável de histórico autoral.
- **Controle:** Documentação; P0 na versão; depende de SL-S59-04; evidência em `reports/S59/SL-S59-05.md`.

### SL-S59-06 — Escrever guia de privacidade

- **Executar:** Mostrar envio Jev/BYOK/web, chaves, logs e revogação.
- **Produzir:** `docs/user/privacy.md`.
- **Aceitar quando:** Não promete que todos os dados ficam locais quando o usuário habilita serviços externos.
- **Controle:** Documentação; P0 na versão; depende de SL-S59-05; evidência em `reports/S59/SL-S59-06.md`.

### SL-S59-07 — Escrever guia de contribuição

- **Executar:** Orientar builds, fixtures, contratos, patches upstream e licenças.
- **Produzir:** `docs/contributing/`.
- **Aceitar quando:** Novo contribuinte consegue executar suíte básica sem credenciais pagas.
- **Controle:** Documentação; P0 na versão; depende de SL-S59-06; evidência em `reports/S59/SL-S59-07.md`.

### SL-S59-08 — Executar piloto com corpus permitido

- **Executar:** Observar jornadas reais ou testes estruturados do mantenedor, distinguindo as modalidades.
- **Produzir:** `reports/pilot/v1.md`.
- **Aceitar quando:** Feedback não é inventado; origem e limitações da amostra são registradas.
- **Controle:** Teste; P0 na versão; depende de SL-S59-07; evidência em `reports/S59/SL-S59-08.md`.

### SL-S59-09 — Revisar pendências da RC

- **Executar:** Classificar bugs, segurança, portabilidade e recursos contra critérios fixados.
- **Produzir:** `reports/releases/v1-rc-readiness.md`.
- **Aceitar quando:** Nenhum defeito crítico fica escondido como trabalho de versão futura.
- **Controle:** Governança; P0 na versão; depende de SL-S59-08; evidência em `reports/S59/SL-S59-09.md`.

### SL-S59-10 — Publicar candidata

- **Executar:** Anexar migrações, rollback, changelog, avisos e matriz de suporte.
- **Produzir:** `release/v1.0.0-rc.1/`.
- **Aceitar quando:** Fonte e binários correspondem à mesma revisão e evidências da RC.
- **Controle:** Release; P0 na versão; depende de SL-S59-09; evidência em `reports/S59/SL-S59-10.md`.

### SL-S59-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Usuário novo instala, captura, produz Peça e recupera erro seguindo somente os guias.**

Registrar commit, ambiente, testes e pendências em `reports/S59/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

# v1.0.0 — Escopo integral estável

**Maturidade:** Estável  
**Entrega acumulada:** Todos os requisitos do escopo aprovado, evidências e condições de distribuição satisfeitos.

<a id="s60"></a>

## S60 — Publicar Sandland v1.0.0

- **Objetivo:** Promover a stable somente o escopo aprovado e demonstrado.
- **Componente / forma:** Release / escopo integral / Próprio.
- **Pré-requisitos técnicos:** S54 (Executar hardening e auditoria adversarial); S55 (Homologar performance e degradação por hardware); S56 (Empacotar e validar Linux e Windows); S57 (Empacotar macOS e fechar beta multiplataforma); S58 (Implementar migrações, backup e Git opcional); S59 (Concluir acessibilidade, documentação e piloto)
- **Entregável do sprint:** v1.0.0 estável, conjunto de evidências e política de manutenção.
- **Demonstração exigida:** Instalar stable limpa, restaurar Vault de referência e concluir ciclo completo sem perda de dados.
- **Requisitos:** R01, R02, R05, R06, R07, R09, R33, R34. **Risco de integração:** Crítico.

### SL-S60-01 — Auditar rastreabilidade

- **Executar:** Conferir requisitos R01–R34/R37 aplicáveis contra testes e entregáveis reais.
- **Produzir:** `reports/releases/v1-traceability.md`.
- **Aceitar quando:** Cada requisito essencial tem evidência, e não apenas tarefa marcada concluída.
- **Controle:** Governança; P0 na versão; depende de SL-S54-GATE, SL-S55-GATE, SL-S56-GATE, SL-S57-GATE, SL-S58-GATE, SL-S59-GATE; evidência em `reports/S60/SL-S60-01.md`.

### SL-S60-02 — Rodar suíte final na revisão candidata

- **Executar:** Reexecutar contratos, e2e, segurança, migrações e recursos relevantes.
- **Produzir:** `reports/releases/v1-final-suite.json`.
- **Aceitar quando:** Resultados pertencem ao commit que será publicado, não a uma versão anterior parecida.
- **Controle:** Teste; P0 na versão; depende de SL-S60-01; evidência em `reports/S60/SL-S60-02.md`.

### SL-S60-03 — Repetir destruição de caches

- **Executar:** Executar prova oficial de soberania no pacote final.
- **Produzir:** `reports/releases/v1-file-truth.json`.
- **Aceitar quando:** Todos os dados autorais previstos reaparecem sem depender dos motores de índice.
- **Controle:** Teste; P0 na versão; depende de SL-S60-02; evidência em `reports/S60/SL-S60-03.md`.

### SL-S60-04 — Repetir modo desconectado

- **Executar:** Bloquear rede em instalação com pacotes provisionados.
- **Produzir:** `reports/releases/v1-offline.json`.
- **Aceitar quando:** Funções locais essenciais funcionam e não há egress inesperado.
- **Controle:** Teste; P0 na versão; depende de SL-S60-03; evidência em `reports/S60/SL-S60-04.md`.

### SL-S60-05 — Fechar conformidade

- **Executar:** Revisar SBOM, notices, fontes correspondentes e licenças dos modelos distribuídos.
- **Produzir:** `compliance/releases/v1.0.0/`.
- **Aceitar quando:** Natureza não comercial não é usada como isenção de obrigação ou custo de API.
- **Controle:** Licença; P0 na versão; depende de SL-S60-04; evidência em `reports/S60/SL-S60-05.md`.

### SL-S60-06 — Congelar e verificar artefatos

- **Executar:** Gerar hashes/assinaturas, registrar ambiente e associar fontes/lockfiles.
- **Produzir:** `release/v1.0.0/manifest.json`.
- **Aceitar quando:** Cada arquivo baixável corresponde a origem e versão auditáveis.
- **Controle:** Release; P0 na versão; depende de SL-S60-05; evidência em `reports/S60/SL-S60-06.md`.

### SL-S60-07 — Publicar notas de versão

- **Executar:** Explicar capacidades, mínimos, limitações, atualização e rollback.
- **Produzir:** `docs/releases/v1.0.0.md`.
- **Aceitar quando:** Não há claim de performance/segurança que exceda a matriz homologada.
- **Controle:** Documentação; P0 na versão; depende de SL-S60-06; evidência em `reports/S60/SL-S60-07.md`.

### SL-S60-08 — Definir suporte e vulnerabilidades

- **Executar:** Criar canal, política de triagem e processo de patch emergencial.
- **Produzir:** `SECURITY.md`.
- **Aceitar quando:** Relato crítico pode interromper roadmap e gerar patch sem aguardar próximo sprint de feature.
- **Controle:** Governança; P0 na versão; depende de SL-S60-07; evidência em `reports/S60/SL-S60-08.md`.

### SL-S60-09 — Fazer go/no-go humano

- **Executar:** Revisar blockers, demonstração e disponibilidade de artefatos.
- **Produzir:** `reports/releases/v1-go-no-go.md`.
- **Aceitar quando:** Autor assina decisão; agente não aprova automaticamente a própria implementação.
- **Controle:** Decisão; P0 na versão; depende de SL-S60-08; evidência em `reports/S60/SL-S60-09.md`.

### SL-S60-10 — Publicar stable e verificar distribuição

- **Executar:** Testar download/instalação a partir do canal real sem expor credenciais.
- **Produzir:** `reports/releases/v1-post-publish.md`.
- **Aceitar quando:** Se artefato estiver incorreto, canal é corrigido/revertido e problema não é ignorado.
- **Controle:** Release; P0 na versão; depende de SL-S60-09; evidência em `reports/S60/SL-S60-10.md`.

### SL-S60-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Instalar stable limpa, restaurar Vault de referência e concluir ciclo completo sem perda de dados.**

Registrar commit, ambiente, testes e pendências em `reports/S60/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

# v1.1.0 — Confiabilidade operacional

**Maturidade:** Estável — maturação  
**Entrega acumulada:** Correções do uso real, recuperação assistida e manutenção upstream sem perda de dados.

<a id="s61"></a>

## S61 — Estabilizar uso real e ratificar evolução

- **Objetivo:** Priorizar confiabilidade observada antes de ampliar escopo.
- **Componente / forma:** Manutenção / produto / Próprio.
- **Pré-requisitos técnicos:** S60 (Publicar Sandland v1.0.0)
- **Entregável do sprint:** Backlog de estabilização e ADR do pós-v1.0.
- **Demonstração exigida:** Reproduzir relato real/ensaio de uso e vincular correção a teste sem coletar dados privados.
- **Requisitos:** R05, R09, R31, R34, R36. **Risco de integração:** Crítico.

### SL-S61-01 — Coletar feedback consentido

- **Executar:** Usar issues e diagnóstico local saneado, sem telemetria compulsória.
- **Produzir:** `reports/post-v1/feedback-register.csv`.
- **Aceitar quando:** Relatos possuem consentimento/origem; ausência de usuários externos é declarada.
- **Controle:** Governança; P0 na versão; depende de SL-S60-GATE; evidência em `reports/S61/SL-S61-01.md`.

### SL-S61-02 — Triar incidentes

- **Executar:** Priorizar perda de dados, fuga de escopo, falhas essenciais e recursos.
- **Produzir:** `docs/maintenance/triage.md`.
- **Aceitar quando:** Hotfix crítico pode interromper qualquer sprint futuro.
- **Controle:** Governança; P0 na versão; depende de SL-S61-01; evidência em `reports/S61/SL-S61-02.md`.

### SL-S61-03 — Reproduzir problemas reais

- **Executar:** Transformar entradas permitidas em fixtures mínimas sem expor documentos do usuário.
- **Produzir:** `tests/regressions/post-v1/`.
- **Aceitar quando:** Cada bug corrigido tem caso reproduzível; bug não confirmado é identificado como hipótese.
- **Controle:** Teste; P0 na versão; depende de SL-S61-02; evidência em `reports/S61/SL-S61-03.md`.

### SL-S61-04 — Corrigir falhas prioritárias

- **Executar:** Implementar patches estreitos com revisão e rollback.
- **Produzir:** `maintenance/v1.1/fixes/`.
- **Aceitar quando:** Correção não introduz migração/feature escondida e passa contratos pertinentes.
- **Controle:** Implementação; P0 na versão; depende de SL-S61-03; evidência em `reports/S61/SL-S61-04.md`.

### SL-S61-05 — Executar uso prolongado

- **Executar:** Alternar Vaults, edição, OCR, pesquisa e inferência em sessão extensa.
- **Produzir:** `reports/post-v1/soak.json`.
- **Aceitar quando:** Crescimento de recursos, deadlocks e atrasos são medidos, não só observados visualmente.
- **Controle:** Teste; P0 na versão; depende de SL-S61-04; evidência em `reports/S61/SL-S61-05.md`.

### SL-S61-06 — Revisar dívida dos adapters

- **Executar:** Listar patches locais, APIs internas e riscos por engine.
- **Produzir:** `upstream/debt-register.md`.
- **Aceitar quando:** Cada divergência tem motivação e estratégia de reduzir/manter/upstreamar.
- **Controle:** Governança; P0 na versão; depende de SL-S61-05; evidência em `reports/S61/SL-S61-06.md`.

### SL-S61-07 — Ratificar propostas pós-v1

- **Executar:** Resolver ADR-017 para escala, offline/interoperabilidade e futuros provedores.
- **Produzir:** `docs/adr/017-post-v1-scope.md`.
- **Aceitar quando:** Novas funcionalidades permanecem propostas até aprovação humana; não bloqueiam correções essenciais.
- **Controle:** Decisão; P0 na versão; depende de SL-S61-06; evidência em `reports/S61/SL-S61-07.md`.

### SL-S61-08 — Definir alvos de escala

- **Executar:** Escolher corpus/hardware com base em uso observado antes de otimizar.
- **Produzir:** `bench/profiles/post-v1.yaml`.
- **Aceitar quando:** Novas metas não substituem silenciosamente budgets básicos do produto.
- **Controle:** Decisão; P0 na versão; depende de SL-S61-07; evidência em `reports/S61/SL-S61-08.md`.

### SL-S61-09 — Definir política de suporte

- **Executar:** Estabelecer ramos, patches de segurança, depreciação e compatibilidade de formatos.
- **Produzir:** `docs/maintenance/support-policy.md`.
- **Aceitar quando:** Não há promessa de suporte eterno ou calendário irreal para equipe solo.
- **Controle:** Governança; P0 na versão; depende de SL-S61-08; evidência em `reports/S61/SL-S61-09.md`.

### SL-S61-10 — Fechar regressão de estabilidade

- **Executar:** Comparar candidata com v1.0 e registrar o que efetivamente melhorou.
- **Produzir:** `reports/v1.1/stability-baseline.md`.
- **Aceitar quando:** Ausência de defeito conhecido não é preenchida por bug inventado para marcar tarefa concluída.
- **Controle:** Teste; P0 na versão; depende de SL-S61-09; evidência em `reports/S61/SL-S61-10.md`.

### SL-S61-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Reproduzir relato real/ensaio de uso e vincular correção a teste sem coletar dados privados.**

Registrar commit, ambiente, testes e pendências em `reports/S61/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s62"></a>

## S62 — Aprimorar recuperação assistida e diagnóstico

- **Objetivo:** Tornar falhas recuperáveis pelo usuário sem depender do mantenedor ou de nuvem.
- **Componente / forma:** Recovery / observabilidade local / Próprio.
- **Pré-requisitos técnicos:** S61 (Estabilizar uso real e ratificar evolução)
- **Entregável do sprint:** Assistente de diagnóstico/reparo com preview e reversão.
- **Demonstração exigida:** Diagnosticar índice corrompido e conflito, reparar somente o que foi autorizado.
- **Requisitos:** R02, R05, R08, R29, R31, R37. **Risco de integração:** Crítico.

### SL-S62-01 — Criar painel de saúde do Vault

- **Executar:** Reunir schemas, referências, histórico, caches e jobs.
- **Produzir:** `apps/desktop/src/diagnostics/vault-health.ts`.
- **Aceitar quando:** UI distingue problema autoral de índice descartável e evita alarmismo genérico.
- **Controle:** Implementação; P0 na versão; depende de SL-S61-GATE; evidência em `reports/S62/SL-S62-01.md`.

### SL-S62-02 — Adicionar preview de reparo

- **Executar:** Listar arquivos afetados, causa e resultado antes de modificar.
- **Produzir:** `crates/recovery/src/plan.rs`.
- **Aceitar quando:** Operação destrutiva não é escondida dentro de botão reconstruir índice.
- **Controle:** Implementação; P0 na versão; depende de SL-S62-01; evidência em `reports/S62/SL-S62-02.md`.

### SL-S62-03 — Criar checkpoint de reparo

- **Executar:** Preservar reversibilidade conforme política de retenção.
- **Produzir:** `crates/recovery/src/checkpoint.rs`.
- **Aceitar quando:** Reparo pode ser desfeito sem perder alterações confirmadas posteriores indevidamente.
- **Controle:** Implementação; P0 na versão; depende de SL-S62-02; evidência em `reports/S62/SL-S62-03.md`.

### SL-S62-04 — Melhorar reconciliação assistida

- **Executar:** Resolver órfãos e conflitos por decisão explícita.
- **Produzir:** `apps/desktop/src/diagnostics/reconcile.ts`.
- **Aceitar quando:** Sistema não escolhe automaticamente o arquivo mais novo como verdade universal.
- **Controle:** Implementação; P0 na versão; depende de SL-S62-03; evidência em `reports/S62/SL-S62-04.md`.

### SL-S62-05 — Criar export de diagnóstico saneado

- **Executar:** Remover corpo documental, caminhos sensíveis e segredos por padrão.
- **Produzir:** `crates/diagnostics/src/export.rs`.
- **Aceitar quando:** Fixtures-semente privadas não aparecem no pacote compartilhável.
- **Controle:** Implementação; P0 na versão; depende de SL-S62-04; evidência em `reports/S62/SL-S62-05.md`.

### SL-S62-06 — Explicar consumo por capacidade

- **Executar:** Atribuir recursos a modelo, browser, OCR, indexador e editor.
- **Produzir:** `apps/desktop/src/diagnostics/resources.ts`.
- **Aceitar quando:** Usuário pode pausar o causador real sem encerrar o app inteiro.
- **Controle:** Implementação; P0 na versão; depende de SL-S62-05; evidência em `reports/S62/SL-S62-06.md`.

### SL-S62-07 — Melhorar retomada de jobs

- **Executar:** Distinguir retry seguro, nova versão do input e tarefa manual necessária.
- **Produzir:** `crates/jobs/src/recovery.rs`.
- **Aceitar quando:** Job antigo não repete envio remoto após consentimento revogado.
- **Controle:** Implementação; P0 na versão; depende de SL-S62-06; evidência em `reports/S62/SL-S62-07.md`.

### SL-S62-08 — Ensaiar corrupção combinada

- **Executar:** Misturar cache inválido, arquivo ausente e journal incompleto em cópia descartável.
- **Produzir:** `tests/recovery/scenarios/`.
- **Aceitar quando:** Ferramenta conserva dados recuperáveis e relata o que não pode reconstruir.
- **Controle:** Teste; P0 na versão; depende de SL-S62-07; evidência em `reports/S62/SL-S62-08.md`.

### SL-S62-09 — Testar privacidade do suporte

- **Executar:** Tentar incluir segredos/metadados por caminhos de erro e dumps.
- **Produzir:** `reports/recovery/privacy.json`.
- **Aceitar quando:** Diagnóstico automático não envia dados; compartilhamento é opção explícita.
- **Controle:** Teste; P0 na versão; depende de SL-S62-08; evidência em `reports/S62/SL-S62-09.md`.

### SL-S62-10 — Escrever runbooks

- **Executar:** Guiar situações de disco cheio, modelo corrompido, índice perdido e conflito Git.
- **Produzir:** `docs/user/troubleshooting/`.
- **Aceitar quando:** Usuário pode seguir passos sem executar comandos desconhecidos como administrador.
- **Controle:** Documentação; P0 na versão; depende de SL-S62-09; evidência em `reports/S62/SL-S62-10.md`.

### SL-S62-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Diagnosticar índice corrompido e conflito, reparar somente o que foi autorizado.**

Registrar commit, ambiente, testes e pendências em `reports/S62/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s63"></a>

## S63 — Atualizar motores com segurança e publicar v1.1

- **Objetivo:** Provar que a composição pode ser mantida sem virar um fork monolítico congelado.
- **Componente / forma:** Upstreams / estabilidade / Integração.
- **Pré-requisitos técnicos:** S61 (Estabilizar uso real e ratificar evolução); S62 (Aprimorar recuperação assistida e diagnóstico)
- **Entregável do sprint:** v1.1.0 e primeiro ciclo completo de manutenção upstream.
- **Demonstração exigida:** Atualizar recorte necessário, comparar resultados e reverter se um contrato quebrar.
- **Requisitos:** R01, R30, R32, R33, R34. **Risco de integração:** Crítico.

### SL-S63-01 — Revisar mudanças upstream

- **Executar:** Examinar somente versões pertinentes e correções relevantes dos motores adotados.
- **Produzir:** `reports/upstream/v1.1-review.md`.
- **Aceitar quando:** Não se atualizam todos os componentes apenas para perseguir latest.
- **Controle:** Upstream; P0 na versão; depende de SL-S61-GATE, SL-S62-GATE; evidência em `reports/S63/SL-S63-01.md`.

### SL-S63-02 — Rebasear patches editoriais

- **Executar:** Aplicar série BlockSuite e verificar grafo de deps/licenças.
- **Produzir:** `upstream/blocksuite/patches/`.
- **Aceitar quando:** Round-trip, input e bundle continuam dentro do contrato.
- **Controle:** Upstream; P0 na versão; depende de SL-S63-01; evidência em `reports/S63/SL-S63-02.md`.

### SL-S63-03 — Rebasear recorte PageIndex

- **Executar:** Inspecionar ferramentas, prompts, model calls e identidade.
- **Produzir:** `upstream/pageindex/patches/`.
- **Aceitar quando:** Atualização não reintroduz cloud/store global ou renumeração de IDs autorais.
- **Controle:** Upstream; P0 na versão; depende de SL-S63-02; evidência em `reports/S63/SL-S63-03.md`.

### SL-S63-04 — Atualizar ingest/web quando necessário

- **Executar:** Verificar Docling, Scrapling, normalizador e engines SearxNG.
- **Produzir:** `upstream/maintenance-v1.1.json`.
- **Aceitar quando:** Mudanças de extração são avaliadas com corpus; fontes antigas não são reescritas automaticamente.
- **Controle:** Upstream; P0 na versão; depende de SL-S63-03; evidência em `reports/S63/SL-S63-04.md`.

### SL-S63-05 — Revisar LocalAI/modelos

- **Executar:** Separar atualização de runtime de mudança de pesos e backend.
- **Produzir:** `models/maintenance-v1.1.json`.
- **Aceitar quando:** Usuário não recebe troca silenciosa de modelo ou espaço de embeddings.
- **Controle:** Upstream; P0 na versão; depende de SL-S63-04; evidência em `reports/S63/SL-S63-05.md`.

### SL-S63-06 — Revisar backend de busca

- **Executar:** Testar índice/SDK escolhido e migração de cache em cópia descartável.
- **Produzir:** `reports/search/v1.1-upgrade.md`.
- **Aceitar quando:** Mudança do motor não exige modificar dados autorais para continuar funcionando.
- **Controle:** Upstream; P0 na versão; depende de SL-S63-05; evidência em `reports/S63/SL-S63-06.md`.

### SL-S63-07 — Executar comparação diferencial

- **Executar:** Rodar corpus antigo e novo comparando estrutura, busca e citações.
- **Produzir:** `reports/v1.1/differential.json`.
- **Aceitar quando:** Ganho em um corpus não oculta regressão de escopo, IDs ou fonte.
- **Controle:** Teste; P0 na versão; depende de SL-S63-06; evidência em `reports/S63/SL-S63-07.md`.

### SL-S63-08 — Atualizar SBOM/fontes

- **Executar:** Registrar novas versões, avisos, patches e artefatos distribuídos.
- **Produzir:** `compliance/releases/v1.1.0/`.
- **Aceitar quando:** Inventário corresponde ao pacote publicado, não só às dependências de desenvolvimento.
- **Controle:** Licença; P0 na versão; depende de SL-S63-07; evidência em `reports/S63/SL-S63-08.md`.

### SL-S63-09 — Contribuir correções genéricas

- **Executar:** Preparar propostas upstream quando adequado, sem depender da aceitação para preservar segurança.
- **Produzir:** `upstream/contributions-v1.1.md`.
- **Aceitar quando:** Mudança mantém rastreabilidade e termos; PR não enviado não é anunciado como contribuição aceita.
- **Controle:** Governança; P0 na versão; depende de SL-S63-08; evidência em `reports/S63/SL-S63-09.md`.

### SL-S63-10 — Publicar v1.1 e verificar rollback

- **Executar:** Incluir fixes, suporte, diagnóstico e restauração testada.
- **Produzir:** `release/v1.1.0/`.
- **Aceitar quando:** Usuário v1.0 atualiza sem perder dados e dispõe de procedimento de retorno compatível.
- **Controle:** Release; P0 na versão; depende de SL-S63-09; evidência em `reports/S63/SL-S63-10.md`.

### SL-S63-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Atualizar recorte necessário, comparar resultados e reverter se um contrato quebrar.**

Registrar commit, ambiente, testes e pendências em `reports/S63/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

# v1.2.0 — Escala e portabilidade

**Maturidade:** Estável — evolução proposta  
**Entrega acumulada:** Acervos maiores, pacotes offline e interoperabilidade ampliada, mediante ratificação de escopo.

<a id="s64"></a>

## S64 — Escalar busca, ingestão e contexto com evidência

- **Objetivo:** Aumentar capacidade onde as medições mostram necessidade, sem reinventar motores.
- **Componente / forma:** Escala / pipelines / Evolução proposta.
- **Pré-requisitos técnicos:** S61 (Estabilizar uso real e ratificar evolução); S63 (Atualizar motores com segurança e publicar v1.1)
- **Entregável do sprint:** Pipelines incrementais e budgets validados no corpus ampliado aprovado.
- **Demonstração exigida:** Processar lote maior mantendo autoria responsiva e resultados/revisões corretos.
- **Requisitos:** R09, R12, R14, R21, R22, R36. **Risco de integração:** Alto.

> Proposta de evolução: executar somente após ratificação ADR-017. Não adia correções de integridade/segurança pendentes.

### SL-S64-01 — Confirmar escopo de escala

- **Executar:** Ratificar metas/corpus de ADR-017 e preservar os budgets base.
- **Produzir:** `docs/product/scale-v1.2.md`.
- **Aceitar quando:** Novo alvo é definido antes do benchmark, não depois de ver resultados favoráveis.
- **Controle:** Decisão; P0 na versão; depende de SL-S61-GATE, SL-S63-GATE; evidência em `reports/S64/SL-S64-01.md`.

### SL-S64-02 — Identificar gargalos

- **Executar:** Perfilar parsing, snapshots, indexação, queries, summaries e cena separadamente.
- **Produzir:** `reports/scale/hotspots.json`.
- **Aceitar quando:** Otimização nasce de evidência e não de preferência por reescrever Python em Rust.
- **Controle:** Teste; P2 na versão; depende de SL-S64-01; evidência em `reports/S64/SL-S64-02.md`.

### SL-S64-03 — Otimizar filas de lote

- **Executar:** Introduzir prioridades/fairness e backpressure para UI versus tarefas em massa.
- **Produzir:** `crates/jobs/src/batch_scheduler.rs`.
- **Aceitar quando:** Lote grande não impede salvar edição ou cancelar tarefa prioritária.
- **Controle:** Implementação; P2 na versão; depende de SL-S64-02; evidência em `reports/S64/SL-S64-03.md`.

### SL-S64-04 — Otimizar parsing incremental

- **Executar:** Reaproveitar revisões/unidades quando backend permitir sem perder cobertura.
- **Produzir:** `crates/ingest/src/incremental_extract.rs`.
- **Aceitar quando:** Documento alterado não reutiliza texto de páginas incompatíveis apenas por posição.
- **Controle:** Implementação; P2 na versão; depende de SL-S64-03; evidência em `reports/S64/SL-S64-04.md`.

### SL-S64-05 — Otimizar indexação em geração

- **Executar:** Agrupar upserts e publicar revisão consistente com tombstones.
- **Produzir:** `crates/search/src/bulk_index.rs`.
- **Aceitar quando:** Pico de throughput não faz ressurgir material excluído.
- **Controle:** Implementação; P2 na versão; depende de SL-S64-04; evidência em `reports/S64/SL-S64-05.md`.

### SL-S64-06 — Otimizar contexto incremental

- **Executar:** Recalcular subárvores/grupos necessários e limitar entradas de cache.
- **Produzir:** `crates/context/src/incremental_large.rs`.
- **Aceitar quando:** Contexto permanece fiel às relações e não usa summaries obsoletos por economia.
- **Controle:** Implementação; P2 na versão; depende de SL-S64-05; evidência em `reports/S64/SL-S64-06.md`.

### SL-S64-07 — Otimizar viewport medida

- **Executar:** Corrigir gargalos de arestas/previews/texturas demonstrados.
- **Produzir:** `apps/desktop/src/board/perf/`.
- **Aceitar quando:** Melhor fluidez não remove acessibilidade ou precisão de hit testing.
- **Controle:** Implementação; P2 na versão; depende de SL-S64-06; evidência em `reports/S64/SL-S64-07.md`.

### SL-S64-08 — Validar cancelamento/retomada em escala

- **Executar:** Interromper import/index/contexto em diferentes pontos do lote.
- **Produzir:** `tests/scale/recovery/`.
- **Aceitar quando:** Retomar não duplica autoria nem exige iniciar todo o trabalho sem motivo documentado.
- **Controle:** Teste; P2 na versão; depende de SL-S64-07; evidência em `reports/S64/SL-S64-08.md`.

### SL-S64-09 — Comparar qualidade antes/depois

- **Executar:** Reexecutar avaliações reservadas de busca, summaries e fontes.
- **Produzir:** `reports/scale/quality-regression.json`.
- **Aceitar quando:** Otimização não reduz métricas essenciais abaixo dos limiares aprovados.
- **Controle:** Teste; P2 na versão; depende de SL-S64-08; evidência em `reports/S64/SL-S64-09.md`.

### SL-S64-10 — Publicar limites reais

- **Executar:** Descrever corpus/hardware e capacidades desabilitadas por orçamento.
- **Produzir:** `docs/performance/v1.2.md`.
- **Aceitar quando:** Nenhum número vira garantia genérica para qualquer desktop ou acervo.
- **Controle:** Documentação; P2 na versão; depende de SL-S64-09; evidência em `reports/S64/SL-S64-10.md`.

### SL-S64-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Processar lote maior mantendo autoria responsiva e resultados/revisões corretos.**

Registrar commit, ambiente, testes e pendências em `reports/S64/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s65"></a>

## S65 — Consolidar pacotes offline e perfis portáveis

- **Objetivo:** Facilitar provisionamento sem rede e sem dependências técnicas instaladas pelo usuário.
- **Componente / forma:** Distribuição offline / modelos / Evolução proposta.
- **Pré-requisitos técnicos:** S32 (Homologar modelos e construir ModelGateway); S57 (Empacotar macOS e fechar beta multiplataforma); S63 (Atualizar motores com segurança e publicar v1.1); S64 (Escalar busca, ingestão e contexto com evidência)
- **Entregável do sprint:** Criador/importador de pacotes de capacidade e perfis por hardware.
- **Demonstração exigida:** Levar pacote permitido para outra máquina compatível e usar IA local sem conectividade.
- **Requisitos:** R01, R07, R09, R30, R36. **Risco de integração:** Crítico.

> Proposta de evolução aprovada em ADR-017; não supõe que todas as licenças de pesos permitem redistribuição.

### SL-S65-01 — Definir pacote de capacidade

- **Executar:** Agrupar backend, modelos, licenças, hashes, target e dependências.
- **Produzir:** `schemas/capability-pack-v1.json`.
- **Aceitar quando:** Pacote declara tudo de que precisa e não executa script arbitrário ao importar.
- **Controle:** Contrato; P0 na versão; depende de SL-S32-GATE, SL-S57-GATE, SL-S63-GATE, SL-S64-GATE; evidência em `reports/S65/SL-S65-01.md`.

### SL-S65-02 — Criar construtor de pacotes

- **Executar:** Selecionar somente artefatos com redistribuição permitida e manifest verificado.
- **Produzir:** `tooling/capability-pack/build/`.
- **Aceitar quando:** Peso com termos incompatíveis não é incorporado apenas porque está disponível para download.
- **Controle:** Implementação; P0 na versão; depende de SL-S65-01; evidência em `reports/S65/SL-S65-02.md`.

### SL-S65-03 — Criar importador offline

- **Executar:** Validar assinatura/hash, arquitetura e espaço antes de publicar instalação.
- **Produzir:** `crates/models/src/import_pack.rs`.
- **Aceitar quando:** Pacote errado/corrompido é recusado sem substituir uma instalação funcional.
- **Controle:** Implementação; P0 na versão; depende de SL-S65-02; evidência em `reports/S65/SL-S65-03.md`.

### SL-S65-04 — Deduplicar armazenamento de artefatos

- **Executar:** Compartilhar conteúdo compatível entre perfis preservando versões.
- **Produzir:** `crates/models/src/artifact_store.rs`.
- **Aceitar quando:** Remover um perfil não remove modelo ainda usado por outro.
- **Controle:** Implementação; P0 na versão; depende de SL-S65-03; evidência em `reports/S65/SL-S65-04.md`.

### SL-S65-05 — Criar perfis de hardware

- **Executar:** Recomendar capacidades com base em memória/CPU/GPU detectadas e homologadas.
- **Produzir:** `crates/resources/src/hardware_profiles.rs`.
- **Aceitar quando:** Detecção não concede download/rede automaticamente nem presume aceleração não testada.
- **Controle:** Implementação; P0 na versão; depende de SL-S65-04; evidência em `reports/S65/SL-S65-05.md`.

### SL-S65-06 — Aplicar fallback de capacidade

- **Executar:** Escolher perfil já instalado ou marcar operação indisponível.
- **Produzir:** `crates/models/src/profile_fallback.rs`.
- **Aceitar quando:** Fallback nunca migra para provedor remoto sem consentimento.
- **Controle:** Implementação; P0 na versão; depende de SL-S65-05; evidência em `reports/S65/SL-S65-06.md`.

### SL-S65-07 — Criar UI de instalação/remoção

- **Executar:** Mostrar tamanho, licença, destino e consequências de remover pacote.
- **Produzir:** `apps/desktop/src/settings/capability-packs/`.
- **Aceitar quando:** Remoção não apaga Vault nem resultado autoral produzido pelo modelo.
- **Controle:** Implementação; P0 na versão; depende de SL-S65-06; evidência em `reports/S65/SL-S65-07.md`.

### SL-S65-08 — Testar portabilidade por target

- **Executar:** Instalar packs nos cinco alvos compatíveis e verificar dependências faltantes.
- **Produzir:** `reports/offline/packs-matrix.json`.
- **Aceitar quando:** Pacote de uma arquitetura não é anunciado como universal sem prova.
- **Controle:** Teste; P0 na versão; depende de SL-S65-07; evidência em `reports/S65/SL-S65-08.md`.

### SL-S65-09 — Validar zero egress

- **Executar:** Executar rotas locais completas em rede bloqueada e registrar tentativas.
- **Produzir:** `reports/offline/airgap-v1.2.json`.
- **Aceitar quando:** Nenhum backend tenta buscar tokenizer/modelo auxiliar não declarado.
- **Controle:** Teste; P0 na versão; depende de SL-S65-08; evidência em `reports/S65/SL-S65-09.md`.

### SL-S65-10 — Publicar guia de operação desconectada

- **Executar:** Explicar preparação, atualização, verificação e limites do modo offline.
- **Produzir:** `docs/user/offline-packs.md`.
- **Aceitar quando:** App distingue operação local de instalação inicial que pode requerer aquisição prévia dos artefatos.
- **Controle:** Documentação; P0 na versão; depende de SL-S65-09; evidência em `reports/S65/SL-S65-10.md`.

### SL-S65-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Levar pacote permitido para outra máquina compatível e usar IA local sem conectividade.**

Registrar commit, ambiente, testes e pendências em `reports/S65/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s66"></a>

## S66 — Ampliar interoperabilidade e publicar v1.2

- **Objetivo:** Preservar utilidade do conhecimento fora do Sandland e preparar contratos estáveis.
- **Componente / forma:** Formatos abertos / exportação / Evolução proposta.
- **Pré-requisitos técnicos:** S58 (Implementar migrações, backup e Git opcional); S63 (Atualizar motores com segurança e publicar v1.1); S64 (Escalar busca, ingestão e contexto com evidência); S65 (Consolidar pacotes offline e perfis portáveis)
- **Entregável do sprint:** v1.2.0 com interoperabilidade ampliada explicitamente aprovada.
- **Demonstração exigida:** Exportar mesa/Peça e importar cópia sem perder os campos declarados suportados.
- **Requisitos:** R02, R04, R13, R29, R32, R36. **Risco de integração:** Alto.

> Novos formatos/exportações dependem de aprovação em ADR-017. A versão do formato do Vault é independente da versão do app.

### SL-S66-01 — Aprovar formatos adicionais

- **Executar:** Escolher recorte interoperável, por exemplo JSON Canvas e export editorial local.
- **Produzir:** `docs/product/interoperability-v1.2.md`.
- **Aceitar quando:** Formatos são propostas ratificadas; não se promete compatibilidade irrestrita com todo app de notas.
- **Controle:** Decisão; P0 na versão; depende de SL-S58-GATE, SL-S63-GATE, SL-S64-GATE, SL-S65-GATE; evidência em `reports/S66/SL-S66-01.md`.

### SL-S66-02 — Definir mapeamento de topologia

- **Executar:** Relacionar nós/arestas/grupos e extensões Sandland ao formato escolhido.
- **Produzir:** `schemas/interop/board-mapping.json`.
- **Aceitar quando:** Relação sem representação é preservada em extensão ou relatada como perda antes do export.
- **Controle:** Contrato; P0 na versão; depende de SL-S66-01; evidência em `reports/S66/SL-S66-02.md`.

### SL-S66-03 — Exportar board aberto

- **Executar:** Produzir referências portáveis e metadados de extensão documentados.
- **Produzir:** `crates/export/src/board_interop.rs`.
- **Aceitar quando:** Conteúdo não é duplicado de modo a perder identidade/proveniência.
- **Controle:** Implementação; P2 na versão; depende de SL-S66-02; evidência em `reports/S66/SL-S66-03.md`.

### SL-S66-04 — Importar board em cópia segura

- **Executar:** Resolver paths e gerar IDs/mapeamentos sem autorizar acesso externo indevido.
- **Produzir:** `crates/import/src/board_interop.rs`.
- **Aceitar quando:** Importação não sobrepõe Vault ativo nem lê path arbitrário vindo do arquivo.
- **Controle:** Implementação; P2 na versão; depende de SL-S66-03; evidência em `reports/S66/SL-S66-04.md`.

### SL-S66-05 — Exportar Peça em formato adicional

- **Executar:** Implementar formato aprovado, como HTML/PDF local, com assets e citações.
- **Produzir:** `crates/export/src/editorial.rs`.
- **Aceitar quando:** Não depende de renderer cloud nem recursos remotos não autorizados.
- **Controle:** Implementação; P2 na versão; depende de SL-S66-04; evidência em `reports/S66/SL-S66-05.md`.

### SL-S66-06 — Exportar mapa de evidências

- **Executar:** Incluir fonte/revisão/localizador em manifest legível.
- **Produzir:** `crates/export/src/evidence_manifest.rs`.
- **Aceitar quando:** Texto final continua auditável por ferramenta externa.
- **Controle:** Implementação; P2 na versão; depende de SL-S66-05; evidência em `reports/S66/SL-S66-06.md`.

### SL-S66-07 — Testar round-trip interoperável

- **Executar:** Comparar campos suportados e perdas declaradas em fixtures.
- **Produzir:** `tests/interop/`.
- **Aceitar quando:** Import/export não é chamado lossless quando o formato alvo não representa toda a semântica.
- **Controle:** Teste; P2 na versão; depende de SL-S66-06; evidência em `reports/S66/SL-S66-07.md`.

### SL-S66-08 — Testar migração de v1.0/v1.1

- **Executar:** Rodar corpus de versões e procedimento de restauração.
- **Produzir:** `reports/v1.2/compatibility.json`.
- **Aceitar quando:** Novos exports não quebram formatos canônicos existentes.
- **Controle:** Teste; P2 na versão; depende de SL-S66-07; evidência em `reports/S66/SL-S66-08.md`.

### SL-S66-09 — Publicar especificação de formato

- **Executar:** Fornecer exemplos e validadores para implementadores externos.
- **Produzir:** `docs/specs/interoperability/`.
- **Aceitar quando:** É possível ler/converter dados sem vincular ao runtime do Sandland.
- **Controle:** Documentação; P2 na versão; depende de SL-S66-08; evidência em `reports/S66/SL-S66-09.md`.

### SL-S66-10 — Publicar v1.2 estável

- **Executar:** Consolidar escala, packs offline e interoperabilidade com evidências.
- **Produzir:** `release/v1.2.0/`.
- **Aceitar quando:** Novas capacidades têm limites claros e não mascaram regressões no fluxo v1.0.
- **Controle:** Release; P2 na versão; depende de SL-S66-09; evidência em `reports/S66/SL-S66-10.md`.

### SL-S66-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Exportar mesa/Peça e importar cópia sem perder os campos declarados suportados.**

Registrar commit, ambiente, testes e pendências em `reports/S66/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

# v2.0.0-beta.1 — Extensibilidade controlada

**Maturidade:** Beta — evolução proposta  
**Entrega acumulada:** Contratos de provedores, kit de conformidade e instalação de extensões com permissões explícitas.

<a id="s67"></a>

## S67 — Ratificar desenho da v2.0 e contratos de provedores

- **Objetivo:** Abrir extensibilidade delimitada sem transformar o app em executor genérico de plugins.
- **Componente / forma:** Arquitetura v2 / extensibilidade / Evolução proposta.
- **Pré-requisitos técnicos:** S61 (Estabilizar uso real e ratificar evolução); S66 (Ampliar interoperabilidade e publicar v1.2)
- **Entregável do sprint:** ADR v2.0, ameaça de extensão e superfície de SDK congelada.
- **Demonstração exigida:** Provedor de exemplo tem capacidades explícitas e não acessa APIs internas do Core.
- **Requisitos:** R03, R06, R08, R32, R35. **Risco de integração:** Crítico.

> Proposta pós-v1: a v2.0 não exige colaboração em nuvem, marketplace, execução de scripts nas notas ou reescrita total dos motores.

### SL-S67-01 — Ratificar escopo v2

- **Executar:** Resolver ADR-018 com objetivos observados e não escopo de marketplace/execução irrestrita.
- **Produzir:** `docs/adr/018-provider-sdk.md`.
- **Aceitar quando:** Autor aprova evolução; ausência de aprovação bloqueia implementação, não é presumida pelo roadmap.
- **Controle:** Decisão; P0 na versão; depende de SL-S61-GATE, SL-S66-GATE; evidência em `reports/S67/SL-S67-01.md`.

### SL-S67-02 — Separar APIs internas e públicas

- **Executar:** Expor somente serviços de geração, extração, busca e contexto necessários.
- **Produzir:** `docs/sdk/api-surface.md`.
- **Aceitar quando:** Store canônico e mutação do filesystem não são disponibilizados diretamente à extensão.
- **Controle:** Contrato; P0 na versão; depende de SL-S67-01; evidência em `reports/S67/SL-S67-02.md`.

### SL-S67-03 — Definir versionamento do SDK

- **Executar:** Separar contrato de transporte, capabilities e schema de dados.
- **Produzir:** `schemas/provider-sdk-version.json`.
- **Aceitar quando:** Breaking change de plugin não obriga breaking change do Vault.
- **Controle:** Contrato; P0 na versão; depende de SL-S67-02; evidência em `reports/S67/SL-S67-03.md`.

### SL-S67-04 — Definir manifesto de extensão

- **Executar:** Identificar origem, versão, target, entradas/saídas, rede e recursos solicitados.
- **Produzir:** `schemas/provider-extension-v1.json`.
- **Aceitar quando:** Instalação não concede todos os direitos por padrão.
- **Controle:** Contrato; P0 na versão; depende de SL-S67-03; evidência em `reports/S67/SL-S67-04.md`.

### SL-S67-05 — Definir modelo de confiança

- **Executar:** Distinguir componentes curados, código de terceiros e perfis experimentais.
- **Produzir:** `docs/security/extensions-trust.md`.
- **Aceitar quando:** Assinatura prova origem/integridade, não ausência de comportamento malicioso.
- **Controle:** Decisão; P0 na versão; depende de SL-S67-04; evidência em `reports/S67/SL-S67-05.md`.

### SL-S67-06 — Atualizar ameaça de extensões

- **Executar:** Modelar exfiltração, resposta maliciosa, resource abuse e tentativa de ampliar escopo.
- **Produzir:** `docs/security/extensions-threat-model.md`.
- **Aceitar quando:** Riscos não são tratados como resolvidos apenas por executar em outro processo.
- **Controle:** Decisão; P0 na versão; depende de SL-S67-05; evidência em `reports/S67/SL-S67-06.md`.

### SL-S67-07 — Definir APIs de dados minimizados

- **Executar:** Passar snapshot/refs limitados em vez de path do Vault.
- **Produzir:** `schemas/sdk-input-refs.json`.
- **Aceitar quando:** Plugin recebe só os dados necessários à tarefa aprovada.
- **Controle:** Contrato; P0 na versão; depende de SL-S67-06; evidência em `reports/S67/SL-S67-07.md`.

### SL-S67-08 — Definir errors e lifecycle

- **Executar:** Padronizar cancelamento, shutdown, health e incompatibilidade.
- **Produzir:** `schemas/sdk-errors.json`.
- **Aceitar quando:** Provedor quebrado pode ser desabilitado sem corromper conteúdo ou impedir abertura do app.
- **Controle:** Contrato; P0 na versão; depende de SL-S67-07; evidência em `reports/S67/SL-S67-08.md`.

### SL-S67-09 — Criar fixtures de conformidade v2

- **Executar:** Derivar casos de providers existentes e ataques de escopo.
- **Produzir:** `tests/sdk/fixtures/`.
- **Aceitar quando:** SDK tem testes antes de uma extensão real depender de comportamento não especificado.
- **Controle:** Teste; P0 na versão; depende de SL-S67-08; evidência em `reports/S67/SL-S67-09.md`.

### SL-S67-10 — Planejar migração interna

- **Executar:** Listar quais adapters atuais serão encaixados no SDK e ordem segura.
- **Produzir:** `docs/sdk/internal-migration.md`.
- **Aceitar quando:** Refatoração não troca motores escolhidos nem remove capacidades estáveis por conveniência.
- **Controle:** Governança; P0 na versão; depende de SL-S67-09; evidência em `reports/S67/SL-S67-10.md`.

### SL-S67-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Provedor de exemplo tem capacidades explícitas e não acessa APIs internas do Core.**

Registrar commit, ambiente, testes e pendências em `reports/S67/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s68"></a>

## S68 — Implementar SDK e kit de conformidade

- **Objetivo:** Tornar integrações reproduzíveis e verificáveis por contribuidores.
- **Componente / forma:** Provider SDK / adapters / Evolução proposta.
- **Pré-requisitos técnicos:** S67 (Ratificar desenho da v2.0 e contratos de provedores)
- **Entregável do sprint:** SDK mínimo, mocks e harness de contratos.
- **Demonstração exigida:** Criar provedor local de exemplo e testar comportamentos permitidos/negados sem chaves.
- **Requisitos:** R06, R08, R27, R32, R34, R35. **Risco de integração:** Crítico.

### SL-S68-01 — Gerar tipos do contrato

- **Executar:** Produzir bindings necessários aos adapters existentes a partir de schemas versionados.
- **Produzir:** `sdk/generated/`.
- **Aceitar quando:** Tipos de linguagens diferentes serializam os mesmos casos sem ambiguidade.
- **Controle:** Implementação; P0 na versão; depende de SL-S67-GATE; evidência em `reports/S68/SL-S68-01.md`.

### SL-S68-02 — Criar host SDK

- **Executar:** Negociar versão/capabilities e integrar o supervisor existente.
- **Produzir:** `crates/provider-host/`.
- **Aceitar quando:** Nova camada não cria outro lifecycle ou outro sistema de permissões concorrente.
- **Controle:** Implementação; P0 na versão; depende de SL-S68-01; evidência em `reports/S68/SL-S68-02.md`.

### SL-S68-03 — Criar cliente SDK mínimo

- **Executar:** Facilitar implementadores sem dar acesso direto ao Core.
- **Produzir:** `sdk/client/`.
- **Aceitar quando:** Exemplo precisa somente de inputs autorizados e saídas por contrato.
- **Controle:** Implementação; P0 na versão; depende de SL-S68-02; evidência em `reports/S68/SL-S68-03.md`.

### SL-S68-04 — Criar harness offline

- **Executar:** Executar contratos com transport/model fakes e fixture controlada.
- **Produzir:** `sdk/testkit/`.
- **Aceitar quando:** CI de terceiro pode validar comportamento básico sem API paga ou documentos privados.
- **Controle:** Implementação; P0 na versão; depende de SL-S68-03; evidência em `reports/S68/SL-S68-04.md`.

### SL-S68-05 — Migrar provider gerativo existente

- **Executar:** Encaixar adapter LocalAI/BYOK sem alterar política de egress.
- **Produzir:** `adapters/generation/sdk_bridge/`.
- **Aceitar quando:** Capabilities e identidade de provedor continuam preservadas.
- **Controle:** Implementação; P0 na versão; depende de SL-S68-04; evidência em `reports/S68/SL-S68-05.md`.

### SL-S68-06 — Migrar provider de extração

- **Executar:** Encaixar Docling/normalizador com mesmas restrições de I/O.
- **Produzir:** `adapters/extraction/sdk_bridge/`.
- **Aceitar quando:** Plugin não obtém escrita canônica como resultado da refatoração.
- **Controle:** Implementação; P0 na versão; depende de SL-S68-05; evidência em `reports/S68/SL-S68-06.md`.

### SL-S68-07 — Migrar interfaces de busca/contexto

- **Executar:** Expor backend escolhido e PageIndex adaptado sem vazar internals.
- **Produzir:** `adapters/retrieval/sdk_bridge/`.
- **Aceitar quando:** IDs/revisões/escopo não mudam por empacotamento em SDK.
- **Controle:** Implementação; P0 na versão; depende de SL-S68-06; evidência em `reports/S68/SL-S68-07.md`.

### SL-S68-08 — Criar provider deliberadamente inválido

- **Executar:** Retornar dados fora de schema/escopo e exceder budget.
- **Produzir:** `sdk/examples/invalid-provider/`.
- **Aceitar quando:** Host rejeita violações e coleta evidência sem aceitar conteúdo malicioso.
- **Controle:** Teste; P0 na versão; depende de SL-S68-07; evidência em `reports/S68/SL-S68-08.md`.

### SL-S68-09 — Comparar pré e pós-SDK

- **Executar:** Rodar contratos e corpora de qualidade da versão estável.
- **Produzir:** `reports/sdk/differential.json`.
- **Aceitar quando:** Refatoração não se considera concluída apenas porque compila.
- **Controle:** Teste; P0 na versão; depende de SL-S68-08; evidência em `reports/S68/SL-S68-09.md`.

### SL-S68-10 — Publicar tutorial mínimo

- **Executar:** Explicar contrato, permissões, testes, licenças e submissão de integração.
- **Produzir:** `docs/sdk/getting-started.md`.
- **Aceitar quando:** Contribuidor consegue executar exemplo offline sem copiar internals de outro adapter.
- **Controle:** Documentação; P0 na versão; depende de SL-S68-09; evidência em `reports/S68/SL-S68-10.md`.

### SL-S68-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Criar provedor local de exemplo e testar comportamentos permitidos/negados sem chaves.**

Registrar commit, ambiente, testes e pendências em `reports/S68/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s69"></a>

## S69 — Instalar extensões com permissões controladas

- **Objetivo:** Adicionar extensões sem perder soberania, segurança ou previsibilidade operacional.
- **Componente / forma:** ExtensionHost / distribuição / Evolução proposta.
- **Pré-requisitos técnicos:** S68 (Implementar SDK e kit de conformidade)
- **Entregável do sprint:** Instalação local/curada, revogação e lifecycle de extensões.
- **Demonstração exigida:** Instalar provedor de exemplo, negar rede, revogar e remover sem tocar dados autorais.
- **Requisitos:** R06, R08, R09, R30, R33, R35. **Risco de integração:** Crítico.

### SL-S69-01 — Validar pacote de extensão

- **Executar:** Checar manifesto, hashes, target e licença antes de extrair/executar.
- **Produzir:** `crates/extensions/src/package.rs`.
- **Aceitar quando:** Zip slip/paths externos e artefatos sem origem são rejeitados.
- **Controle:** Implementação; P0 na versão; depende de SL-S68-GATE; evidência em `reports/S69/SL-S69-01.md`.

### SL-S69-02 — Criar fluxo de consentimento

- **Executar:** Mostrar dados recebidos, destino de rede e recursos pedidos.
- **Produzir:** `apps/desktop/src/extensions/permissions.ts`.
- **Aceitar quando:** Instalar não equivale a permitir acesso a todo Vault ou todas as APIs.
- **Controle:** Implementação; P0 na versão; depende de SL-S69-01; evidência em `reports/S69/SL-S69-02.md`.

### SL-S69-03 — Vincular permissões a tarefa

- **Executar:** Gerar capabilities limitadas por workspace/revisão e operação.
- **Produzir:** `crates/extensions/src/capabilities.rs`.
- **Aceitar quando:** Extensão não pode ampliar autorização através da própria resposta.
- **Controle:** Implementação; P0 na versão; depende de SL-S69-02; evidência em `reports/S69/SL-S69-03.md`.

### SL-S69-04 — Isolar execução

- **Executar:** Reutilizar perfis de sandbox adequados e recusar combinação não suportada.
- **Produzir:** `crates/extensions/src/sandbox.rs`.
- **Aceitar quando:** Falta de contenção não provoca execução irrestrita para manter compatibilidade.
- **Controle:** Implementação; P0 na versão; depende de SL-S69-03; evidência em `reports/S69/SL-S69-04.md`.

### SL-S69-05 — Controlar rede por provedor

- **Executar:** Roteamento/destinos compatíveis com política já aprovada pelo usuário.
- **Produzir:** `crates/extensions/src/egress.rs`.
- **Aceitar quando:** Permissão local não habilita serviço remoto adicionado posteriormente pela extensão.
- **Controle:** Implementação; P0 na versão; depende de SL-S69-04; evidência em `reports/S69/SL-S69-05.md`.

### SL-S69-06 — Gerenciar update e rollback

- **Executar:** Tratar mudança de permissões como nova aprovação.
- **Produzir:** `crates/extensions/src/update.rs`.
- **Aceitar quando:** Atualização não aumenta direitos silenciosamente nem apaga configuração funcional sem retorno.
- **Controle:** Implementação; P0 na versão; depende de SL-S69-05; evidência em `reports/S69/SL-S69-06.md`.

### SL-S69-07 — Implementar revogação

- **Executar:** Cancelar jobs, invalidar tokens e impedir novas chamadas do provedor.
- **Produzir:** `crates/extensions/src/revoke.rs`.
- **Aceitar quando:** Jobs em andamento não continuam usando segredo após revogação sem controle.
- **Controle:** Implementação; P0 na versão; depende de SL-S69-06; evidência em `reports/S69/SL-S69-07.md`.

### SL-S69-08 — Implementar remoção

- **Executar:** Encerrar processos e retirar pacote/cache preservando dados autorais.
- **Produzir:** `crates/extensions/src/remove.rs`.
- **Aceitar quando:** Remover provider não remove textos, summaries aceitos ou evidências do usuário.
- **Controle:** Implementação; P0 na versão; depende de SL-S69-07; evidência em `reports/S69/SL-S69-08.md`.

### SL-S69-09 — Testar abuso de recursos e dados

- **Executar:** Usar extensão de prova que tenta exfiltrar/forçar respostas excessivas.
- **Produzir:** `tests/extensions/security/`.
- **Aceitar quando:** Host bloqueia violações dentro do modelo de ameaça documentado.
- **Controle:** Teste; P0 na versão; depende de SL-S69-08; evidência em `reports/S69/SL-S69-09.md`.

### SL-S69-10 — Documentar distribuição curada

- **Executar:** Definir fonte correspondente, notices e limites de endosso de terceiros.
- **Produzir:** `docs/extensions/distribution.md`.
- **Aceitar quando:** Projeto não promete segurança automática de qualquer plugin disponível na internet.
- **Controle:** Licença; P0 na versão; depende de SL-S69-09; evidência em `reports/S69/SL-S69-10.md`.

### SL-S69-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Instalar provedor de exemplo, negar rede, revogar e remover sem tocar dados autorais.**

Registrar commit, ambiente, testes e pendências em `reports/S69/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

<a id="s70"></a>

## S70 — Atualizar recortes e consolidar beta v2.0

- **Objetivo:** Consolidar arquitetura madura sem exigir reescrita do Vault ou perda de compatibilidade.
- **Componente / forma:** Upstreams / SDK / formato / Integração.
- **Pré-requisitos técnicos:** S68 (Implementar SDK e kit de conformidade); S69 (Instalar extensões com permissões controladas)
- **Entregável do sprint:** v2.0.0-beta.1 com motores atualizados e adapters conformes.
- **Demonstração exigida:** Executar o fluxo completo usando adapters sob SDK e manter dados v1.x intactos.
- **Requisitos:** R01, R04, R22, R30, R32, R33, R35. **Risco de integração:** Crítico.

### SL-S70-01 — Revisar revisões-alvo v2

- **Executar:** Selecionar updates necessários com changelogs, licenças e riscos.
- **Produzir:** `upstream/releases/v2-candidates.json`.
- **Aceitar quando:** Não se usa alteração de major para atualizar tudo sem teste.
- **Controle:** Upstream; P0 na versão; depende de SL-S68-GATE, SL-S69-GATE; evidência em `reports/S70/SL-S70-01.md`.

### SL-S70-02 — Rebasear BlockSuite

- **Executar:** Validar codecs, IDs, input e montagem mínima na nova revisão escolhida.
- **Produzir:** `reports/upstream/v2-blocksuite.md`.
- **Aceitar quando:** Editor não incorpora o produto AFFiNE ou canvas concorrente durante update.
- **Controle:** Upstream; P0 na versão; depende de SL-S70-01; evidência em `reports/S70/SL-S70-02.md`.

### SL-S70-03 — Rebasear PageIndex

- **Executar:** Preservar portas de modelos/fontes, IDs e relações fora da árvore.
- **Produzir:** `reports/upstream/v2-pageindex.md`.
- **Aceitar quando:** Mudanças upstream não reintroduzem armazenamento canônico privado ou leitura global.
- **Controle:** Upstream; P0 na versão; depende de SL-S70-02; evidência em `reports/S70/SL-S70-03.md`.

### SL-S70-04 — Revalidar ingest e web

- **Executar:** Rodar corpus Docling/Readability/Turndown/Scrapling e contrato SearxNG.
- **Produzir:** `reports/upstream/v2-ingest-web.json`.
- **Aceitar quando:** Formatos/engines quebrados geram degradação clara, não corrupção silenciosa.
- **Controle:** Upstream; P0 na versão; depende de SL-S70-03; evidência em `reports/S70/SL-S70-04.md`.

### SL-S70-05 — Revalidar runtime e busca

- **Executar:** Avaliar LocalAI/backends e o banco escolhido, mantendo espaço de embeddings identificado.
- **Produzir:** `reports/upstream/v2-runtime-search.json`.
- **Aceitar quando:** Troca de modelo/índice exige caminho explícito; não há mistura de vetores.
- **Controle:** Upstream; P0 na versão; depende de SL-S70-04; evidência em `reports/S70/SL-S70-05.md`.

### SL-S70-06 — Remover acoplamentos antigos

- **Executar:** Retirar adapters obsoletos somente após paridade e migração testadas.
- **Produzir:** `refactors/v2/`.
- **Aceitar quando:** Nenhuma funcionalidade estável desaparece só porque a fachada mudou.
- **Controle:** Implementação; P0 na versão; depende de SL-S70-05; evidência em `reports/S70/SL-S70-06.md`.

### SL-S70-07 — Versionar formato apenas se necessário

- **Executar:** Implementar migração real quando houver mudança autoral aprovada.
- **Produzir:** `crates/migrations/src/v2.rs`.
- **Aceitar quando:** Número v2.0 do app não força alteração gratuita no formato do Vault.
- **Controle:** Implementação; P0 na versão; depende de SL-S70-06; evidência em `reports/S70/SL-S70-07.md`.

### SL-S70-08 — Rodar pacote completo de conformidade

- **Executar:** Testar providers internos como se fossem integrações externas.
- **Produzir:** `reports/v2-beta/provider-conformance.json`.
- **Aceitar quando:** Nenhum provider privilegiado ignora regras que seriam exigidas de um terceiro.
- **Controle:** Teste; P0 na versão; depende de SL-S70-07; evidência em `reports/S70/SL-S70-08.md`.

### SL-S70-09 — Fechar inventário beta

- **Executar:** Atualizar SBOM, fontes, modelos e manifests de extensão por alvo.
- **Produzir:** `compliance/releases/v2.0.0-beta.1/`.
- **Aceitar quando:** Pacotes têm origem/licença correspondente à revisão efetivamente construída.
- **Controle:** Licença; P0 na versão; depende de SL-S70-08; evidência em `reports/S70/SL-S70-09.md`.

### SL-S70-10 — Publicar beta v2

- **Executar:** Documentar SDK, limites, migração e retorno à linha v1 suportada.
- **Produzir:** `release/v2.0.0-beta.1/`.
- **Aceitar quando:** Beta exige backup/consentimento quando houver mudança de formato; não usa Vault único sem proteção.
- **Controle:** Release; P0 na versão; depende de SL-S70-09; evidência em `reports/S70/SL-S70-10.md`.

### SL-S70-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Executar o fluxo completo usando adapters sob SDK e manter dados v1.x intactos.**

Registrar commit, ambiente, testes e pendências em `reports/S70/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

# v2.0.0-rc.1 — Compatibilidade consolidada

**Maturidade:** RC  
**Entrega acumulada:** Migração de v1.x, rollback e matriz completa de regressão de motores/formatos.

<a id="s71"></a>

## S71 — Homologar compatibilidade e candidata v2.0

- **Objetivo:** Comprovar que extensibilidade e evolução não romperam as promessas originais.
- **Componente / forma:** Compatibilidade / RC v2 / Próprio.
- **Pré-requisitos técnicos:** S70 (Atualizar recortes e consolidar beta v2.0)
- **Entregável do sprint:** v2.0.0-rc.1 e matriz de upgrade/rollback completa.
- **Demonstração exigida:** Migrar cópias de Vaults v1.x, usar extensões e voltar/exportar sem perda autoral.
- **Requisitos:** R01, R02, R05, R06, R07, R09, R32, R35. **Risco de integração:** Crítico.

### SL-S71-01 — Montar matriz histórica

- **Executar:** Incluir v1.0, v1.1, v1.2 e estados de recursos opcionais.
- **Produzir:** `tests/migrations/v2-matrix.yaml`.
- **Aceitar quando:** Cada origem possui fixture e expectativa de upgrade/downgrade/export explícita.
- **Controle:** Teste; P0 na versão; depende de SL-S70-GATE; evidência em `reports/S71/SL-S71-01.md`.

### SL-S71-02 — Executar upgrades interrompidos

- **Executar:** Matar processo, simular disco cheio e pacote faltante durante migração.
- **Produzir:** `reports/v2-rc/upgrade-faults.json`.
- **Aceitar quando:** Estado pode ser recuperado sem confiar em índice vetorial ou extensão instalada.
- **Controle:** Teste; P0 na versão; depende de SL-S71-01; evidência em `reports/S71/SL-S71-02.md`.

### SL-S71-03 — Testar rollback e export de emergência

- **Executar:** Validar retorno suportado e alternativa quando downgrade não for possível.
- **Produzir:** `reports/v2-rc/rollback.json`.
- **Aceitar quando:** Limitações são mostradas antes de migrar, não descobertas após perda de acesso.
- **Controle:** Teste; P0 na versão; depende de SL-S71-02; evidência em `reports/S71/SL-S71-03.md`.

### SL-S71-04 — Repetir prova File-as-Truth

- **Executar:** Apagar índices, caches de extensão e stores internos dos motores.
- **Produzir:** `reports/v2-rc/file-truth.json`.
- **Aceitar quando:** Autoria, citações, topologia e histórico aprovados continuam recuperáveis.
- **Controle:** Teste; P0 na versão; depende de SL-S71-03; evidência em `reports/S71/SL-S71-04.md`.

### SL-S71-05 — Repetir isolamento completo

- **Executar:** Incluir extensões, novos backends e políticas por plataforma.
- **Produzir:** `reports/v2-rc/security.json`.
- **Aceitar quando:** Refatoração do SDK não abriu rota alternativa para filesystem/rede.
- **Controle:** Teste; P0 na versão; depende de SL-S71-04; evidência em `reports/S71/SL-S71-05.md`.

### SL-S71-06 — Repetir offline completo

- **Executar:** Provisionar pacotes e usar todos os caminhos locais homologados.
- **Produzir:** `reports/v2-rc/offline.json`.
- **Aceitar quando:** Extensão sem rede não tenta instalar/download por conta própria.
- **Controle:** Teste; P0 na versão; depende de SL-S71-05; evidência em `reports/S71/SL-S71-06.md`.

### SL-S71-07 — Repetir performance/qualidade

- **Executar:** Comparar budgets, cenas, recuperação e resumos contra baselines estáveis.
- **Produzir:** `reports/v2-rc/regression.json`.
- **Aceitar quando:** Novos recursos não justificam regressão silenciosa nos requisitos essenciais.
- **Controle:** Teste; P0 na versão; depende de SL-S71-06; evidência em `reports/S71/SL-S71-07.md`.

### SL-S71-08 — Fazer piloto de atualização

- **Executar:** Observar uso da candidata com dados permitidos e diagnóstico saneado.
- **Produzir:** `reports/v2-rc/pilot.md`.
- **Aceitar quando:** Feedback real e ensaio interno são identificados separadamente.
- **Controle:** Teste; P0 na versão; depende de SL-S71-07; evidência em `reports/S71/SL-S71-08.md`.

### SL-S71-09 — Fechar blockers e compatibilidade

- **Executar:** Revisar breaking changes, deprecações e suporte da linha anterior.
- **Produzir:** `docs/releases/v2-compatibility.md`.
- **Aceitar quando:** Contribuidor/usuário sabe quais contratos mudaram e como migrar.
- **Controle:** Governança; P0 na versão; depende de SL-S71-08; evidência em `reports/S71/SL-S71-09.md`.

### SL-S71-10 — Publicar RC v2

- **Executar:** Emitir pacotes, manifests e guia de migração final candidato.
- **Produzir:** `release/v2.0.0-rc.1/`.
- **Aceitar quando:** Revisão da RC corresponde às evidências e está pronta para go/no-go humano.
- **Controle:** Release; P0 na versão; depende de SL-S71-09; evidência em `reports/S71/SL-S71-10.md`.

### SL-S71-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Migrar cópias de Vaults v1.x, usar extensões e voltar/exportar sem perda autoral.**

Registrar commit, ambiente, testes e pendências em `reports/S71/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

# v2.0.0 — Plataforma estável e sustentável

**Maturidade:** Estável — fim do horizonte  
**Entrega acumulada:** Base madura, interoperável e extensível; continuidade de manutenção, não fim definitivo do projeto.

<a id="s72"></a>

## S72 — Publicar v2.0 e estabelecer continuidade

- **Objetivo:** Fechar o horizonte com produto completo, mantível e aberto, não com um fim artificial do software.
- **Componente / forma:** Release / sustentabilidade / Próprio.
- **Pré-requisitos técnicos:** S71 (Homologar compatibilidade e candidata v2.0)
- **Entregável do sprint:** v2.0.0 estável, SDK documentado e processo contínuo de manutenção.
- **Demonstração exigida:** Instalar stable, importar pesquisa, produzir Peça e verificar portabilidade/rollback de capacidades.
- **Requisitos:** R01, R02, R05, R07, R30, R32, R33, R34, R35, R36. **Risco de integração:** Crítico.

### SL-S72-01 — Auditar escopo final

- **Executar:** Conferir requisitos originais e propostas aprovadas em ADR-017/018.
- **Produzir:** `reports/releases/v2-traceability.md`.
- **Aceitar quando:** Requisito não aprovado não é declarado entregue; requisito essencial faltante bloqueia stable.
- **Controle:** Governança; P0 na versão; depende de SL-S71-GATE; evidência em `reports/S72/SL-S72-01.md`.

### SL-S72-02 — Rodar regressão final

- **Executar:** Executar suites em commit exato do release nos cinco alvos e perfis homologados.
- **Produzir:** `reports/releases/v2-final-suite.json`.
- **Aceitar quando:** Build, testes e artefatos compartilham identidade verificável.
- **Controle:** Teste; P0 na versão; depende de SL-S72-01; evidência em `reports/S72/SL-S72-02.md`.

### SL-S72-03 — Verificar distribuição limpa

- **Executar:** Instalar sem toolchains globais, provisionar offline e testar update/removal.
- **Produzir:** `reports/releases/v2-clean-install.md`.
- **Aceitar quando:** Usuário não precisa reconstruir ambientes de desenvolvimento para usar o produto.
- **Controle:** Teste; P0 na versão; depende de SL-S72-02; evidência em `reports/S72/SL-S72-03.md`.

### SL-S72-04 — Fechar conformidade v2

- **Executar:** Revisar fontes/avisos, SDK, extensões curadas e modelos redistribuídos.
- **Produzir:** `compliance/releases/v2.0.0/`.
- **Aceitar quando:** Natureza open source/nonprofit e dependência opcional de serviços são descritas corretamente.
- **Controle:** Licença; P0 na versão; depende de SL-S72-03; evidência em `reports/S72/SL-S72-04.md`.

### SL-S72-05 — Publicar documentação estável

- **Executar:** Consolidar usuário, formato, recuperação, SDK e contribuição.
- **Produzir:** `docs/releases/v2.0.0.md`.
- **Aceitar quando:** Guias refletem APIs/capacidades publicadas e não protótipos abandonados.
- **Controle:** Documentação; P0 na versão; depende de SL-S72-04; evidência em `reports/S72/SL-S72-05.md`.

### SL-S72-06 — Definir manutenção pós-v2

- **Executar:** Estabelecer triagem, patches, cadence de auditoria e política de depreciação sustentável.
- **Produzir:** `docs/maintenance/post-v2.md`.
- **Aceitar quando:** Roadmap não promete última versão definitiva nem suporte impossível para um mantenedor.
- **Controle:** Governança; P0 na versão; depende de SL-S72-05; evidência em `reports/S72/SL-S72-06.md`.

### SL-S72-07 — Preparar continuidade comunitária

- **Executar:** Documentar builds, acesso de release, backups de chaves e sucessão sem divulgar segredos.
- **Produzir:** `docs/governance/continuity.md`.
- **Aceitar quando:** Outro mantenedor pode reconstruir o projeto com procedimentos públicos e permissões apropriadas.
- **Controle:** Governança; P0 na versão; depende de SL-S72-06; evidência em `reports/S72/SL-S72-07.md`.

### SL-S72-08 — Fazer go/no-go final

- **Executar:** Revisar blockers, métricas e a demonstração integral com aprovação humana.
- **Produzir:** `reports/releases/v2-go-no-go.md`.
- **Aceitar quando:** Agente não pode marcar aprovação final sozinho; exceções materiais ficam registradas.
- **Controle:** Decisão; P0 na versão; depende de SL-S72-07; evidência em `reports/S72/SL-S72-08.md`.

### SL-S72-09 — Publicar artefatos stable

- **Executar:** Disponibilizar versões, hashes/assinaturas, fontes e notas no canal escolhido.
- **Produzir:** `release/v2.0.0/`.
- **Aceitar quando:** Downloads e documentação correspondem à mesma revisão; possibilidade de rollback do canal existe.
- **Controle:** Release; P0 na versão; depende de SL-S72-08; evidência em `reports/S72/SL-S72-09.md`.

### SL-S72-10 — Verificar publicação e handoff

- **Executar:** Instalar pelo canal real, checar links e registrar próximos riscos sem inventar novas features.
- **Produzir:** `reports/releases/v2-post-publish.md`.
- **Aceitar quando:** Horizonte concluído com evidências e fila de manutenção, não com todos os problemas futuros declarados resolvidos.
- **Controle:** Teste; P0 na versão; depende de SL-S72-09; evidência em `reports/S72/SL-S72-10.md`.

### SL-S72-GATE — Encerramento por aceite

Revisão humana das dez tarefas e da demonstração: **Instalar stable, importar pesquisa, produzir Peça e verificar portabilidade/rollback de capacidades.**

Registrar commit, ambiente, testes e pendências em `reports/S72/acceptance.md`. Só fechar quando os critérios particulares e a Definition of Done forem satisfeitos. Se houver blocker, manter o sprint aberto e registrar ação corretiva.

---

---

## H. Requisitos e rastreabilidade

| ID | Área | Requisito | Origem | Sprints | Verificação |
| --- | --- | --- | --- | --- | --- |
| R01 | Distribuição | Linux x86_64/aarch64; Windows x86_64; macOS Apple Silicon/Intel | RFC §§1,12,13 | S01, S05, S06, S19, S24, S25, S26, S38, S39, S49, S56, S57, S59, S60, S63, S65, S70, S71, S72 | Instalação limpa e matriz de capacidades por alvo; build cruzado não equivale a execução homologada. |
| R02 | Soberania | Arquivos abertos são a autoridade; bancos e caches são reconstruíveis | RFC §§1,2,4 | S02, S07, S08, S10, S11, S13, S17, S21, S22, S30, S35, S37, S40, S41, S43, S53, S58, S60, S62, S66, S71, S72 | Apagar caches preserva conteúdo, geometria, taxonomia, citações e histórico canônico. |
| R03 | Composição | Núcleo Rust com motores gerenciados, condicionado à ADR | Plano §1 | S01, S03, S04, S06, S12, S23, S31, S42, S67 | Alteração de Rust estrito aprovada; nenhum runtime é introduzido sem inventário e lifecycle. |
| R04 | Formato | Markdown/frontmatter, IDs e extensões abertas documentadas | RFC §§2,4,7 | S02, S07, S10, S13, S14, S18, S20, S43, S66, S70 | Round-trip sem perda semântica no perfil suportado; metadados desconhecidos preservados. |
| R05 | Durabilidade | Journal/snapshots, recuperação e conflitos externos | RFC §§4,11,13 | S02, S07, S08, S10, S11, S14, S17, S21, S22, S23, S41, S54, S58, S60, S61, S62, S71, S72 | Falhas em cada etapa não corrompem estado confirmado; janela de perda medida. |
| R06 | Segurança FS | I/O por handles/capabilities e contenção por sistema operacional | RFC §§1,4,8,13 | S09, S11, S23, S24, S25, S26, S44, S48, S49, S54, S56, S57, S60, S67, S68, S69, S71 | Tentativas de traversal, troca de links e fuga de processos são negadas nos perfis homologados. |
| R07 | Offline | Autoria e leitura sem rede; inferência local após provisionamento | RFC §1; conceito | S01, S06, S07, S11, S15, S22, S31, S32, S34, S35, S46, S53, S59, S60, S65, S71, S72 | Bloqueio de rede não impede trabalho local nem dispara fallback externo silencioso. |
| R08 | Privacidade | Autorização de egress, segregação de segredos e escopos | RFC §§1,8 | S01, S06, S09, S23, S24, S25, S26, S29, S31, S32, S34, S44, S47, S48, S49, S51, S52, S54, S59, S62, S67, S68, S69 | Nenhum envio sem política autorizada; ferramentas não podem ampliar seu próprio escopo. |
| R09 | Recursos | Idle 350 MB, indexing leve 650 MB, inference 2,8 GB e unload 5 min | RFC §§1,10,13 | S05, S15, S19, S23, S24, S25, S26, S28, S31, S32, S36, S45, S49, S55, S56, S57, S60, S61, S64, S65, S69, S71 | Medir árvore de processos e perfis aprovados; desvios bloqueiam release ou requerem revisão explícita. |
| R10 | Editor | BlockSuite como motor de texto/blocos | Escolha do autor; RFC §§5,7 | S12, S13, S14, S15, S18, S20 | Editor integrado ao Vault; Yjs não é a única cópia de conteúdo autoral. |
| R11 | Mesa | PixiJS, células, grupos, arestas e múltiplos workspaces | RFC §5 | S02, S16, S17, S18, S19, S43, S50, S53 | Topologia reaberta fielmente; conteúdo e ocorrência visual têm identidades separadas. |
| R12 | Renderização | Culling, LOD, overlay editorial e fluidez | RFC §§5,13 | S05, S16, S18, S19, S55, S64 | Cenas de 100/1.000 nós avaliadas com frame time, hardware e payload documentados. |
| R13 | Peça | Editor editorial, fork-on-insert e proveniência | RFC §7 | S02, S08, S13, S20, S21, S22, S41, S46, S51, S53, S66 | Inserção cria cópia independente referenciando célula, revisão, trecho e bloco de destino. |
| R14 | Ingest | Captura sem formulário e enriquecimento assíncrono | RFC §3; conceito | S27, S28, S29, S30, S33, S36, S50, S53, S64 | Original salvo antes de OCR/IA; falha de enriquecimento não perde a captura. |
| R15 | Extração | Docling para documentos/OCR com mapeamento à fonte | Escolha do autor | S05, S27, S28, S30 | Texto, tabelas e localizadores preservados; worker não escreve no Vault canônico. |
| R16 | Transcrição | Áudio/vídeo e transcrições locais | RFC §§1,2,10 | S36 | Modelo homologado, timestamps e fonte rastreáveis; cancelamento e recursos controlados. |
| R17 | Clipping | Readability/Turndown em ambiente seguro | Escolha do autor | S29, S30 | HTML remoto não ganha scripts, rede ou IPC privilegiado durante normalização. |
| R18 | Decisões | Jev nativo, taxonomia e tratamento de incerteza | Escolha do autor; RFC §3 revisado | S34, S35, S36, S52 | Choice/Score/Noul respeitados; novos termos e merges não são inferidos como certeza. |
| R19 | Resumo | Resumo executivo denso de 2–5 frases por modelo leve | Escolha do autor | S33, S36, S45 | Texto separado da decisão Jev; fatos e limitações rastreáveis; edição autoral preservada. |
| R20 | Embeddings | Embeddings locais e adequados ao português | RFC §§3,4,10 | S05, S32, S35, S37, S38, S39, S40, S41, S55 | Manifesto de modelo, normalização e versão; espaços vetoriais incompatíveis não se misturam. |
| R21 | Busca | BM25 + semântica para Ingest e Peças; Qdrant OU HelixDB | Escolha do autor; RFC §4 | S10, S37, S38, S39, S40, S41, S52, S64 | Busca filtrada, fusão e exclusões/revisões corretas, com reconstrução a partir dos arquivos. |
| R22 | Contexto | PageIndex adaptado a células e topologia | Escolha do autor; RFC §8 | S17, S42, S43, S44, S45, S46, S64, S70 | Hierarquia navega sem apagar ciclos, relações cruzadas, geometria e identidade dos nós. |
| R23 | Orquestração | Skeleton Map, expansão e síntese progressivas | RFC §8 | S33, S42, S44, S45, S46, S51, S52, S55 | Orçamento de tokens/chamadas/tempo/recursão imposto em código; resposta cita evidências. |
| R24 | Descoberta | SearxNG com consultas autorizadas | Escolha do autor; RFC §6 | S47, S50 | API de resultados configurada; rede externa explícita; ausência de dependência pública oculta. |
| R25 | Aquisição web | Scrapling HTTP/browser sob demanda | Escolha do autor; RFC §6 | S48, S49, S50 | SSRF e recursos controlados; não prometer bypass universal; browser pode encerrar sem órfãos. |
| R26 | Widget | Pesquisa, snapshots e promoção para células | RFC §§5,6 | S50, S53 | Atualizar resultados não sobrescreve síntese editada; snippets não viram prova sem fonte. |
| R27 | IA gerativa | LocalAI e BYOK sob política comum | Escolha do autor; RFC §§7,10 | S31, S32, S33, S36, S46, S51, S68 | Capabilities verificadas, streaming/cancelamento e identidade de provedor preservados. |
| R28 | Intenções | Metas, hipóteses, chat global e sugestões proativas | RFC §9 | S35, S52, S53 | Escopo e finalidade explícitos; sugestão não altera conhecimento sem confirmação. |
| R29 | Portabilidade | Backup, restauração e integração Git opcional | RFC §11 | S11, S22, S58, S62, S66 | Exportação/restauração verificadas; push remoto exige configuração e autorização. |
| R30 | Supply chain | Pacotes, hashes, SBOM, atualização e rollback | RFC §10; Plano §14 | S03, S04, S12, S25, S26, S27, S28, S29, S31, S32, S38, S39, S40, S42, S47, S48, S49, S54, S56, S57, S63, S65, S69, S70, S72 | Instalação íntegra por arquitetura, cadeia de origem e fontes/avisos disponíveis. |
| R31 | Usabilidade | Acessibilidade, teclado, erros e documentação | Derivado do produto desktop | S06, S14, S15, S16, S18, S19, S20, S22, S30, S41, S50, S51, S59, S61, S62 | Fluxos essenciais acessíveis e compreensíveis; estados de falha/pendência claros. |
| R32 | Compatibilidade | Migrações e estabilidade de contratos/formatos | Plano §§11,16 | S02, S13, S40, S43, S58, S63, S66, S67, S68, S70, S71, S72 | Matriz de versões e migrações sem perda; export legível independente do app. |
| R33 | Open source | Projeto não comercial com licença open source e conformidade | Orientação do autor | S01, S03, S04, S12, S27, S34, S38, S39, S42, S47, S56, S57, S59, S60, S63, S69, S70, S72 | Licença ratificada, direitos upstream preservados e integrações pagas opcionais explicadas. |
| R34 | Qualidade | Testes, evidências e revisão humana de código produzido com IA | Estratégia solo + agentes | S01, S03, S04, S05, S06, S09, S11, S12, S15, S27, S34, S37, S53, S54, S55, S59, S60, S61, S63, S68, S72 | Nenhum gate fechado apenas por relato do agente; execução/resultado e revisão registrados. |
| R35 | Extensibilidade | Contratos/kit de provedores e extensões controladas | Proposta pós-v1.0 | S67, S68, S69, S70, S71, S72 | Requer ratificação; extensão não amplia permissões nem introduz leitura/escrita irrestrita. |
| R36 | Escala | Acervos maiores e pacotes offline portáveis | Proposta pós-v1.0 | S61, S64, S65, S66, S72 | Metas aprovadas após baseline; sem reduzir fidelidade, escopo ou budgets silenciosamente. |
| R37 | Auditoria | Rastreabilidade criptográfica com limites explícitos | RFC §§1,11 | S02, S08, S21, S58, S62 | Encadeamento/checagem de eventos e limites de adulteração/ancoragem documentados. |

A aba **Requisitos** lista os sprints que implementam/verificam cada item. A cobertura demonstra que há trabalho planejado, não que os requisitos já foram atendidos.

## I. Decisões de arquitetura pendentes

| ID | Decisão | Sprint | Regra |
| --- | --- | --- | --- |
| ADR-001 | Rust estrito versus núcleo Rust + workers | S01 | Aprovar composição poliglota controlada; se rejeitada, replanejar ports antes de avançar. |
| ADR-002 | Autoridade JSON/journal e MPK derivado | S02 | Escolher precedência, fronteira de confirmação e formato aberto de histórico. |
| ADR-003 | Licença do código próprio e recorte upstream | S03 | Ratificar licença; examinar por origem/arquivo, incluindo SearxNG e BlockSuite. |
| ADR-004 | Política Jev, BYOK e modo offline | S01 | Jev de primeira classe quando autorizado; fallback local/pendência sem equivalente probabilístico presumido. |
| ADR-005 | Perfil Markdown e representação de IDs | S02 | Fixar blocos suportados, campos preservados e extensões de proveniência. |
| ADR-006 | OS de referência e matriz de distribuição | S01 | Escolher host principal sem reduzir os cinco alvos finais; obter acesso aos ambientes de teste. |
| ADR-007 | Budgets e perfis de hardware | S05 | Fixar método de medição e perfis; OCR/STT pesado não recebe teto maior implicitamente. |
| ADR-008 | Recorte BlockSuite e integração DOM/PixiJS | S12 | Fechar grafo de pacotes e licença/revisão; manter PixiJS como cena principal. |
| ADR-009 | Protocolos de workers e políticas por SO | S23 | Fixar IPC, capabilities e fail-closed; tratar GPU/browser separadamente. |
| ADR-010 | Runtime LocalAI e backends/modelos permitidos | S31 | Selecionar pacotes e desabilitar superfícies concorrentes de agentes/RAG/tools. |
| ADR-011 | Modelos de resumo, embeddings e transcrição | S32 | Homologar por função/idioma/hardware; licença e hashes completos. |
| ADR-012 | Taxonomia, thresholds e abstenção | S35 | Aprovar política calibrada; retirar merges silenciosos por distância/similaridade. |
| ADR-013 | Backend de busca inicial | S40 | Escolher Qdrant ou HelixDB por evidência; não distribuir ambos por hábito. |
| ADR-014 | Recorte PageIndex e contrato de Context Engine | S42 | Definir upstream mínimo e fronteiras substituídas; árvore é projeção do grafo. |
| ADR-015 | Instância SearxNG e normalização web | S47 | Aprovar execução local/instância pessoal e política de consultas; sem instância pública implícita. |
| ADR-016 | Git, retenção e evidências criptográficas | S58 | Aprovar dados incluídos/excluídos, purga, assinatura/ancoragem e alcance de rollback. |
| ADR-017 | Escopo pós-v1.0 | S61 | Ratificar evolução sem bloquear correções; distinguir obrigatório, experimental e não escopo. |
| ADR-018 | SDK/extensões da v2.0 | S67 | Aprovar superfície restrita, distribuição e permissões; não abrir execução arbitrária de plugins. |

## J. Riscos e respostas previstas

| ID | Risco | Severidade | Onde | Resposta |
| --- | --- | --- | --- | --- |
| K01 | Incompatibilidade do requisito Rust estrito com os motores escolhidos | Crítico | S01 | ADR-001 aprovada antes de implementar; rejeição implica replanejar ports, não fingir compatibilidade. |
| K02 | Perda autoral por dois stores ou dual-write arquivo/banco | Crítico | S02; S07–S11 | Journal/checkpoints, escritor único, expected_revision e destruição de caches como gate. |
| K03 | BlockSuite não representa todo o perfil Markdown sem perda | Alto | S12–S15 | Golden round-trip; limitar blocos ou extensão aberta; não converter silenciosamente em texto plano. |
| K04 | Recorte PageIndex reintroduz IDs instáveis ou acesso global | Crítico | S42–S46 | Portas de dados/modelos, grafo canônico, corpus contrastivo e testes negativos por ferramenta. |
| K05 | Qdrant e HelixDB não satisfazem requisitos de distribuição/recuperação | Alto | S37–S40 | Spikes comparáveis; decisão humana; se ambos falham, interromper integração e revisar ADR. |
| K06 | Jev obrigatório inviabiliza offline ou envia dados sem permissão | Crítico | S01; S34–S35 | Integração nativa autorizada, fallback distinto/pendência e consentimento antes de qualquer chamada. |
| K07 | Parsers/browser expõem host a conteúdo malformado | Crítico | S23–S30; S48–S49 | Perfis de SO reais, inputs mínimos, quotas, broker e capacidades desabilitadas onde não confinadas. |
| K08 | Sandbox de um SO é tratada como equivalente à de todos | Crítico | S24–S26; S56–S57 | Testes nativos; matriz de capacidades; AppContainer/ACLs além de Jobs; macOS com mecanismo auditado. |
| K09 | LocalAI/OCR/browser impedem atingir budgets | Alto | S31–S32; S49; S55 | Lazy start, unload, backpressure e medição de processos; revisão explícita, nunca esconder consumo. |
| K10 | Falta de hardware/assinatura/runner para cinco alvos | Alto | S01; S05; S56–S57 | Inventariar acesso e custos; sem teste real não publicar plataforma como homologada. |
| K11 | Atualização upstream torna o fork difícil de manter | Alto | S03; S12; S42; S63; S70 | Facades estreitas, patches pequenos, lockfiles e comparação diferencial. |
| K12 | Geração fluente esconde erro factual ou citação inventada | Alto | S33; S46; S51 | Validação de referências, corpus reservado, cobertura declarada, abstenção e revisão humana. |
| K13 | Escopo e backlog excedem capacidade de um mantenedor | Alto | Todos | WIP humano 1; releases úteis cedo; sprints podem ser subdivididos; v2 proposta ratificada antes de executar. |
| K14 | Agente declara testes verdes sem executá-los | Crítico | S04–S05; todos os gates | Evidência vinculada a commit/ambiente; revisor humano; resultados não executados ficam pendentes. |
| K15 | Licenças/pesos ou API são considerados gratuitos por ser nonprofit | Alto | S03; S32; S47; releases | SBOM, direitos por artefato e orçamento/consentimento; licença do código não cobre automaticamente modelos. |
| K16 | Nova extensão v2 amplia privilégios ou acopla dados ao provedor | Crítico | S67–S72 | SDK restrito, capabilities, revogação, sandbox e prova File-as-Truth com extensões removidas. |
| K17 | Mudança de embedding mistura espaços vetoriais | Alto | S32; S37; S40–S41; S63 | Manifesto por índice; rebuild/swap de geração e comparação de qualidade. |
| K18 | Migração ou purga apaga histórico necessário | Crítico | S08; S58; S62; S71 | Preview, checkpoint, falhas injetadas, retenção explícita e restore testado. |

## K. Critérios de saída por marco principal

### v0.4 — primeiro produto de autoria útil

- Vault, notas, Mesa e Peça funcionam sem IA/rede.
- Fork-on-insert mantém origem e independência editorial.
- Editor/canvas podem perder seus caches sem perda de autoria.
- Falhas e conflitos não são escondidos por autosave.

### v0.9 — ciclo analítico integrado

- Ingest, pesquisa, classificação, summaries, busca e contexto estrutural se conectam às fases.
- Cada motor respeita o contrato e não mantém autoridade paralela.
- Fontes lidas são distinguidas de snippets e inferências.
- Persistem lacunas de homologação que impedem chamar a alpha de estável.

### v1.0 — escopo original estável

- Requisitos essenciais originais revisados/ratificados possuem evidências.
- Cinco alvos de desktop e perfis publicados realmente testados.
- Segurança, recuperação, performance e migrações possuem go/no-go humano.
- Modo offline, egress consentido, licenças e distribuição estão documentados.
- Nenhum blocker de integridade/segurança é transferido à v2 para fechar o marco.

### v2.0 — evolução aprovada e sustentabilidade

- Dados e formatos continuam independentes dos provedores.
- SDK/extensões, se ratificados, operam com capacidades restritas e conformidade testável.
- Upgrade/rollback/export de v1.x e pacotes offline foram revalidados.
- Upstreams, patches, SBOM e governança permitem continuidade comunitária.
- Existe política pós-v2; o roadmap termina, não a manutenção do software.

## L. Ordem prática para começar amanhã

1. Abrir a aba **Tarefas**, filtrar **S01** e trabalhar em `SL-S01-01`.
2. Concluir as decisões bloqueadoras antes de mandar um agente clonar todos os repositórios.
3. S02 define formatos; S03 define o que pode ser reutilizado e sob quais condições.
4. S04–S06 criam a bancada, não um protótipo que ignora persistência/segurança.
5. S07–S11 constroem a autoridade dos dados; só depois o editor começa a gravar nela.
6. Manter WIP baixo e registrar evidências desde o primeiro gate.
7. Se uma premissa falhar, reabrir a ADR e o trecho afetado; não compensar com um segundo store ou serviço remoto oculto.

## M. Manutenção destes artefatos

Os dados estruturados e scripts de geração ficam em `planejamento/_geracao/`. O gerador preserva campos de acompanhamento existentes da planilha por ID antes de recriar os artefatos. Faça backup antes de regenerar; novas tarefas exigem atualizar dependências e validar o grafo. Não edite o CSV como fonte concorrente de status se a planilha é o acompanhamento principal.

```text
python planejamento/_geracao/render_plan.py
```

Essa ferramenta gera documentos de planejamento. Ela não cria, compila ou testa o aplicativo Sandland. Caminhos de evidências de implementação só existirão conforme o trabalho futuro for executado.

**Referências de base:** RFC/PRD original em `uploads/Documento de Arquitetura Técnica e Especificação de Produto.md`; plano complementar `Sandland-RFC-Integracao-e-Desconstrucao.md`; repositórios oficiais listados na trilha de motores. Revisões e termos upstream devem ser conferidos no início de cada spike e update.
