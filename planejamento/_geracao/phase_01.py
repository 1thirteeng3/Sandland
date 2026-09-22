from plan_base import add

add(1,'Ratificar escopo, soberania e composição','Arquitetura / produto','Próprio',['R01','R03','R07','R08','R33','R34'],[],
'Eliminar decisões contraditórias antes de criar dependências de implementação.','Escopo v1.0 e registro de decisões iniciais.','Autor consegue explicar offline, Jev, workers e cinco alvos sem contradições.',r'''
[Decisão] Congelar documentos de entrada :: Copiar RFC e plano de integração para referências; registrar hashes e divergências. | docs/baseline/manifest.json | Os dois documentos permanecem íntegros e cada divergência aponta à seção de origem.
[Decisão] Delimitar v1.0 :: Enumerar jornadas Captura→Mesa→Peça e separar requisitos de propostas pós-v1.0. | docs/product/scope-v1.md | Nenhuma funcionalidade original é silenciosamente adiada para v2; exclusões estão explícitas.
[Decisão] Ratificar núcleo e workers :: Resolver ADR-001; distinguir linguagem do Core, runtime e modelo de distribuição. | docs/adr/001-composicao.md | Decisão aprovada pelo autor; rejeição de workers bloqueia o restante e exige novo plano.
[Decisão] Ratificar Jev/offline :: Escolher fallback local ou pendência; definir quando a API pode receber conteúdo. | docs/adr/004-egress-offline.md | Falta de chave/rede não bloqueia captura, leitura ou autoria; nenhuma chamada implícita permitida.
[Decisão] Escolher host de referência :: Registrar máquina principal e acesso de teste aos cinco alvos previstos. | docs/adr/006-targets.md | Plataforma de desenvolvimento escolhida e lacunas de hardware identificadas, sem alegar homologação.
[Decisão] Fixar semântica de versões :: Separar versão do app, formato do Vault, contratos de workers e modelos. | docs/product/version-policy.md | Atualizar o app não implica migrar o Vault; regras pre-1.0 e compatibilidade estão claras.
[Decisão] Definir dados sensíveis :: Classificar conteúdo, consultas, metadados, chaves e logs; negar envio por padrão. | docs/security/data-classification.md | Casos de resumo e título sensível também exigem política de saída, não apenas documentos completos.
[Decisão] Mapear atores e ameaças :: Incluir arquivo malformado, página hostil, extensão, modelo e processo local malicioso. | docs/security/threat-model.md | Cada ator tem ativos, fronteiras e ataques de teste; modelo não aparece como autoridade confiável.
[Decisão] Registrar não escopo :: Excluir SaaS obrigatório, colaboração em tempo real e marketplace irrestrito deste horizonte. | docs/product/non-goals.md | Funcionalidades não aprovadas não entram por implicação da presença de Yjs, LocalAI ou MCP.
[Decisão] Aprovar inventário de requisitos :: Numerar invariantes, métricas e critérios; designar o autor como aprovador dos gates. | docs/product/requirements.yaml | Cada requisito tem origem, critério e responsável; ambiguidades não são tratadas como concluídas.
''')

add(2,'Definir formatos, identidade e autoridade','Vault / contratos','Próprio',['R02','R04','R05','R11','R13','R32','R37'],['S01'],
'Especificar representação persistente antes do editor e dos índices.','Schemas v0 do Vault, board, eventos, células e evidências.','Fixtures independentes dos motores representam uma mesa e uma Peça completas.',r'''
[Contrato] Definir entidade e ocorrência :: Separar fonte, conteúdo da célula/nota, ocorrência visual, grupo, aresta, Peça e bloco. | schemas/domain-v0.json | A mesma nota em dois boards tem conteúdo único e duas geometrias independentes.
[Contrato] Definir IDs estáveis :: Fixar geração, namespace e regras de colisão sem depender do nome do arquivo. | schemas/identity-v0.json | Renomear/mover não altera identidade; duplicar como nova autoria recebe novo ID.
[Decisão] Definir autoridade de arquivos :: Formalizar JSON+histórico e MPK derivado; resolver ADR-002. | docs/adr/002-file-truth.md | Recuperação tem precedência determinística e não escolhe fonte por timestamp arbitrário.
[Contrato] Especificar perfil Markdown :: Listar blocos suportados e representação de frontmatter, IDs, tabelas e citações. | docs/adr/005-markdown-profile.md | Cada bloco possui round-trip previsto; recurso não representável é preservado ou recusado, não achatado.
[Contrato] Especificar topologia :: Registrar posições, dimensões, grupos, conexões, rótulos e referências de conteúdo. | schemas/board-v0.json | Ciclos e arestas entre grupos são representáveis sem duplicar conteúdo.
[Contrato] Especificar revisões e eventos :: Definir base_revision, operação, autor, origem IA e marcador de confirmação. | schemas/event-v0.json | Evento não confirmado é distinguível de estado salvo; replay e conflitos têm regras explícitas.
[Contrato] Especificar evidência :: Combinar source_id, revisão, trecho, localizador, hash e bloco de destino. | schemas/evidence-v0.json | Referência distingue texto citado de comentário e não se torna inválida por renomeação.
[Contrato] Especificar metadados de IA :: Separar resumo gerado/editado, tags sugeridas/aceitas e identificação do pipeline. | schemas/enrichment-v0.json | Reprocessamento não sobrescreve correção humana; cobertura parcial pode ser declarada.
[Teste] Construir fixtures válidas e inválidas :: Criar um Vault mínimo, um completo e casos com campos desconhecidos. | tests/fixtures/schema/ | Validadores aceitam extensões permitidas e rejeitam IDs duplicados, tipos errados e referências impossíveis.
[Decisão] Definir evolução de schema :: Documentar migração, versão desconhecida, abertura somente leitura e exportação. | docs/contracts/schema-evolution.md | Um app antigo nunca regrava silenciosamente um formato novo que não compreende.
''')

add(3,'Inventariar upstreams e definir recortes','Todos os motores / licenças','Desconstrução',['R03','R30','R33','R34'],['S01','S02'],
'Fixar origem, licenças e modo de integração sem criar forks indiscriminados.','Catálogo de componentes e política de manutenção upstream.','Cada motor tem origem rastreável, fronteira proposta e decisão de manter/extrair/adaptar.',r'''
[Upstream] Inventariar repositórios :: Registrar URL, licença observada, linguagens, releases e componentes transitivos críticos. | upstream/catalog.yaml | BlockSuite, PageIndex, Docling, Jev, Qdrant, HelixDB, Scrapling, SearxNG e LocalAI estão cobertos.
[Upstream] Criar manifesto de origem :: Definir campos commit/tag, hashes, patches, avisos e comando de build. | upstream/manifest.schema.json | Nenhum componente empacotado pode usar latest ou branch móvel sem resolução para uma revisão.
[Licença] Examinar BlockSuite por árvore :: Distinguir standalone de AFFiNE e fechar o inventário dos arquivos candidatos. | docs/legal/blocksuite-scope.md | Licença não é inferida apenas pelo nome do projeto ou por um único package.json.
[Licença] Examinar SearxNG e modelos :: Separar código AGPL, integração por API e termos dos pesos/backends. | docs/legal/component-obligations.md | Fonte correspondente, avisos e redistribuição estão mapeados por artefato.
[Decisão] Ratificar licença própria :: Resolver ADR-003; explicar operação não comercial versus direitos open source de terceiros. | docs/adr/003-license.md | Licença aprovada sem cláusula non-commercial incompatível disfarçada de open source.
[Upstream] Delimitar reutilização :: Marcar bibliotecas, adapters, forks mínimos e serviços; excluir UIs completas e stores concorrentes. | docs/architecture/reuse-map.md | Cada motor tem lista de incluir/excluir e interfaces Sandland responsáveis pela autoridade.
[Upstream] Definir política de patches :: Criar série pequena e reprodutível; separar correções genéricas de customizações. | upstream/PATCH_POLICY.md | Um fork pode ser rebaseado e sua diferença para upstream pode ser enumerada.
[Governança] Preparar avisos e SBOM :: Criar estrutura de LICENSES, NOTICE e inventário de dependências. | compliance/ | Não há licença de terceiros substituída pela licença própria sem base legal.
[Governança] Definir ciclo de atualização :: Fixar auditoria, testes diferenciais, rollback e prioridade para vulnerabilidades. | docs/maintenance/upstream.md | Atualização sem evidência não chega automaticamente ao usuário ou altera modelos instalados.
[Decisão] Definir limites de spikes :: Para cada prova, listar pergunta, corpus, evidência mínima e decisão de continuar/replanejar. | docs/spikes/register.md | Relatório inconclusivo não vira integração aprovada; Qdrant e Helix têm protocolo comparável.
''')

add(4,'Montar repositório e fluxo solo com IA','Ferramentas / governança','Próprio',['R03','R30','R33','R34'],['S03'],
'Tornar alterações pequenas, revisáveis e reproduzíveis por um único mantenedor.','Monorepo de trabalho e instruções de contribuição/agentes.','Um agente implementa uma tarefa isolada e o autor consegue revisar o patch e seus testes.',r'''
[Implementação] Criar estrutura do monorepo :: Separar app desktop, crates, adapters, workers, schemas, fixtures e docs. | repository-layout.md | Dependências apontam para contratos e não para UI/estado privado de outro motor.
[Implementação] Fixar toolchains :: Pin Rust, JS e runtimes necessários para desenvolvimento, sem instaladores globais implícitos. | toolchains.lock | Ambiente limpo reproduz versões; mudanças de versão são diff explícito.
[Implementação] Configurar formatadores e lint :: Definir regras por linguagem e exclusões de fontes vendorizadas. | tooling/lint/ | Comando único reporta erros sem reformatar ou apagar avisos de arquivos upstream.
[Implementação] Criar comandos de projeto :: Padronizar bootstrap, check, test, fixtures e pacote de evidências. | tooling/tasks/ | Comandos têm help, retorno de erro confiável e não dependem do histórico do shell.
[Governança] Criar política de branches :: Uma frente mutável principal; PRs pequenos com IDs de tarefa e rollback. | CONTRIBUTING.md | Dois agentes não alteram contratos compartilhados em paralelo sem acordo explícito.
[Governança] Escrever instruções de agentes :: Delimitar arquivos, segredos, rede, testes obrigatórios e critérios de parada. | AGENTS.md | Agente não pode declarar testes executados sem logs nem aprovar a própria entrega.
[Governança] Criar template de tarefa :: Incluir contrato, fixture, não escopo, aceite e evidências. | .github/ISSUE_TEMPLATE/task.yml | Tarefa pode ser entregue a novo agente sem depender de memória informal da conversa.
[Governança] Criar template de PR :: Exigir origem de código, mudanças de schema, segurança, testes e artefatos. | .github/pull_request_template.md | PR sem evidências ou com teste pulado tem bloqueio explícito.
[Implementação] Separar configuração e segredos :: Fornecer arquivos de exemplo sem credenciais e política de logs. | config/examples/ | Scanner detecta segredos plantados em fixture e não coleta dados reais do desenvolvedor.
[Teste] Executar ensaio do fluxo :: Fazer alteração descartável em fixture, revisão adversarial e reversão. | reports/workflow-dry-run.md | Histórico evidencia proposta, teste, revisão humana e rollback, sem merge automático por agente.
''')

add(5,'Construir CI, corpus e protocolo de medição','Qualidade / desempenho','Próprio',['R01','R09','R12','R15','R20','R34'],['S04'],
'Criar a bancada que impedirá promessas de qualidade ou recursos sem medição.','CI inicial, corpus licenciado e perfis mensuráveis.','Falha introduzida em contrato bloqueia pipeline e um benchmark gera dados reproduzíveis.',r'''
[Implementação] Montar CI de contratos :: Rodar schemas, lint e testes determinísticos sem chaves de serviços. | .github/workflows/check.yml | PR com schema inválido falha; CI básica passa sem acesso a Jev/BYOK.
[Implementação] Criar matriz de builds :: Incluir os cinco alvos quando suportado pelo ambiente; distinguir build e teste nativo. | ci/target-matrix.yaml | Painel não mostra target compilado como target funcionalmente homologado.
[Teste] Montar corpus documental :: Reunir PDFs textuais/escaneados, DOCX, HTML, Markdown e transcrições com direitos de uso. | tests/corpus/manifest.json | Cada item possui hash, idioma, formato e licença/origem; nenhum dado privado entra no repositório.
[Teste] Montar corpus de busca :: Rotular perguntas, respostas esperadas, ausência de resposta e fontes em português. | tests/eval/retrieval-v1.jsonl | Conjunto de avaliação é separado dos exemplos usados para ajustar prompts e thresholds.
[Teste] Criar corpus hostil :: Incluir traversal, HTML ativo, injeção de prompt, arquivos truncados e URLs internas. | tests/security/fixtures/ | Cada fixture é inerte fora de harness controlado e tem comportamento negado esperado.
[Decisão] Fixar métricas de memória :: Definir RAM/RSS/PSS/VRAM, processos incluídos e diferenças por SO. | docs/adr/007-resource-profiles.md | Meta de 350 MB não é validada medindo apenas o processo Rust.
[Implementação] Criar harness de desempenho :: Medir startup interativo, ack IPC, latência de busca e frame times. | tooling/bench/ | Dados brutos incluem hardware, versão, dataset, warm/cold e repetição; não apenas médias soltas.
[Teste] Gerar cenas sintéticas :: Criar boards de 100 e 1.000 nós com textos, imagens, grupos e conexões. | tests/fixtures/boards/ | Cenas são determinísticas e não representam só caixas vazias favoráveis ao benchmark.
[Implementação] Padronizar evidências :: Definir JSON de resultados e caminho por tarefa, hash de commit e ambiente. | schemas/evidence-report.json | Relatório distingue executado, falhou, bloqueado e não executado; campo de aprovação é humano.
[Teste] Validar a própria bancada :: Injetar regressão de tempo, erro de schema e ausência de runner. | reports/ci-self-test.md | Cada situação gera falha ou estado bloqueado, nunca resultado verde fictício.
''')

add(6,'Criar shell desktop e contratos IPC','Tauri / apresentação','Integração',['R01','R03','R07','R08','R31','R34'],['S04','S05'],
'Abrir um app local mínimo sem antecipar a integração de todos os motores.','Shell Tauri com navegação Ingest/Mesa/Peça e IPC validado.','Instalar build de desenvolvimento, navegar e selecionar um diretório sem rede.',r'''
[Implementação] Inicializar Tauri v2 :: Montar frontend mínimo e Core; manter motores pesados fora do startup. | apps/desktop/ | Shell abre sem modelos, browsers, servidores de pesquisa ou bibliotecas Python carregados.
[Implementação] Definir contratos IPC :: Versionar request/response, erros, IDs e cancelamento; gerar tipos onde possível. | schemas/ipc-v0.json | Frontend não chama comandos arbitrários ou APIs nativas por nomes não autorizados.
[Implementação] Configurar capabilities :: Expor apenas comandos do app e aplicar CSP restritiva à apresentação. | apps/desktop/capabilities/ | Script de conteúdo não obtém acesso automático a filesystem, shell ou segredos.
[Implementação] Montar navegação de fases :: Criar estados Ingest, Mesa, Peça e ajustes sem duplicar dados por tela. | apps/desktop/src/shell/ | Alternar fase não perde estado nem exige conta ou serviço remoto.
[Implementação] Implementar seleção de Vault :: Usar diálogo nativo e raiz dinâmica; adiar leitura ao serviço autorizado. | crates/app-core/src/vault_selection.rs | Paths com espaços/Unicode funcionam e nenhuma raiz de usuário está hardcoded.
[Implementação] Criar estados de tarefa :: Padronizar vazio, carregando, falhou, bloqueado, offline e concluído. | apps/desktop/src/components/status/ | Falha recuperável mostra ação útil e não fica em spinner infinito.
[Implementação] Preparar acessibilidade básica :: Dar nomes, foco e atalhos às regiões principais. | apps/desktop/src/accessibility/ | Percurso essencial por teclado funciona sem mouse; foco não desaparece ao trocar de fase.
[Implementação] Integrar logs locais saneados :: Emitir IDs e tempos sem incluir texto do Vault por padrão. | crates/diagnostics/ | Fixture sensível não aparece em log, erro serializado ou console da UI.
[Teste] Fazer smoke nos alvos disponíveis :: Abrir/fechar, selecionar pasta e testar IPC em ambientes reais. | reports/shell-targets/ | Ambiente não disponível é marcado bloqueado, com tarefa de acesso antes da homologação.
[Teste] Medir baseline vazio :: Registrar árvore de processos e startup sem funcionalidades opcionais. | reports/baseline-shell.json | Baseline reproduzível permite atribuir consumo incremental a motores futuros.
''')

add(7,'Implementar Vault, identidade e assets','VaultStore / CAS','Próprio',['R02','R04','R05','R07'],['S02','S06'],
'Criar a primeira autoridade persistente do produto.','VaultStore básico, manifest e armazenamento de assets por hash.','Criar Vault, salvar nota/asset, fechar e listar os mesmos IDs ao reabrir.',r'''
[Implementação] Criar lifecycle do Vault :: Abrir, criar, validar versão e recusar raiz inválida sem modificar seus arquivos. | crates/vault/src/lifecycle.rs | Pasta existente incompatível não é sobrescrita; erro aponta a causa.
[Implementação] Implementar identidade :: Gerar IDs, detectar duplicatas e separar ID de localização atual. | crates/domain/src/identity.rs | Renomear não muda ID; cópia com ID repetido gera conflito tratável.
[Implementação] Ler/escrever frontmatter :: Preservar campos desconhecidos e tipos permitidos no perfil aprovado. | crates/vault/src/markdown.rs | Round-trip de fixtures não apaga extensões nem converte datas/strings inadvertidamente.
[Implementação] Implementar CAS :: Calcular hash por streaming, detectar duplicatas e publicar arquivo completo. | crates/vault/src/assets.rs | Dois imports iguais compartilham bytes; interrupção não cria asset final parcial.
[Implementação] Mapear referências de assets :: Referenciar hash/tipo sem caminhos absolutos dependentes da máquina. | crates/domain/src/asset_ref.rs | Vault copiado para outra raiz continua resolvendo anexos.
[Implementação] Criar workspace/célula :: Persistir manifest, board vazio e célula Markdown por contratos. | crates/vault/src/workspaces.rs | Criação incompleta é detectada e reparável sem gerar workspace invisível perdido.
[Implementação] Implementar inventário por varredura :: Ler arquivos canônicos sem exigir banco existente. | crates/vault/src/scan.rs | Inventário inicial retorna os mesmos IDs após remover índices derivados.
[Implementação] Definir descarte e recuperação :: Criar lixeira/tombstones sem destruir imediatamente a fonte de rollback. | crates/vault/src/deletion.rs | Excluir uma referência não apaga asset ainda usado por outra célula ou revisão.
[Teste] Testar portabilidade de paths :: Cobrir Unicode, case sensitivity, nomes reservados e diretórios longos. | tests/vault/path-compat/ | Casos não suportados são recusados com explicação em vez de truncados silenciosamente.
[Teste] Criar ferramenta de inspeção :: Listar invariantes e problemas em modo somente leitura. | tooling/vault-inspect/ | Ferramenta lê o Vault sem iniciar UI, modelos ou serviços externos.
''')

add(8,'Implementar transações de arquivos e histórico','Journal / RevisionStore','Próprio',['R02','R05','R13','R37'],['S07'],
'Tornar confirmação, replay e desfazer independentes de bancos descartáveis.','Journal durável e protocolo de publicação de revisões.','Interromper cada etapa de uma edição e recuperar o último estado confirmado.',r'''
[Contrato] Especificar máquina de estados de commit :: Definir preparação, confirmação, publicação e replay idempotente. | docs/contracts/file-commit.md | Ordem de flush/rename e estado reconhecido após cada falha estão definidos por SO.
[Implementação] Criar escritor único :: Serializar mutações por Vault e validar expected_revision. | crates/vault/src/writer.rs | Duas edições concorrentes não substituem uma à outra sem conflito explícito.
[Implementação] Persistir objetos de revisão :: Armazenar snapshots/deltas necessários antes de anunciar commit. | crates/history/src/objects.rs | Toda revisão confirmada possui objetos íntegros para reconstrução.
[Implementação] Implementar journal :: Registrar eventos, integridade de registros e tratamento de cauda truncada. | crates/history/src/journal.rs | Registro parcial não invalida commits anteriores e não é executado como completo.
[Implementação] Publicar arquivos atômicos :: Usar temporários, flush e troca apropriada; documentar garantias reais de cada SO. | crates/vault/src/atomic_write.rs | Leitor observa versão anterior ou nova completa; falhas de disco são propagadas.
[Implementação] Implementar replay :: Repetir operações confirmadas sem criar cópias ou efeitos extras. | crates/history/src/replay.rs | Executar replay duas vezes produz o mesmo estado e contagem de entidades.
[Implementação] Criar undo semântico :: Reverter edição, tag ou operação de board por nova operação registrada. | crates/history/src/undo.rs | Undo preserva proveniência e não apaga a existência do evento original.
[Implementação] Encadear eventos :: Definir serialização canônica, hashes e verificação do histórico. | crates/history/src/integrity.rs | Alteração detectada relativamente ao checkpoint confiável; limite de reescrita total documentado.
[Teste] Injetar falhas de I/O :: Simular disco cheio, permissão, flush/rename interrompidos e erro de objeto ausente. | tests/history/fault-injection/ | App não informa salvo para commit não durável nem remove a única cópia válida.
[Teste] Medir janela de perda :: Instrumentar timestamps de edição/commit e executar encerramentos controlados. | reports/durability-window.json | Janela de 500 ms é medida; teste de processo não é apresentado como teste de queda de energia.
''')

add(9,'Implementar broker de arquivos e defesa TOCTOU','VaultGuard / ToolBroker','Próprio',['R06','R08','R34'],['S07','S08'],
'Eliminar validação de path separada da abertura antes de integrar workers.','I/O autorizado por referências/handles e suíte de ataques de paths.','Solicitar um arquivo permitido e negar escapes durante resolução e concorrência.',r'''
[Contrato] Definir capabilities de I/O :: Vincular ator, workspace, objetos permitidos, operação, expiração e revisão. | schemas/io-capability.json | Capacidade de leitura não autoriza escrita nem acesso a outro workspace.
[Implementação] Implementar resolução por IDs :: Converter referências em objetos autorizados dentro do Core. | crates/security/src/object_broker.rs | Texto vindo do modelo não pode se tornar path arbitrário por concatenação.
[Implementação] Implementar abertura Linux :: Usar diretório-base/handle e resolução restrita conforme kernel disponível. | crates/security/src/fs/linux.rs | Troca de symlink durante acesso não sai do escopo; recurso ausente tem política fail-closed.
[Implementação] Implementar abertura Windows :: Tratar handles, reparse points, junctions e canonicalização por mecanismo efetivo. | crates/security/src/fs/windows.rs | Junction trocada e path especial não contornam a autorização.
[Implementação] Implementar abertura macOS :: Usar primitivas relativas/handles e política de links consistente com o contrato. | crates/security/src/fs/macos.rs | Testes nativos demonstram contenção; não se reutiliza apenas canonicalize seguido de open.
[Implementação] Limitar leitura e escrita :: Aplicar tamanho, tipo e destino permitido; evitar arquivos especiais inesperados. | crates/security/src/io_limits.rs | Arquivo/dispositivo excessivo ou tipo indevido é rejeitado antes de consumo sem limite.
[Implementação] Autorizar fontes compartilhadas :: Separar assets/origens read-only de conteúdo privado de workspaces. | crates/security/src/source_scope.rs | Fonte aprovada pode ser lida sem conceder o diretório global inteiro.
[Teste] Criar corridas controladas :: Trocar symlinks/diretórios em loop de teste durante leituras. | tests/security/toctou/ | Harness registra nenhuma leitura de sentinela externa; falhas não são ignoradas.
[Teste] Cobrir nomes hostis e hard links :: Definir política explícita e testar absolutos, UNC, traversal e links. | tests/security/path-policy/ | Política é aplicada e limitações de hard links/mounts constam do relatório.
[Teste] Auditar APIs de filesystem :: Procurar acessos diretos fora dos módulos autorizados e criar regra de revisão. | reports/security/fs-authority.md | Fluxos de aplicação não têm bypass conhecido do broker para operações com dados não confiáveis.
''')

add(10,'Reconciliar alterações externas e reconstruir índices','Watcher / projeções','Próprio',['R02','R04','R05','R21'],['S08','S09'],
'Manter o disco como autoridade mesmo com edição externa e perda de notificações.','Reconciliação incremental e cache de inventário descartável.','Editar/renomear fora do app, perder um evento de watcher e reconciliar sem perda.',r'''
[Implementação] Integrar watcher :: Debounce eventos, distinguir escrita própria e tratar sequência rename/delete/create. | crates/vault/src/watch.rs | Uma escrita do Core não gera loop infinito de reimportação.
[Implementação] Criar reconciliação por scan :: Comparar hashes/revisões e descobrir mudanças perdidas pelo watcher. | crates/vault/src/reconcile.rs | Eventos descartados intencionalmente são recuperados pela varredura posterior.
[Implementação] Resolver renomeação por ID :: Atualizar localização sem recriar nota e sem quebrar referências. | crates/vault/src/relocation.rs | Citações e ocorrências continuam apontando para a mesma identidade.
[Implementação] Detectar edição concorrente :: Comparar base, estado em edição e mudança externa. | crates/domain/src/conflict.rs | Nenhuma estratégia last-write-wins silenciosa destrói alteração autoral.
[Implementação] Criar UI de conflito :: Mostrar versões, salvar cópia e permitir resolução explícita. | apps/desktop/src/conflicts/ | Cancelar resolução conserva as duas versões e não bloqueia todo o Vault.
[Implementação] Criar projeção SQLite :: Indexar inventário/revisões para navegação sem torná-lo canônico. | crates/index/src/catalog.rs | Apagar DB e reconstruir produz inventário equivalente ao scan direto.
[Implementação] Registrar eventos de invalidação :: Emitir mudança de texto, metadado e topologia separadamente. | crates/domain/src/change_events.rs | Mudança de posição não é tratada como alteração do corpo textual.
[Implementação] Invalidar tombstones :: Remover projeções de itens excluídos e impedir ressurreição por job atrasado. | crates/index/src/tombstones.rs | Resultado de revisão anterior não recria material excluído.
[Teste] Testar importação externa parcial :: Cobrir arquivo ainda sendo escrito, frontmatter inválido e lock externo. | tests/vault/external-edits/ | App aguarda/reporta erro sem normalizar arquivo incompleto de modo destrutivo.
[Teste] Validar reconstrução repetida :: Executar rebuild em Vault com referências e conflitos conhecidos. | reports/rebuild-catalog.json | Duas reconstruções têm as mesmas entidades e não alteram arquivos canônicos.
''')

add(11,'Concluir v0.1: recuperação e diagnóstico do Vault','Vault / integração','Próprio',['R02','R05','R06','R07','R29','R34'],['S07','S08','S09','S10'],
'Entregar um núcleo de dados utilizável antes de adicionar complexidade visual.','v0.1.0 alpha técnica e utilitário de recuperação.','Criar, modificar, corromper um cache, reconstruir e exportar um Vault local.',r'''
[Implementação] Criar comando validate :: Inspecionar schemas, referências, objetos e journal sem efetuar reparo automático. | tooling/vault-cli/validate | Relatório distingue erro canônico, cache inválido e aviso recuperável.
[Implementação] Criar comando rebuild :: Recriar projeções a partir de snapshot coerente e trocar índice ao concluir. | tooling/vault-cli/rebuild | Interrupção conserva o índice anterior ou retorna modo degradado; não altera autoria.
[Implementação] Criar recuperação assistida :: Listar revisões recuperáveis e pedir confirmação antes de publicar reparo. | tooling/vault-cli/recover | Preview corresponde ao resultado e existe ponto de retorno.
[Implementação] Criar exportação canônica :: Exportar manifest, textos, topologia, assets e histórico conforme perfil. | tooling/vault-cli/export | Vault exportado valida numa raiz diferente sem dados da máquina original.
[Teste] Executar matriz de crashes :: Interromper commits em todos os pontos instrumentados e comparar resultados. | reports/v0.1/crash-matrix.json | Nenhum estado confirmado fica irrecuperável nas falhas modeladas.
[Teste] Executar ensaio de energia/disco :: Documentar setup seguro e limitações; usar ambiente descartável apropriado. | reports/v0.1/power-failure.md | Não há afirmação de durabilidade física baseada apenas em matar processo.
[Teste] Apagar todos os caches :: Remover SQL/MPK/projeções previstas e executar a recuperação completa. | reports/v0.1/cache-destruction.json | Conteúdo, referências e histórico autoral são preservados byte a byte ou semanticamente conforme contrato.
[Documentação] Escrever manual do formato :: Explicar diretórios, autoridade, recovery e edição externa. | docs/vault/format-v0.md | Outro implementador consegue localizar dados sem abrir o app.
[Licença] Empacotar alpha técnica :: Anexar manifests, avisos, fontes e limitações dos alvos efetivamente testados. | release/v0.1.0/ | Pacote não afirma conter editor/canvas/IA prontos nem homologação ausente.
[Teste] Gravar demonstração de aceite :: Repetir jornada do sprint em instalação de desenvolvimento limpa. | reports/v0.1/acceptance.md | Autor aprova com logs, hashes e pendências; problemas de integridade bloqueiam a tag.
''')
