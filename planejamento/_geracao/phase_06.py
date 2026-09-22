from plan_base import add

add(61,'Estabilizar uso real e ratificar evolução','Manutenção / produto','Próprio',['R05','R09','R31','R34','R36'],['S60'],
'Priorizar confiabilidade observada antes de ampliar escopo.','Backlog de estabilização e ADR do pós-v1.0.','Reproduzir relato real/ensaio de uso e vincular correção a teste sem coletar dados privados.',r'''
[Governança] Coletar feedback consentido :: Usar issues e diagnóstico local saneado, sem telemetria compulsória. | reports/post-v1/feedback-register.csv | Relatos possuem consentimento/origem; ausência de usuários externos é declarada.
[Governança] Triar incidentes :: Priorizar perda de dados, fuga de escopo, falhas essenciais e recursos. | docs/maintenance/triage.md | Hotfix crítico pode interromper qualquer sprint futuro.
[Teste] Reproduzir problemas reais :: Transformar entradas permitidas em fixtures mínimas sem expor documentos do usuário. | tests/regressions/post-v1/ | Cada bug corrigido tem caso reproduzível; bug não confirmado é identificado como hipótese.
[Implementação] Corrigir falhas prioritárias :: Implementar patches estreitos com revisão e rollback. | maintenance/v1.1/fixes/ | Correção não introduz migração/feature escondida e passa contratos pertinentes.
[Teste] Executar uso prolongado :: Alternar Vaults, edição, OCR, pesquisa e inferência em sessão extensa. | reports/post-v1/soak.json | Crescimento de recursos, deadlocks e atrasos são medidos, não só observados visualmente.
[Governança] Revisar dívida dos adapters :: Listar patches locais, APIs internas e riscos por engine. | upstream/debt-register.md | Cada divergência tem motivação e estratégia de reduzir/manter/upstreamar.
[Decisão] Ratificar propostas pós-v1 :: Resolver ADR-017 para escala, offline/interoperabilidade e futuros provedores. | docs/adr/017-post-v1-scope.md | Novas funcionalidades permanecem propostas até aprovação humana; não bloqueiam correções essenciais.
[Decisão] Definir alvos de escala :: Escolher corpus/hardware com base em uso observado antes de otimizar. | bench/profiles/post-v1.yaml | Novas metas não substituem silenciosamente budgets básicos do produto.
[Governança] Definir política de suporte :: Estabelecer ramos, patches de segurança, depreciação e compatibilidade de formatos. | docs/maintenance/support-policy.md | Não há promessa de suporte eterno ou calendário irreal para equipe solo.
[Teste] Fechar regressão de estabilidade :: Comparar candidata com v1.0 e registrar o que efetivamente melhorou. | reports/v1.1/stability-baseline.md | Ausência de defeito conhecido não é preenchida por bug inventado para marcar tarefa concluída.
''')

add(62,'Aprimorar recuperação assistida e diagnóstico','Recovery / observabilidade local','Próprio',['R02','R05','R08','R29','R31','R37'],['S61'],
'Tornar falhas recuperáveis pelo usuário sem depender do mantenedor ou de nuvem.','Assistente de diagnóstico/reparo com preview e reversão.','Diagnosticar índice corrompido e conflito, reparar somente o que foi autorizado.',r'''
[Implementação] Criar painel de saúde do Vault :: Reunir schemas, referências, histórico, caches e jobs. | apps/desktop/src/diagnostics/vault-health.ts | UI distingue problema autoral de índice descartável e evita alarmismo genérico.
[Implementação] Adicionar preview de reparo :: Listar arquivos afetados, causa e resultado antes de modificar. | crates/recovery/src/plan.rs | Operação destrutiva não é escondida dentro de botão reconstruir índice.
[Implementação] Criar checkpoint de reparo :: Preservar reversibilidade conforme política de retenção. | crates/recovery/src/checkpoint.rs | Reparo pode ser desfeito sem perder alterações confirmadas posteriores indevidamente.
[Implementação] Melhorar reconciliação assistida :: Resolver órfãos e conflitos por decisão explícita. | apps/desktop/src/diagnostics/reconcile.ts | Sistema não escolhe automaticamente o arquivo mais novo como verdade universal.
[Implementação] Criar export de diagnóstico saneado :: Remover corpo documental, caminhos sensíveis e segredos por padrão. | crates/diagnostics/src/export.rs | Fixtures-semente privadas não aparecem no pacote compartilhável.
[Implementação] Explicar consumo por capacidade :: Atribuir recursos a modelo, browser, OCR, indexador e editor. | apps/desktop/src/diagnostics/resources.ts | Usuário pode pausar o causador real sem encerrar o app inteiro.
[Implementação] Melhorar retomada de jobs :: Distinguir retry seguro, nova versão do input e tarefa manual necessária. | crates/jobs/src/recovery.rs | Job antigo não repete envio remoto após consentimento revogado.
[Teste] Ensaiar corrupção combinada :: Misturar cache inválido, arquivo ausente e journal incompleto em cópia descartável. | tests/recovery/scenarios/ | Ferramenta conserva dados recuperáveis e relata o que não pode reconstruir.
[Teste] Testar privacidade do suporte :: Tentar incluir segredos/metadados por caminhos de erro e dumps. | reports/recovery/privacy.json | Diagnóstico automático não envia dados; compartilhamento é opção explícita.
[Documentação] Escrever runbooks :: Guiar situações de disco cheio, modelo corrompido, índice perdido e conflito Git. | docs/user/troubleshooting/ | Usuário pode seguir passos sem executar comandos desconhecidos como administrador.
''')

add(63,'Atualizar motores com segurança e publicar v1.1','Upstreams / estabilidade','Integração',['R01','R30','R32','R33','R34'],['S61','S62'],
'Provar que a composição pode ser mantida sem virar um fork monolítico congelado.','v1.1.0 e primeiro ciclo completo de manutenção upstream.','Atualizar recorte necessário, comparar resultados e reverter se um contrato quebrar.',r'''
[Upstream] Revisar mudanças upstream :: Examinar somente versões pertinentes e correções relevantes dos motores adotados. | reports/upstream/v1.1-review.md | Não se atualizam todos os componentes apenas para perseguir latest.
[Upstream] Rebasear patches editoriais :: Aplicar série BlockSuite e verificar grafo de deps/licenças. | upstream/blocksuite/patches/ | Round-trip, input e bundle continuam dentro do contrato.
[Upstream] Rebasear recorte PageIndex :: Inspecionar ferramentas, prompts, model calls e identidade. | upstream/pageindex/patches/ | Atualização não reintroduz cloud/store global ou renumeração de IDs autorais.
[Upstream] Atualizar ingest/web quando necessário :: Verificar Docling, Scrapling, normalizador e engines SearxNG. | upstream/maintenance-v1.1.json | Mudanças de extração são avaliadas com corpus; fontes antigas não são reescritas automaticamente.
[Upstream] Revisar LocalAI/modelos :: Separar atualização de runtime de mudança de pesos e backend. | models/maintenance-v1.1.json | Usuário não recebe troca silenciosa de modelo ou espaço de embeddings.
[Upstream] Revisar backend de busca :: Testar índice/SDK escolhido e migração de cache em cópia descartável. | reports/search/v1.1-upgrade.md | Mudança do motor não exige modificar dados autorais para continuar funcionando.
[Teste] Executar comparação diferencial :: Rodar corpus antigo e novo comparando estrutura, busca e citações. | reports/v1.1/differential.json | Ganho em um corpus não oculta regressão de escopo, IDs ou fonte.
[Licença] Atualizar SBOM/fontes :: Registrar novas versões, avisos, patches e artefatos distribuídos. | compliance/releases/v1.1.0/ | Inventário corresponde ao pacote publicado, não só às dependências de desenvolvimento.
[Governança] Contribuir correções genéricas :: Preparar propostas upstream quando adequado, sem depender da aceitação para preservar segurança. | upstream/contributions-v1.1.md | Mudança mantém rastreabilidade e termos; PR não enviado não é anunciado como contribuição aceita.
[Release] Publicar v1.1 e verificar rollback :: Incluir fixes, suporte, diagnóstico e restauração testada. | release/v1.1.0/ | Usuário v1.0 atualiza sem perder dados e dispõe de procedimento de retorno compatível.
''')

add(64,'Escalar busca, ingestão e contexto com evidência','Escala / pipelines','Evolução proposta',['R09','R12','R14','R21','R22','R36'],['S61','S63'],
'Aumentar capacidade onde as medições mostram necessidade, sem reinventar motores.','Pipelines incrementais e budgets validados no corpus ampliado aprovado.','Processar lote maior mantendo autoria responsiva e resultados/revisões corretos.',r'''
[Decisão] Confirmar escopo de escala :: Ratificar metas/corpus de ADR-017 e preservar os budgets base. | docs/product/scale-v1.2.md | Novo alvo é definido antes do benchmark, não depois de ver resultados favoráveis.
[Teste] Identificar gargalos :: Perfilar parsing, snapshots, indexação, queries, summaries e cena separadamente. | reports/scale/hotspots.json | Otimização nasce de evidência e não de preferência por reescrever Python em Rust.
[Implementação] Otimizar filas de lote :: Introduzir prioridades/fairness e backpressure para UI versus tarefas em massa. | crates/jobs/src/batch_scheduler.rs | Lote grande não impede salvar edição ou cancelar tarefa prioritária.
[Implementação] Otimizar parsing incremental :: Reaproveitar revisões/unidades quando backend permitir sem perder cobertura. | crates/ingest/src/incremental_extract.rs | Documento alterado não reutiliza texto de páginas incompatíveis apenas por posição.
[Implementação] Otimizar indexação em geração :: Agrupar upserts e publicar revisão consistente com tombstones. | crates/search/src/bulk_index.rs | Pico de throughput não faz ressurgir material excluído.
[Implementação] Otimizar contexto incremental :: Recalcular subárvores/grupos necessários e limitar entradas de cache. | crates/context/src/incremental_large.rs | Contexto permanece fiel às relações e não usa summaries obsoletos por economia.
[Implementação] Otimizar viewport medida :: Corrigir gargalos de arestas/previews/texturas demonstrados. | apps/desktop/src/board/perf/ | Melhor fluidez não remove acessibilidade ou precisão de hit testing.
[Teste] Validar cancelamento/retomada em escala :: Interromper import/index/contexto em diferentes pontos do lote. | tests/scale/recovery/ | Retomar não duplica autoria nem exige iniciar todo o trabalho sem motivo documentado.
[Teste] Comparar qualidade antes/depois :: Reexecutar avaliações reservadas de busca, summaries e fontes. | reports/scale/quality-regression.json | Otimização não reduz métricas essenciais abaixo dos limiares aprovados.
[Documentação] Publicar limites reais :: Descrever corpus/hardware e capacidades desabilitadas por orçamento. | docs/performance/v1.2.md | Nenhum número vira garantia genérica para qualquer desktop ou acervo.
''',note='Proposta de evolução: executar somente após ratificação ADR-017. Não adia correções de integridade/segurança pendentes.')

add(65,'Consolidar pacotes offline e perfis portáveis','Distribuição offline / modelos','Evolução proposta',['R01','R07','R09','R30','R36'],['S32','S57','S63','S64'],
'Facilitar provisionamento sem rede e sem dependências técnicas instaladas pelo usuário.','Criador/importador de pacotes de capacidade e perfis por hardware.','Levar pacote permitido para outra máquina compatível e usar IA local sem conectividade.',r'''
[Contrato] Definir pacote de capacidade :: Agrupar backend, modelos, licenças, hashes, target e dependências. | schemas/capability-pack-v1.json | Pacote declara tudo de que precisa e não executa script arbitrário ao importar.
[Implementação] Criar construtor de pacotes :: Selecionar somente artefatos com redistribuição permitida e manifest verificado. | tooling/capability-pack/build/ | Peso com termos incompatíveis não é incorporado apenas porque está disponível para download.
[Implementação] Criar importador offline :: Validar assinatura/hash, arquitetura e espaço antes de publicar instalação. | crates/models/src/import_pack.rs | Pacote errado/corrompido é recusado sem substituir uma instalação funcional.
[Implementação] Deduplicar armazenamento de artefatos :: Compartilhar conteúdo compatível entre perfis preservando versões. | crates/models/src/artifact_store.rs | Remover um perfil não remove modelo ainda usado por outro.
[Implementação] Criar perfis de hardware :: Recomendar capacidades com base em memória/CPU/GPU detectadas e homologadas. | crates/resources/src/hardware_profiles.rs | Detecção não concede download/rede automaticamente nem presume aceleração não testada.
[Implementação] Aplicar fallback de capacidade :: Escolher perfil já instalado ou marcar operação indisponível. | crates/models/src/profile_fallback.rs | Fallback nunca migra para provedor remoto sem consentimento.
[Implementação] Criar UI de instalação/remoção :: Mostrar tamanho, licença, destino e consequências de remover pacote. | apps/desktop/src/settings/capability-packs/ | Remoção não apaga Vault nem resultado autoral produzido pelo modelo.
[Teste] Testar portabilidade por target :: Instalar packs nos cinco alvos compatíveis e verificar dependências faltantes. | reports/offline/packs-matrix.json | Pacote de uma arquitetura não é anunciado como universal sem prova.
[Teste] Validar zero egress :: Executar rotas locais completas em rede bloqueada e registrar tentativas. | reports/offline/airgap-v1.2.json | Nenhum backend tenta buscar tokenizer/modelo auxiliar não declarado.
[Documentação] Publicar guia de operação desconectada :: Explicar preparação, atualização, verificação e limites do modo offline. | docs/user/offline-packs.md | App distingue operação local de instalação inicial que pode requerer aquisição prévia dos artefatos.
''',note='Proposta de evolução aprovada em ADR-017; não supõe que todas as licenças de pesos permitem redistribuição.')

add(66,'Ampliar interoperabilidade e publicar v1.2','Formatos abertos / exportação','Evolução proposta',['R02','R04','R13','R29','R32','R36'],['S58','S63','S64','S65'],
'Preservar utilidade do conhecimento fora do Sandland e preparar contratos estáveis.','v1.2.0 com interoperabilidade ampliada explicitamente aprovada.','Exportar mesa/Peça e importar cópia sem perder os campos declarados suportados.',r'''
[Decisão] Aprovar formatos adicionais :: Escolher recorte interoperável, por exemplo JSON Canvas e export editorial local. | docs/product/interoperability-v1.2.md | Formatos são propostas ratificadas; não se promete compatibilidade irrestrita com todo app de notas.
[Contrato] Definir mapeamento de topologia :: Relacionar nós/arestas/grupos e extensões Sandland ao formato escolhido. | schemas/interop/board-mapping.json | Relação sem representação é preservada em extensão ou relatada como perda antes do export.
[Implementação] Exportar board aberto :: Produzir referências portáveis e metadados de extensão documentados. | crates/export/src/board_interop.rs | Conteúdo não é duplicado de modo a perder identidade/proveniência.
[Implementação] Importar board em cópia segura :: Resolver paths e gerar IDs/mapeamentos sem autorizar acesso externo indevido. | crates/import/src/board_interop.rs | Importação não sobrepõe Vault ativo nem lê path arbitrário vindo do arquivo.
[Implementação] Exportar Peça em formato adicional :: Implementar formato aprovado, como HTML/PDF local, com assets e citações. | crates/export/src/editorial.rs | Não depende de renderer cloud nem recursos remotos não autorizados.
[Implementação] Exportar mapa de evidências :: Incluir fonte/revisão/localizador em manifest legível. | crates/export/src/evidence_manifest.rs | Texto final continua auditável por ferramenta externa.
[Teste] Testar round-trip interoperável :: Comparar campos suportados e perdas declaradas em fixtures. | tests/interop/ | Import/export não é chamado lossless quando o formato alvo não representa toda a semântica.
[Teste] Testar migração de v1.0/v1.1 :: Rodar corpus de versões e procedimento de restauração. | reports/v1.2/compatibility.json | Novos exports não quebram formatos canônicos existentes.
[Documentação] Publicar especificação de formato :: Fornecer exemplos e validadores para implementadores externos. | docs/specs/interoperability/ | É possível ler/converter dados sem vincular ao runtime do Sandland.
[Release] Publicar v1.2 estável :: Consolidar escala, packs offline e interoperabilidade com evidências. | release/v1.2.0/ | Novas capacidades têm limites claros e não mascaram regressões no fluxo v1.0.
''',note='Novos formatos/exportações dependem de aprovação em ADR-017. A versão do formato do Vault é independente da versão do app.')

add(67,'Ratificar desenho da v2.0 e contratos de provedores','Arquitetura v2 / extensibilidade','Evolução proposta',['R03','R06','R08','R32','R35'],['S61','S66'],
'Abrir extensibilidade delimitada sem transformar o app em executor genérico de plugins.','ADR v2.0, ameaça de extensão e superfície de SDK congelada.','Provedor de exemplo tem capacidades explícitas e não acessa APIs internas do Core.',r'''
[Decisão] Ratificar escopo v2 :: Resolver ADR-018 com objetivos observados e não escopo de marketplace/execução irrestrita. | docs/adr/018-provider-sdk.md | Autor aprova evolução; ausência de aprovação bloqueia implementação, não é presumida pelo roadmap.
[Contrato] Separar APIs internas e públicas :: Expor somente serviços de geração, extração, busca e contexto necessários. | docs/sdk/api-surface.md | Store canônico e mutação do filesystem não são disponibilizados diretamente à extensão.
[Contrato] Definir versionamento do SDK :: Separar contrato de transporte, capabilities e schema de dados. | schemas/provider-sdk-version.json | Breaking change de plugin não obriga breaking change do Vault.
[Contrato] Definir manifesto de extensão :: Identificar origem, versão, target, entradas/saídas, rede e recursos solicitados. | schemas/provider-extension-v1.json | Instalação não concede todos os direitos por padrão.
[Decisão] Definir modelo de confiança :: Distinguir componentes curados, código de terceiros e perfis experimentais. | docs/security/extensions-trust.md | Assinatura prova origem/integridade, não ausência de comportamento malicioso.
[Decisão] Atualizar ameaça de extensões :: Modelar exfiltração, resposta maliciosa, resource abuse e tentativa de ampliar escopo. | docs/security/extensions-threat-model.md | Riscos não são tratados como resolvidos apenas por executar em outro processo.
[Contrato] Definir APIs de dados minimizados :: Passar snapshot/refs limitados em vez de path do Vault. | schemas/sdk-input-refs.json | Plugin recebe só os dados necessários à tarefa aprovada.
[Contrato] Definir errors e lifecycle :: Padronizar cancelamento, shutdown, health e incompatibilidade. | schemas/sdk-errors.json | Provedor quebrado pode ser desabilitado sem corromper conteúdo ou impedir abertura do app.
[Teste] Criar fixtures de conformidade v2 :: Derivar casos de providers existentes e ataques de escopo. | tests/sdk/fixtures/ | SDK tem testes antes de uma extensão real depender de comportamento não especificado.
[Governança] Planejar migração interna :: Listar quais adapters atuais serão encaixados no SDK e ordem segura. | docs/sdk/internal-migration.md | Refatoração não troca motores escolhidos nem remove capacidades estáveis por conveniência.
''',note='Proposta pós-v1: a v2.0 não exige colaboração em nuvem, marketplace, execução de scripts nas notas ou reescrita total dos motores.')

add(68,'Implementar SDK e kit de conformidade','Provider SDK / adapters','Evolução proposta',['R06','R08','R27','R32','R34','R35'],['S67'],
'Tornar integrações reproduzíveis e verificáveis por contribuidores.','SDK mínimo, mocks e harness de contratos.','Criar provedor local de exemplo e testar comportamentos permitidos/negados sem chaves.',r'''
[Implementação] Gerar tipos do contrato :: Produzir bindings necessários aos adapters existentes a partir de schemas versionados. | sdk/generated/ | Tipos de linguagens diferentes serializam os mesmos casos sem ambiguidade.
[Implementação] Criar host SDK :: Negociar versão/capabilities e integrar o supervisor existente. | crates/provider-host/ | Nova camada não cria outro lifecycle ou outro sistema de permissões concorrente.
[Implementação] Criar cliente SDK mínimo :: Facilitar implementadores sem dar acesso direto ao Core. | sdk/client/ | Exemplo precisa somente de inputs autorizados e saídas por contrato.
[Implementação] Criar harness offline :: Executar contratos com transport/model fakes e fixture controlada. | sdk/testkit/ | CI de terceiro pode validar comportamento básico sem API paga ou documentos privados.
[Implementação] Migrar provider gerativo existente :: Encaixar adapter LocalAI/BYOK sem alterar política de egress. | adapters/generation/sdk_bridge/ | Capabilities e identidade de provedor continuam preservadas.
[Implementação] Migrar provider de extração :: Encaixar Docling/normalizador com mesmas restrições de I/O. | adapters/extraction/sdk_bridge/ | Plugin não obtém escrita canônica como resultado da refatoração.
[Implementação] Migrar interfaces de busca/contexto :: Expor backend escolhido e PageIndex adaptado sem vazar internals. | adapters/retrieval/sdk_bridge/ | IDs/revisões/escopo não mudam por empacotamento em SDK.
[Teste] Criar provider deliberadamente inválido :: Retornar dados fora de schema/escopo e exceder budget. | sdk/examples/invalid-provider/ | Host rejeita violações e coleta evidência sem aceitar conteúdo malicioso.
[Teste] Comparar pré e pós-SDK :: Rodar contratos e corpora de qualidade da versão estável. | reports/sdk/differential.json | Refatoração não se considera concluída apenas porque compila.
[Documentação] Publicar tutorial mínimo :: Explicar contrato, permissões, testes, licenças e submissão de integração. | docs/sdk/getting-started.md | Contribuidor consegue executar exemplo offline sem copiar internals de outro adapter.
''')

add(69,'Instalar extensões com permissões controladas','ExtensionHost / distribuição','Evolução proposta',['R06','R08','R09','R30','R33','R35'],['S68'],
'Adicionar extensões sem perder soberania, segurança ou previsibilidade operacional.','Instalação local/curada, revogação e lifecycle de extensões.','Instalar provedor de exemplo, negar rede, revogar e remover sem tocar dados autorais.',r'''
[Implementação] Validar pacote de extensão :: Checar manifesto, hashes, target e licença antes de extrair/executar. | crates/extensions/src/package.rs | Zip slip/paths externos e artefatos sem origem são rejeitados.
[Implementação] Criar fluxo de consentimento :: Mostrar dados recebidos, destino de rede e recursos pedidos. | apps/desktop/src/extensions/permissions.ts | Instalar não equivale a permitir acesso a todo Vault ou todas as APIs.
[Implementação] Vincular permissões a tarefa :: Gerar capabilities limitadas por workspace/revisão e operação. | crates/extensions/src/capabilities.rs | Extensão não pode ampliar autorização através da própria resposta.
[Implementação] Isolar execução :: Reutilizar perfis de sandbox adequados e recusar combinação não suportada. | crates/extensions/src/sandbox.rs | Falta de contenção não provoca execução irrestrita para manter compatibilidade.
[Implementação] Controlar rede por provedor :: Roteamento/destinos compatíveis com política já aprovada pelo usuário. | crates/extensions/src/egress.rs | Permissão local não habilita serviço remoto adicionado posteriormente pela extensão.
[Implementação] Gerenciar update e rollback :: Tratar mudança de permissões como nova aprovação. | crates/extensions/src/update.rs | Atualização não aumenta direitos silenciosamente nem apaga configuração funcional sem retorno.
[Implementação] Implementar revogação :: Cancelar jobs, invalidar tokens e impedir novas chamadas do provedor. | crates/extensions/src/revoke.rs | Jobs em andamento não continuam usando segredo após revogação sem controle.
[Implementação] Implementar remoção :: Encerrar processos e retirar pacote/cache preservando dados autorais. | crates/extensions/src/remove.rs | Remover provider não remove textos, summaries aceitos ou evidências do usuário.
[Teste] Testar abuso de recursos e dados :: Usar extensão de prova que tenta exfiltrar/forçar respostas excessivas. | tests/extensions/security/ | Host bloqueia violações dentro do modelo de ameaça documentado.
[Licença] Documentar distribuição curada :: Definir fonte correspondente, notices e limites de endosso de terceiros. | docs/extensions/distribution.md | Projeto não promete segurança automática de qualquer plugin disponível na internet.
''')

add(70,'Atualizar recortes e consolidar beta v2.0','Upstreams / SDK / formato','Integração',['R01','R04','R22','R30','R32','R33','R35'],['S68','S69'],
'Consolidar arquitetura madura sem exigir reescrita do Vault ou perda de compatibilidade.','v2.0.0-beta.1 com motores atualizados e adapters conformes.','Executar o fluxo completo usando adapters sob SDK e manter dados v1.x intactos.',r'''
[Upstream] Revisar revisões-alvo v2 :: Selecionar updates necessários com changelogs, licenças e riscos. | upstream/releases/v2-candidates.json | Não se usa alteração de major para atualizar tudo sem teste.
[Upstream] Rebasear BlockSuite :: Validar codecs, IDs, input e montagem mínima na nova revisão escolhida. | reports/upstream/v2-blocksuite.md | Editor não incorpora o produto AFFiNE ou canvas concorrente durante update.
[Upstream] Rebasear PageIndex :: Preservar portas de modelos/fontes, IDs e relações fora da árvore. | reports/upstream/v2-pageindex.md | Mudanças upstream não reintroduzem armazenamento canônico privado ou leitura global.
[Upstream] Revalidar ingest e web :: Rodar corpus Docling/Readability/Turndown/Scrapling e contrato SearxNG. | reports/upstream/v2-ingest-web.json | Formatos/engines quebrados geram degradação clara, não corrupção silenciosa.
[Upstream] Revalidar runtime e busca :: Avaliar LocalAI/backends e o banco escolhido, mantendo espaço de embeddings identificado. | reports/upstream/v2-runtime-search.json | Troca de modelo/índice exige caminho explícito; não há mistura de vetores.
[Implementação] Remover acoplamentos antigos :: Retirar adapters obsoletos somente após paridade e migração testadas. | refactors/v2/ | Nenhuma funcionalidade estável desaparece só porque a fachada mudou.
[Implementação] Versionar formato apenas se necessário :: Implementar migração real quando houver mudança autoral aprovada. | crates/migrations/src/v2.rs | Número v2.0 do app não força alteração gratuita no formato do Vault.
[Teste] Rodar pacote completo de conformidade :: Testar providers internos como se fossem integrações externas. | reports/v2-beta/provider-conformance.json | Nenhum provider privilegiado ignora regras que seriam exigidas de um terceiro.
[Licença] Fechar inventário beta :: Atualizar SBOM, fontes, modelos e manifests de extensão por alvo. | compliance/releases/v2.0.0-beta.1/ | Pacotes têm origem/licença correspondente à revisão efetivamente construída.
[Release] Publicar beta v2 :: Documentar SDK, limites, migração e retorno à linha v1 suportada. | release/v2.0.0-beta.1/ | Beta exige backup/consentimento quando houver mudança de formato; não usa Vault único sem proteção.
''')

add(71,'Homologar compatibilidade e candidata v2.0','Compatibilidade / RC v2','Próprio',['R01','R02','R05','R06','R07','R09','R32','R35'],['S70'],
'Comprovar que extensibilidade e evolução não romperam as promessas originais.','v2.0.0-rc.1 e matriz de upgrade/rollback completa.','Migrar cópias de Vaults v1.x, usar extensões e voltar/exportar sem perda autoral.',r'''
[Teste] Montar matriz histórica :: Incluir v1.0, v1.1, v1.2 e estados de recursos opcionais. | tests/migrations/v2-matrix.yaml | Cada origem possui fixture e expectativa de upgrade/downgrade/export explícita.
[Teste] Executar upgrades interrompidos :: Matar processo, simular disco cheio e pacote faltante durante migração. | reports/v2-rc/upgrade-faults.json | Estado pode ser recuperado sem confiar em índice vetorial ou extensão instalada.
[Teste] Testar rollback e export de emergência :: Validar retorno suportado e alternativa quando downgrade não for possível. | reports/v2-rc/rollback.json | Limitações são mostradas antes de migrar, não descobertas após perda de acesso.
[Teste] Repetir prova File-as-Truth :: Apagar índices, caches de extensão e stores internos dos motores. | reports/v2-rc/file-truth.json | Autoria, citações, topologia e histórico aprovados continuam recuperáveis.
[Teste] Repetir isolamento completo :: Incluir extensões, novos backends e políticas por plataforma. | reports/v2-rc/security.json | Refatoração do SDK não abriu rota alternativa para filesystem/rede.
[Teste] Repetir offline completo :: Provisionar pacotes e usar todos os caminhos locais homologados. | reports/v2-rc/offline.json | Extensão sem rede não tenta instalar/download por conta própria.
[Teste] Repetir performance/qualidade :: Comparar budgets, cenas, recuperação e resumos contra baselines estáveis. | reports/v2-rc/regression.json | Novos recursos não justificam regressão silenciosa nos requisitos essenciais.
[Teste] Fazer piloto de atualização :: Observar uso da candidata com dados permitidos e diagnóstico saneado. | reports/v2-rc/pilot.md | Feedback real e ensaio interno são identificados separadamente.
[Governança] Fechar blockers e compatibilidade :: Revisar breaking changes, deprecações e suporte da linha anterior. | docs/releases/v2-compatibility.md | Contribuidor/usuário sabe quais contratos mudaram e como migrar.
[Release] Publicar RC v2 :: Emitir pacotes, manifests e guia de migração final candidato. | release/v2.0.0-rc.1/ | Revisão da RC corresponde às evidências e está pronta para go/no-go humano.
''')

add(72,'Publicar v2.0 e estabelecer continuidade','Release / sustentabilidade','Próprio',['R01','R02','R05','R07','R30','R32','R33','R34','R35','R36'],['S71'],
'Fechar o horizonte com produto completo, mantível e aberto, não com um fim artificial do software.','v2.0.0 estável, SDK documentado e processo contínuo de manutenção.','Instalar stable, importar pesquisa, produzir Peça e verificar portabilidade/rollback de capacidades.',r'''
[Governança] Auditar escopo final :: Conferir requisitos originais e propostas aprovadas em ADR-017/018. | reports/releases/v2-traceability.md | Requisito não aprovado não é declarado entregue; requisito essencial faltante bloqueia stable.
[Teste] Rodar regressão final :: Executar suites em commit exato do release nos cinco alvos e perfis homologados. | reports/releases/v2-final-suite.json | Build, testes e artefatos compartilham identidade verificável.
[Teste] Verificar distribuição limpa :: Instalar sem toolchains globais, provisionar offline e testar update/removal. | reports/releases/v2-clean-install.md | Usuário não precisa reconstruir ambientes de desenvolvimento para usar o produto.
[Licença] Fechar conformidade v2 :: Revisar fontes/avisos, SDK, extensões curadas e modelos redistribuídos. | compliance/releases/v2.0.0/ | Natureza open source/nonprofit e dependência opcional de serviços são descritas corretamente.
[Documentação] Publicar documentação estável :: Consolidar usuário, formato, recuperação, SDK e contribuição. | docs/releases/v2.0.0.md | Guias refletem APIs/capacidades publicadas e não protótipos abandonados.
[Governança] Definir manutenção pós-v2 :: Estabelecer triagem, patches, cadence de auditoria e política de depreciação sustentável. | docs/maintenance/post-v2.md | Roadmap não promete última versão definitiva nem suporte impossível para um mantenedor.
[Governança] Preparar continuidade comunitária :: Documentar builds, acesso de release, backups de chaves e sucessão sem divulgar segredos. | docs/governance/continuity.md | Outro mantenedor pode reconstruir o projeto com procedimentos públicos e permissões apropriadas.
[Decisão] Fazer go/no-go final :: Revisar blockers, métricas e a demonstração integral com aprovação humana. | reports/releases/v2-go-no-go.md | Agente não pode marcar aprovação final sozinho; exceções materiais ficam registradas.
[Release] Publicar artefatos stable :: Disponibilizar versões, hashes/assinaturas, fontes e notas no canal escolhido. | release/v2.0.0/ | Downloads e documentação correspondem à mesma revisão; possibilidade de rollback do canal existe.
[Teste] Verificar publicação e handoff :: Instalar pelo canal real, checar links e registrar próximos riscos sem inventar novas features. | reports/releases/v2-post-publish.md | Horizonte concluído com evidências e fila de manutenção, não com todos os problemas futuros declarados resolvidos.
''')
