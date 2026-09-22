from plan_base import add

add(37,'Definir contrato e bancada de busca híbrida','SearchService / avaliação','Próprio',['R02','R20','R21','R34'],['S05','S10','S22','S32','S36'],
'Comparar motores sob o mesmo contrato antes de escolhê-los.','Contrato de busca, indexação e protocolo comparativo.','Mesma fixture/consulta roda em providers fake e produz ranking/evidências verificáveis.',r'''
[Contrato] Definir SearchDocument :: Identificar item, tipo, escopo, revisão, bloco, localizador e estado editorial. | schemas/search-document-v0.json | Peça derivada e fonte externa podem ser filtradas separadamente.
[Contrato] Definir SearchQuery/Hit :: Incluir filtros, top-k, budget, score/origem e referência de evidência. | schemas/search-query-v0.json | Resultado não entrega path arbitrário para ferramenta abrir.
[Implementação] Criar interface SearchProvider :: Delimitar upsert, delete, query, health, rebuild e capabilities. | crates/search/src/provider.rs | Domínio compila contra interface sem SDK privado de Qdrant/Helix espalhado.
[Implementação] Definir unidades de indexação :: Usar seções/blocos com vínculo ao item; resumo é representação adicional. | crates/search/src/units.rs | Recuperação não depende só do resumo de 2–5 frases nem trata canvas como chunks cegos.
[Implementação] Integrar embeddings versionados :: Associar manifesto e rejeitar dimensão/espaço divergente. | crates/search/src/vector_manifest.rs | Troca de modelo exige reindexação ou coleção distinta, sem mesclar vetores.
[Implementação] Construir baseline léxico :: Usar FTS5 ou baseline equivalente para comparar ganhos reais. | experiments/search/lexical-baseline/ | Busca semântica não é declarada superior sem comparação com léxico.
[Teste] Fixar protocolo de avaliação :: Definir recall/ranking, filtros, exclusão, latência, memória e cold start. | experiments/search/protocol.md | Métricas, corpus reservado e método são idênticos para os dois candidatos.
[Teste] Criar suíte de conformidade :: Testar IDs, revision update, tombstone, filtros e reconstrução. | tests/contracts/search-provider/ | Provider fake malicioso falha ao retornar item de outro escopo ou revisão obsoleta.
[Teste] Preparar escalas de corpus :: Medir conjuntos pequeno/médio/grande escolhidos antes dos resultados. | experiments/search/datasets.yaml | Tamanho/semântica não mudam para favorecer um candidato depois da medição.
[Decisão] Fixar regra de escolha :: Priorizar correção, distribuição/licença, recursos e depois desempenho. | experiments/search/decision-rubric.md | Empate ou inviabilidade possui regra de replanejamento, não escolha por preferência não registrada.
''')

add(38,'Avaliar Qdrant sem incorporar servidor completo','Qdrant / candidato A','Spike de motor',['R01','R20','R21','R30','R33'],['S37'],
'Testar Edge/Server adequados ao desktop com recorte experimental descartável.','Relatório Qdrant e provider experimental.','Executar conformidade e benchmarks iguais ao candidato B, com conclusão viável/inviável.',r'''
[Upstream] Fixar revisão/artefato Qdrant :: Auditar Edge e Server, APIs disponíveis, licença e dependências. | experiments/search/qdrant/manifest.json | Não se presume paridade Edge/Server ou licença apenas pela marca.
[Spike] Escolher modo de prova :: Comparar exigências embutido versus processo local para o conjunto requerido. | experiments/search/qdrant/mode.md | Motivo do modo e recursos indisponíveis ficam explícitos.
[Implementação] Montar harness mínimo :: Abrir engine/serviço em diretório temporário e expor contrato experimental. | experiments/search/qdrant/adapter/ | Nenhuma UI do produto depende deste candidato antes da decisão.
[Teste] Exercitar CRUD e revisões :: Inserir, atualizar, excluir e reiniciar com IDs do contrato. | reports/search/qdrant/crud.json | Evidência registra resultado de cada caso; falha funcional torna candidato rejeitado, não teste omitido.
[Teste] Exercitar filtros de escopo :: Consultar tipos, workspaces, revisões e estados editoriais. | reports/search/qdrant/filters.json | Não há hits fora do escopo nos casos válidos; incompatibilidade é conclusiva.
[Teste] Exercitar caminho híbrido :: Combinar dense/sparse ou léxico externo conforme capabilities reais. | reports/search/qdrant/hybrid.json | Fusões e normalizações são documentadas e comparáveis ao baseline.
[Teste] Medir busca/indexação :: Executar protocolo reservado e guardar resultados brutos. | reports/search/qdrant/performance.json | Memória/latência incluem modo de execução usado, não números promocionais.
[Teste] Exercitar falha/rebuild :: Interromper índice, apagar cache e reconstruir do corpus canônico. | reports/search/qdrant/recovery.json | Dados autorais não dependem do candidato; recuperação e limitações são observadas.
[Spike] Verificar distribuição :: Testar empacotamento e dependências nos alvos prioritários e mapear pendências dos demais. | reports/search/qdrant/packaging.md | Suporte não observado permanece pendente; cenário experimental não vira homologação.
[Decisão] Consolidar conclusão A :: Marcar capabilities aprovadas, rejeitadas e custos de adaptação. | reports/search/qdrant/conclusion.md | Relatório pode concluir inviabilidade; deve completar o protocolo em vez de mascarar falhas.
''',note='Este sprint fecha uma investigação, não aprova automaticamente Qdrant para produção. Conclusão negativa fundamentada é uma saída válida do spike.')

add(39,'Avaliar HelixDB sob o mesmo contrato','HelixDB / candidato B','Spike de motor',['R01','R20','R21','R30','R33'],['S37'],
'Medir integração embutida e busca/grafo sem tornar HelixDB fonte da verdade.','Relatório HelixDB e provider experimental.','Rodar o mesmo corpus e confrontar resultados com o candidato A.',r'''
[Upstream] Fixar revisão HelixDB :: Auditar SDK/engine embutido, licença e requisitos de storage/cache. | experiments/search/helix/manifest.json | Modo local/cloud/embedded e geração de API não são misturados.
[Spike] Escolher storage de prova :: Usar disco local e perfil explícito de caches sem exigir object storage remoto. | experiments/search/helix/mode.md | Modo atende independência de nuvem ou é classificado incompatível.
[Implementação] Montar harness mínimo :: Traduzir contrato SearchProvider para SDK sem acoplamento da UI. | experiments/search/helix/adapter/ | Código é experimental e não instala um segundo backend de produção.
[Teste] Exercitar CRUD e revisão :: Repetir inserção, exclusão, atualização e reabertura do protocolo. | reports/search/helix/crud.json | Mesmos critérios e IDs do candidato A; falhas preservadas no relatório.
[Teste] Exercitar filtros e BM25 :: Verificar busca textual/vetorial e isolamento de escopos. | reports/search/helix/hybrid.json | Operação integrada não mistura conteúdo de workspace não autorizado.
[Teste] Exercitar relações derivadas :: Projetar pequeno grafo canônico e executar consultas úteis ao produto. | reports/search/helix/graph.json | Ganho relacional demonstrado por caso real; grafo no banco não substitui topologia em arquivos.
[Teste] Medir recursos e consultas :: Aplicar mesmo corpus, warm/cold, filtros e hardware da prova A. | reports/search/helix/performance.json | Caches limitados não são apresentados como limite rígido de RSS total.
[Teste] Exercitar falha/reconstrução :: Apagar ou interromper índice e reconstruir a projeção. | reports/search/helix/recovery.json | Nenhuma autoria só pode ser recuperada do banco.
[Spike] Verificar distribuição :: Inspecionar SDK/runtime por alvo e dependências nativas. | reports/search/helix/packaging.md | Lacunas de plataforma são fator da decisão, não escondidas como detalhe futuro.
[Decisão] Consolidar conclusão B :: Registrar viabilidade, lacunas e custo incremental versus Qdrant. | reports/search/helix/conclusion.md | Protocolo termina com evidências comparáveis mesmo quando motor é rejeitado.
''',note='Concluir o spike não implica adotar HelixDB. Os dois protótipos ficam em experiments e não são distribuídos juntos por padrão.')

add(40,'Escolher e integrar um backend de busca','Qdrant OU HelixDB / produção','Integração',['R02','R20','R21','R30','R32'],['S38','S39'],
'Converter evidência comparativa em uma implementação de produto, não em dual-stack.','SearchProvider de produção e ADR de seleção.','Executar busca híbrida no app com apenas o backend escolhido instalado.',r'''
[Decisão] Aprovar seleção :: Confrontar rubrica e relatórios; resolver ADR-013 com veto para requisitos essenciais não atendidos. | docs/adr/013-search-backend.md | Um candidato escolhido explicitamente; se ambos falham, sprint bloqueia e plano é revisto.
[Implementação] Promover adapter selecionado :: Mover somente código necessário do experimento à camada de produção. | adapters/search-selected/ | Segundo candidato não é dependência de runtime nem inicia processos.
[Implementação] Integrar índice e manifest :: Versionar schema, engine, embeddings e configuração de armazenamento. | crates/search/src/index_manifest.rs | Índice incompatível é reconstruído/isolado e não lido como se fosse atual.
[Implementação] Integrar backend léxico :: Manter FTS5 ou adotar BM25 do motor conforme ADR, evitando duplicação injustificada. | crates/search/src/lexical.rs | Caminho léxico funciona sem gerar embedding e possui contrato equivalente.
[Implementação] Implementar fusão RRF :: Combinar rankings no coordenador quando motores forem separados. | crates/search/src/fusion.rs | Fórmula/testes não dependem da antiga query SQL única com sqlite-vec.
[Implementação] Aplicar autorização a hits :: Filtrar/validar escopo e revisão antes de expor conteúdo. | crates/search/src/authorization.rs | Resultado inesperado do backend não contorna o ToolBroker.
[Implementação] Integrar geração de consultas vetoriais :: Usar mesmo espaço aprovado e fallback léxico em indisponibilidade. | crates/search/src/query_vectors.rs | Modelo descarregado pode causar latência explícita, não envio externo automático.
[Implementação] Conectar lifecycle e métricas :: Abrir/fechar engine, gerenciar recursos e expor diagnóstico. | crates/search/src/lifecycle.rs | Engine não vira processo órfão e modo offline é observado.
[Teste] Executar conformidade de produção :: Rodar suite comum no adapter promovido, não apenas no protótipo. | reports/search/production-contract.json | Todos os requisitos essenciais aprovados na decisão continuam satisfeitos após integração.
[Licença] Remover dependências não escolhidas :: Atualizar SBOM, pacotes e documentação de instalação. | compliance/search-runtime.json | Pacote final contém somente o caminho adotado e suas licenças reais.
''')

add(41,'Entregar busca incremental no acervo e nas Peças','SearchService / UI','Integração',['R02','R05','R13','R20','R21','R31'],['S35','S40'],
'Fazer recuperação refletir revisões, exclusões e diferentes tipos de autoria.','v0.7.0 com busca híbrida e reconstrução assistida.','Editar/excluir material e verificar atualização da busca sem perder as fontes canônicas.',r'''
[Implementação] Conectar eventos à indexação :: Consumir captura, extração, edição da Peça e tombstones por revisão. | crates/search/src/index_jobs.rs | Evento repetido não duplica unidades e edição antiga não substitui a mais nova.
[Implementação] Fazer reindexação incremental :: Reprocessar apenas unidades afetadas e propagar mudanças de manifesto. | crates/search/src/incremental.rs | Alteração de layout não dispara indexação textual novamente; troca de embedding inicia rebuild compatível.
[Implementação] Publicar rebuild atomicamente :: Construir nova geração e trocar quando validada. | crates/search/src/rebuild.rs | Interrupção mantém busca anterior ou degradação explícita, não índice parcialmente anunciado pronto.
[Implementação] Expor progresso e saúde :: Mostrar pendente, desatualizado, reconstruindo e falha por tipo de índice. | apps/desktop/src/search/health.ts | Usuário não confunde ausência de resultado com material não indexado.
[Implementação] Criar UI de busca híbrida :: Incluir filtros de tipo, workspace, origem e estado editorial. | apps/desktop/src/search/ | Peças próprias podem ser excluídas quando se busca evidência externa.
[Implementação] Navegar ao trecho :: Resolver hit por referência/revisão no leitor e na Peça. | apps/desktop/src/search/open-hit.ts | Hit antigo é detectado e não abre o trecho errado da revisão atual.
[Implementação] Integrar taxonomia ao provider :: Substituir lookup experimental de candidatos quando útil sem trocar política de decisão. | crates/taxonomy/src/search_adapter.rs | Modelo/índice de termos permanece identificado e não se mistura com embeddings de outro espaço.
[Teste] Validar relevância no corpus reservado :: Medir léxico, semântico e híbrido com rótulos estáveis. | reports/v0.7/relevance.json | Ganho/perda por modalidade é registrado, sem declarar vencedor universal.
[Teste] Executar exclusão e isolamento :: Remover material, editar Peça e consultar em outro workspace. | tests/search/end-to-end/ | Nenhum conteúdo excluído/desautorizado reaparece por cache ou job atrasado.
[Release] Publicar alpha de recuperação :: Documentar backend escolhido, limites e comando de reconstrução. | release/v0.7.0/ | Apagar índices vetoriais/léxicos não perde materiais ou texto editorial.
''')

add(42,'Auditar e recortar PageIndex para células','PageIndex / upstream','Desconstrução',['R03','R22','R23','R30','R33'],['S03','S23','S32','S41'],
'Identificar código reaproveitável e pontos que precisam perder acoplamento documental/cloud.','Pacote experimental de contexto e ADR do recorte PageIndex.','Construir índice de Markdown de fixture sem PDF e sem store canônico do SDK.',r'''
[Upstream] Fixar revisão PageIndex :: Registrar licença, dependências Python, APIs públicas e módulos internos usados. | upstream/pageindex/manifest.json | Paths e versões são verificados na revisão escolhida, não assumidos a partir de documentação móvel.
[Upstream] Reproduzir baseline Markdown :: Exercitar page_index_md ou equivalente real com fixture e modelo controlado. | reports/pageindex/upstream-baseline.md | Entrada/saída e comportamento de IDs/headings estão documentados.
[Upstream] Mapear acoplamentos :: Identificar store, nomes de documento, páginas, provedores, prompts e frameworks de agentes. | docs/upstream/pageindex-couplings.md | Cloud e operações de gestão de biblioteca não entram implicitamente no adapter de células.
[Upstream] Delimitar módulos úteis :: Selecionar construção/navegação/resumos e utilitários necessários por grafo de dependências. | upstream/pageindex/slice.json | Não se copia função isolada com dependências wildcard desconhecidas.
[Implementação] Criar fachada ContextProvider :: Expor contratos Sandland sem vazar nomes de documento/página como identidade. | adapters/pageindex/ | App não depende de DocStore nem de IDs sequenciais internos para autoria.
[Implementação] Criar porta de modelos :: Redirecionar chamadas de resumo/navegação ao ModelGateway autorizado. | adapters/pageindex/model_port.py | Configuração padrão não envia células a provedor remoto do upstream.
[Implementação] Criar porta de evidências :: Intermediar toda leitura pelo broker e pelas referências autorizadas. | adapters/pageindex/evidence_port.py | Prompt de contexto não é a única barreira de acesso à biblioteca.
[Implementação] Criar patch set testável :: Isolar mudanças do núcleo e conservar avisos/origem. | upstream/pageindex/patches/ | Diferenças são reproduzíveis e possuem fixtures; não há fork integral sem necessidade.
[Decisão] Ratificar ADR de contexto :: Resolver ADR-014, manter árvore como projeção e escolher responsabilidades do adapter. | docs/adr/014-context-engine.md | Grafo completo continua canônico fora da árvore; nome RLM é definido sem ambiguidade.
[Teste] Validar isolamento inicial :: Executar contexto fake sem SDK cloud, filesystem geral ou dependências não necessárias. | reports/pageindex/isolation.json | Uso offline e perímetro de leitura são demonstrados antes de ligar a UI.
''')

add(43,'Projetar a topologia em índice estrutural estável','PageIndex / StructureAdapter','Adaptação',['R02','R04','R11','R22','R32'],['S17','S42'],
'Transformar workspace/células em hierarquia navegável sem perder o grafo.','WorkspaceSnapshot e índice estrutural por revisão.','Renomear notas, mover cartões e reconstruir mantendo IDs e referências.',r'''
[Contrato] Definir WorkspaceSnapshot :: Combinar hierarquia, relações, layout, fontes e revisões com schema explícito. | schemas/workspace-snapshot-v0.json | Snapshot representa ciclos/arestas cruzadas fora da árvore de navegação.
[Implementação] Construir snapshot coerente :: Ler revisão consistente de board e conteúdos pelo Core. | crates/context/src/snapshot.rs | Não combina geometria nova com texto antigo sem registrar as revisões usadas.
[Implementação] Projetar grupos/células/blocos :: Usar estrutura existente do produto em vez de inferi-la novamente por LLM. | adapters/pageindex/structure_adapter.py | Célula não é convertida em PDF e grupo manual não é substituído por cluster inventado.
[Implementação] Mapear IDs estáveis :: Traduzir node_id interno para identidade Sandland com revision key. | adapters/pageindex/identity.py | Inserir um nó não renumera todas as evidências existentes.
[Implementação] Preservar referências cruzadas :: Guardar arestas tipadas/direção e ocorrências múltiplas em estrutura auxiliar. | crates/context/src/relations.rs | Tree thinning não elimina células/arestas autorais.
[Implementação] Representar geometria relevante :: Conservar bounds/grupos e distinguir posição observada de relação semântica. | crates/context/src/layout.rs | Proximidade não é convertida automaticamente em apoio causal ou factual.
[Implementação] Mapear blocos/fontes :: Associar folhas a conteúdo e evidências com localizadores corretos. | adapters/pageindex/source_mapping.py | Leitura de folha resolve a revisão exata e não uma página fictícia.
[Implementação] Persistir cache derivado :: Versionar schema, pipeline e hashes sem duplicar autoridade canônica. | crates/context/src/cache.rs | Apagar árvore/summary cache permite reconstrução dos arquivos.
[Teste] Testar casos topológicos :: Cobrir ciclos, grupos sobrepostos admitidos, notas repetidas e órfãos. | tests/context/structure/ | Dados não representáveis na árvore continuam recuperáveis pelas relações.
[Teste] Testar estabilidade incremental :: Reordenar/renomear/mover mantendo conjunto de evidências. | reports/context/identity-stability.json | Apenas revisões realmente afetadas mudam; nenhuma referência é redirecionada por índice posicional.
''')

add(44,'Criar ferramentas de contexto com escopo efetivo','ToolBroker / PageIndex','Adaptação',['R06','R08','R22','R23'],['S09','S26','S43'],
'Permitir exploração pelo modelo sem lhe entregar o Vault ou a biblioteca global.','Ferramentas de overview, grupo, célula, vizinhos e fonte.','Modelo recebe ID fora do escopo e o broker nega mesmo com instrução em documento.',r'''
[Contrato] Definir catálogo de ferramentas :: Especificar args, resultados, leitura, limites e erros de cada operação. | schemas/context-tools-v0.json | Nenhuma ferramenta aceita path arbitrário como substituto de EvidenceRef.
[Implementação] Implementar overview limitado :: Retornar mapa global resumido por orçamento e snapshot. | crates/context-tools/src/overview.rs | Board grande não injeta automaticamente todas as células no prompt.
[Implementação] Implementar leitura de grupo :: Paginar membros/relações e manter revisão coerente. | crates/context-tools/src/group.rs | Limites são impostos em código, não dependem de modelo obedecer instrução.
[Implementação] Implementar leitura de célula :: Permitir ranges/blocos e devolução de referência de origem. | crates/context-tools/src/cell.rs | Leitura recusa célula de outro workspace não autorizada.
[Implementação] Implementar vizinhos/arestas :: Filtrar relação, direção e profundidade; controlar ciclos. | crates/context-tools/src/neighbors.rs | Traversal termina no limite e não perde tipo apoia/contradiz.
[Implementação] Implementar leitura de fonte :: Resolver asset/revisão/trecho autorizado por broker. | crates/context-tools/src/source.rs | Modelo não transforma URL/filename em permissão de filesystem.
[Implementação] Validar capability por chamada :: Checar expiração, revisão e budget acumulado antes de executar. | crates/context-tools/src/authorize.rs | Reutilizar ferramenta com token de outra tarefa é negado.
[Implementação] Limitar resultados e logs :: Cortar por contrato com paginação explícita sem vazar corpo em logs. | crates/context-tools/src/limits.rs | Truncamento é sinalizado e não se confunde com evidência completa.
[Teste] Testar injeção de prompt :: Colocar instruções maliciosas em células, PDFs e resultados de ferramentas. | tests/context/tool-injection/ | Conteúdo não concede novas ferramentas, credenciais ou acesso a outro projeto.
[Teste] Auditar ferramentas de gestão :: Garantir ausência de remove/upload/shell e outros comandos não aprovados no agente. | reports/context/tool-surface.md | Só o catálogo read-only aprovado é exposto nesta fase.
''')

add(45,'Implementar summaries incrementais e Skeleton Map','ContextService / LocalAI','Próprio',['R09','R19','R22','R23'],['S33','S43','S44'],
'Usar síntese estrutural dentro do contexto dos modelos locais e com invalidação precisa.','Skeleton Map com budget e cache incremental de resumos.','Editar uma célula e recomputar apenas summaries dependentes, sem perder evidências.',r'''
[Contrato] Definir orçamento por camada :: Reservar tokens para sistema, pergunta, evidências e resposta. | docs/contracts/context-budget.md | Limite do modelo é verificado antes da chamada e não após erro de contexto.
[Implementação] Gerar resumos de célula :: Reutilizar resumo válido ou produzir síntese por conteúdo/revisão. | crates/context/src/cell_summary.rs | Resumo automático nunca substitui conteúdo nem comentário autoral editado.
[Implementação] Gerar resumos de grupo :: Agregar filhos com references e relações relevantes, preservando divergências. | crates/context/src/group_summary.rs | Grupo não apaga evidência contraditória apenas para reduzir texto.
[Implementação] Criar grafo de dependências :: Associar summaries a revisões de conteúdo, grupo, modelo e prompt. | crates/context/src/summary_dependencies.rs | Alteração local invalida somente ancestrais/dependências pertinentes.
[Implementação] Criar Skeleton Map :: Selecionar visão geral útil sem concatenar 100 palavras de cada nó. | crates/context/src/skeleton.rs | Cena de 1.000 nós respeita budget e oferece mecanismo de expansão.
[Implementação] Integrar scheduler de summaries :: Limitar concorrência e reagendar conforme hardware/pergunta ativa. | crates/context/src/summary_jobs.rs | Chamadas em massa do upstream não carregam múltiplos modelos sem controle.
[Implementação] Marcar cache desatualizado :: Expor revisão do contexto à UI e reconstruir antes de resposta crítica. | apps/desktop/src/context/status.ts | Usuário não recebe contexto antigo apresentado como versão atual sem indicação.
[Implementação] Evitar trabalho em mudança visual :: Distinguir posição, relação e corpo do texto na invalidação. | crates/context/src/invalidation.rs | Mover cartão não regenera resumo textual; mudar aresta atualiza mapa relacional.
[Teste] Medir custo e cobertura :: Comparar full-context viável com navegação progressiva em corpus fixo. | reports/context/budget-eval.json | Redução de tokens é medida e não alegada como 80% universal.
[Teste] Exercitar offline/OOM :: Testar interrupção no meio de resumo de grupo e reinício. | tests/context/summary-recovery/ | Cache parcial não vira resultado válido e a Mesa permanece editável.
''')

add(46,'Entregar recuperação estrutural e síntese progressiva','Context Engine / orquestração','Integração',['R07','R13','R22','R23','R27'],['S44','S45'],
'Fechar o motor da Mesa com evidências, abstenção e limites de execução.','v0.8.0: consulta estrutural local sobre células e fontes.','Perguntar sobre grupos que se contradizem e receber resposta apoiada nas revisões corretas.',r'''
[Implementação] Criar planejador de exploração :: Selecionar grupos a partir do Skeleton Map e pergunta autorizada. | crates/context-agent/src/planner.rs | Plano não é interpretado como autorização para ferramenta fora do catálogo.
[Implementação] Implementar expansão limitada :: Navegar árvore e relações com limites de hops/chamadas/tempo. | crates/context-agent/src/explorer.rs | Ciclo no grafo não causa execução sem fim nem consumo ilimitado.
[Implementação] Executar subconsultas :: Compartilhar snapshot e serializar inferência quando o perfil exigir. | crates/context-agent/src/subqueries.rs | Paralelismo lógico não implica carregar vários modelos em RAM.
[Implementação] Consolidar evidências :: Manter quotes/referências junto às micro-sínteses e detectar incompatibilidades. | crates/context-agent/src/evidence.rs | Resumos intermediários não são a única fonte disponível para a resposta.
[Implementação] Gerar resposta com referências :: Validar IDs/localizadores e separar inferência de afirmação citada. | crates/context-agent/src/answer.rs | Citação inexistente é rejeitada/sinalizada, não renderizada como prova válida.
[Implementação] Implementar abstenção :: Reconhecer evidência insuficiente, acesso negado e budget esgotado. | crates/context-agent/src/abstain.rs | Sistema não inventa resposta para ocultar ausência de fonte.
[Implementação] Integrar cancelamento e revisão :: Descartar/apresentar como antiga resposta construída sobre snapshot alterado. | crates/context-agent/src/session.rs | Resultado atrasado não modifica texto da Peça nem se anuncia como atual.
[Teste] Avaliar relações e layout :: Perguntas contrastivas com mesmas palavras, mas grupos/arestas diferentes. | tests/eval/context-topology/ | Motor usa estrutura relevante em vez de responder só por similaridade textual.
[Teste] Executar avaliação de citações :: Verificar fontes, revisões, ausência de resposta e leaks entre workspaces. | reports/v0.8/context-quality.json | Precisão de referência e escopo são gates, não somente fluência do texto.
[Release] Publicar alpha de raciocínio :: Entregar modo local, diagnóstico de contexto e limites conhecidos. | release/v0.8.0/ | PageIndex adaptado não assume autoridade canônica nem acesso global às células.
''')
