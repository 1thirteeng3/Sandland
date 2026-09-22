ENGINES = [
 ('BlockSuite','S12–S15; S18; S20–S21; S63; S70','Bibliotecas + facade; patch set mínimo','store/std/rich-text, modelos/blocos necessários; confirmar grafo workspace na revisão fixada','UI AFFiNE, cloud e segundo canvas','Codec Markdown, IDs, salvamento, overlay PixiJS e proveniência','https://github.com/toeverything/AFFiNE/tree/canary/blocksuite'),
 ('PageIndex','S42–S46; S63; S70','Recorte/fork delimitado + portas de modelo/dados','page_index_md, navegação/resumos/tools úteis; confirmar módulos reais no checkout','DocStore como autoridade, páginas de PDF artificiais, acesso global e cloud implícita','Snapshot estrutural, IDs estáveis, relações cruzadas, budgets, invalidação e ToolBroker','https://github.com/VectifyAI/PageIndex'),
 ('Qdrant','S37–S38; S40–S41 se escolhido','Spike; depois adapter sem extrair o storage interno','Edge ou Server conforme capacidades testadas','Segundo banco permanente se não for escolhido; UI/cluster sem necessidade','SearchProvider, fusão/escopo, revisões e reconstrução','https://qdrant.tech/documentation/edge/'),
 ('HelixDB','S37; S39; S40–S41 se escolhido','Spike; depois adapter embutido/local conforme evidência','Engine/SDK e consultas requeridas de texto, vetores e relações derivadas','Fonte da verdade e object storage remoto obrigatório','SearchProvider, projeções da topologia e ciclo de índices','https://docs.helix-db.com/database/helix-db/start-here/local-development/embedded-database'),
 ('Jev','S34–S35; S52; S63','Adapter de serviço + contrato de decisões','Choice, Score e Noul da API, com versão registrada','Tentativa de extrair pesos fechados, executar Jev no LocalAI ou gerar texto livre','Taxonomia, candidatos, calibração, consentimento, fallback e revisão','https://docs.typesafe.ai/introduction'),
 ('Docling','S27–S28; S30; S63; S70','Worker Python empacotado; adapter mínimo','Parsing textual, tabelas, OCR aprovado e mapas de fonte','Servidor sempre residente, todos os modelos e acesso amplo ao Vault','NormalizedDocument, jobs, persistência, limites e reprocessamento','https://github.com/docling-project/docling'),
 ('Readability / Turndown','S29–S30; S48; S63; S70','Bibliotecas JS em ambiente DOM isolado','Extração de artigo e conversão HTML para Markdown','Scripts/trackers remotos e janela principal como parser hostil','Sanitização, assets, origem, workers e normalização canônica','https://github.com/mozilla/readability'),
 ('SearxNG','S47; S50; S63; S70','Serviço local/pessoal configurado + adapter','Metabusca e API JSON dos engines escolhidos','Frontend próprio, instância pública implícita, alegação de busca offline','Consulta/egress, UI, histórico, seleção, lifecycle e distribuição AGPL','https://github.com/searxng/searxng'),
 ('Scrapling','S48–S50; S63; S70','Worker HTTP e browser separado sob demanda','Fetchers, parser/sessões e aquisição dinâmica quando aprovada','Crawl irrestrito, um browser por widget e bypass universal prometido','Política SSRF/rede, snapshots, pool, limits e promoção à Mesa','https://github.com/D4Vinci/Scrapling'),
 ('LocalAI','S31–S32; S36; S63; S65; S70','Runtime gerenciado, não reescrita de kernels','APIs e backends homologados, carga/descarga e mecanismos de recursos','Agentes, shell, memória/RAG paralelos e downloads automáticos arbitrários','Catálogo, gateway, consentimento, pacotes, supervisor e diagnóstico','https://github.com/mudler/LocalAI'),
 ('PixiJS / Tauri / SQLite','S04–S11; S16–S19; S54–S57','Infraestrutura reutilizada com domínio próprio','Janela/IPC, renderização e projeções SQL','Autoridade de dados nas caches e permissões implícitas','Vault, board, interação, conflitos, política de segurança e lifecycle','Sandland-RFC-Integracao-e-Desconstrucao.md'),
]

RISKS = [
 ('K01','Incompatibilidade do requisito Rust estrito com os motores escolhidos','Crítico','S01','ADR-001 aprovada antes de implementar; rejeição implica replanejar ports, não fingir compatibilidade.'),
 ('K02','Perda autoral por dois stores ou dual-write arquivo/banco','Crítico','S02; S07–S11','Journal/checkpoints, escritor único, expected_revision e destruição de caches como gate.'),
 ('K03','BlockSuite não representa todo o perfil Markdown sem perda','Alto','S12–S15','Golden round-trip; limitar blocos ou extensão aberta; não converter silenciosamente em texto plano.'),
 ('K04','Recorte PageIndex reintroduz IDs instáveis ou acesso global','Crítico','S42–S46','Portas de dados/modelos, grafo canônico, corpus contrastivo e testes negativos por ferramenta.'),
 ('K05','Qdrant e HelixDB não satisfazem requisitos de distribuição/recuperação','Alto','S37–S40','Spikes comparáveis; decisão humana; se ambos falham, interromper integração e revisar ADR.'),
 ('K06','Jev obrigatório inviabiliza offline ou envia dados sem permissão','Crítico','S01; S34–S35','Integração nativa autorizada, fallback distinto/pendência e consentimento antes de qualquer chamada.'),
 ('K07','Parsers/browser expõem host a conteúdo malformado','Crítico','S23–S30; S48–S49','Perfis de SO reais, inputs mínimos, quotas, broker e capacidades desabilitadas onde não confinadas.'),
 ('K08','Sandbox de um SO é tratada como equivalente à de todos','Crítico','S24–S26; S56–S57','Testes nativos; matriz de capacidades; AppContainer/ACLs além de Jobs; macOS com mecanismo auditado.'),
 ('K09','LocalAI/OCR/browser impedem atingir budgets','Alto','S31–S32; S49; S55','Lazy start, unload, backpressure e medição de processos; revisão explícita, nunca esconder consumo.'),
 ('K10','Falta de hardware/assinatura/runner para cinco alvos','Alto','S01; S05; S56–S57','Inventariar acesso e custos; sem teste real não publicar plataforma como homologada.'),
 ('K11','Atualização upstream torna o fork difícil de manter','Alto','S03; S12; S42; S63; S70','Facades estreitas, patches pequenos, lockfiles e comparação diferencial.'),
 ('K12','Geração fluente esconde erro factual ou citação inventada','Alto','S33; S46; S51','Validação de referências, corpus reservado, cobertura declarada, abstenção e revisão humana.'),
 ('K13','Escopo e backlog excedem capacidade de um mantenedor','Alto','Todos','WIP humano 1; releases úteis cedo; sprints podem ser subdivididos; v2 proposta ratificada antes de executar.'),
 ('K14','Agente declara testes verdes sem executá-los','Crítico','S04–S05; todos os gates','Evidência vinculada a commit/ambiente; revisor humano; resultados não executados ficam pendentes.'),
 ('K15','Licenças/pesos ou API são considerados gratuitos por ser nonprofit','Alto','S03; S32; S47; releases','SBOM, direitos por artefato e orçamento/consentimento; licença do código não cobre automaticamente modelos.'),
 ('K16','Nova extensão v2 amplia privilégios ou acopla dados ao provedor','Crítico','S67–S72','SDK restrito, capabilities, revogação, sandbox e prova File-as-Truth com extensões removidas.'),
 ('K17','Mudança de embedding mistura espaços vetoriais','Alto','S32; S37; S40–S41; S63','Manifesto por índice; rebuild/swap de geração e comparação de qualidade.'),
 ('K18','Migração ou purga apaga histórico necessário','Crítico','S08; S58; S62; S71','Preview, checkpoint, falhas injetadas, retenção explícita e restore testado.'),
]

OPENING = r'''# Sandland — Roadmap granular e plano de implementação até a v2.0

**Data de elaboração:** 22/09/2026  
**Modelo de execução escolhido:** uma pessoa responsável + agentes de IA  
**Cadência escolhida:** sprints encerrados por critérios de aceite, sem duração fixa  
**Horizonte escolhido:** produto completo na v1.0, maturação e evolução até a v2.0  
**Natureza:** projeto open source conduzido sem fins comerciais  
**Estado inicial:** planejamento; nenhuma tarefa de implementação está declarada concluída

> **Dimensão do plano:** {{sprints}} sprints, {{tasks}} tarefas detalhadas, {{gates}} gates de aceite e {{versions}} marcos de versão. São {{items}} itens rastreáveis no backlog. Esses números representam unidades de trabalho, não semanas, dias ou uma previsão de prazo.

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

{{engines_table}}

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

{{releases_table}}

**Sem datas de conclusão:** o plano é ordenado por dependências e aceite. Beta/RC têm gates próprios, em vez de nomes promocionais para o mesmo artefato.

## F. Navegação dos sprints

{{sprint_links}}

---

## G. Plano operacional sprint a sprint

Cada item abaixo tem ID estável, passo de execução, artefato e aceite. “Dependências” são referências de trabalho, não bibliotecas de runtime. `SL-Sxx-GATE` exige conclusão/evidência das tarefas do sprint; o backlog inclui esses gates como linhas separadas.

'''

CLOSING = r'''
---

## H. Requisitos e rastreabilidade

{{requirements_table}}

A aba **Requisitos** lista os sprints que implementam/verificam cada item. A cobertura demonstra que há trabalho planejado, não que os requisitos já foram atendidos.

## I. Decisões de arquitetura pendentes

{{adrs_table}}

## J. Riscos e respostas previstas

{{risks_table}}

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
'''
