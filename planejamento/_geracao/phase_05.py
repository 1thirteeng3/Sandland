from plan_base import add

add(47,'Integrar SearxNG como serviço de descoberta','SearxNG / WebDiscoveryProvider','Desconstrução',['R08','R24','R30','R33'],['S23','S26','S34','S46'],
'Usar metabusca sem incorporar sua UI ou criar dependência pública implícita.','Provider SearxNG com configuração, consentimento e testes de resposta.','Pesquisar por consulta autorizada e apresentar resultados sem enviar o board inteiro.',r'''
[Upstream] Fixar origem SearxNG :: Registrar revisão, runtime Python, engines habilitados e obrigações AGPL. | upstream/searxng/manifest.json | Fonte correspondente e configuração distribuída são identificáveis.
[Decisão] Escolher instância e lifecycle :: Resolver ADR-015: local gerenciada ou instância pessoal explícita. | docs/adr/015-web-discovery.md | Instância pública não é fallback silencioso nem dependência obrigatória para abrir o app.
[Implementação] Criar configuração mínima :: Habilitar API JSON e apenas engines necessários; separar serviço de sua interface web. | workers/searxng/settings.yml | Formato não habilitado gera diagnóstico em vez de assumir que toda instância responde JSON.
[Implementação] Integrar supervisor :: Iniciar/verificar/encerrar serviço local quando essa modalidade for escolhida. | adapters/searxng/lifecycle.rs | Serviço não inicia no startup da autoria sem necessidade.
[Contrato] Definir resultado de descoberta :: Normalizar título, URL, snippet, engine, consulta e timestamp sem chamar isso de evidência lida. | schemas/web-search-result.json | Resultado preserva origem e não inventa conteúdo da página completa.
[Implementação] Implementar cliente :: Tratar idioma, filtros, timeout, cancelamento e falhas parciais de engines. | adapters/searxng/client.rs | Uma engine bloqueada não faz loop infinito nem retorna sucesso vazio sem contexto.
[Implementação] Aplicar política de consulta :: Autorizar texto enviado, registrar finalidade e bloquear contexto sensível não aprovado. | crates/web/src/query_policy.rs | Consulta gerada pela IA é inspecionável; a mesa inteira não é enviada automaticamente.
[Implementação] Deduplicar resultados :: Canonicalizar URLs com cuidado, preservando versões/origens relevantes. | crates/web/src/result_merge.rs | Parâmetros semanticamente relevantes não são removidos indiscriminadamente.
[Teste] Criar API fake e smoke autorizado :: Cobrir 403 JSON, indisponibilidade, respostas parciais e uma pesquisa real consentida. | tests/web/searxng/ | CI determinística não depende de buscadores públicos; teste real tem estado separado.
[Licença] Preparar distribuição e avisos :: Incluir fontes/configuração e documentação da divulgação de consultas externas. | compliance/searxng/ | Auto-hospedagem não é anunciada como busca offline ou anonimato absoluto.
''')

add(48,'Integrar Scrapling no caminho HTTP','Scrapling / WebFetchProvider','Desconstrução',['R06','R08','R25','R30'],['S23','S26','S29','S47'],
'Adquirir páginas com limites e origem preservada antes de habilitar um browser.','Worker HTTP Scrapling e política de destinos.','Baixar página permitida e negar URL interna/redirect indevido em harness controlado.',r'''
[Upstream] Fixar Scrapling e dependências :: Registrar fetchers/parser necessários e excluir inicialmente extras de browser/crawl. | upstream/scrapling/manifest.json | Pacote HTTP não instala navegador por simples import sem política aprovada.
[Upstream] Reproduzir baseline HTTP :: Exercitar requests, redirecionamentos e HTML de fixture sem customização. | reports/scrapling/http-baseline.md | Mudança de comportamento no adapter pode ser comparada à revisão original.
[Contrato] Definir FetchRequest/Snapshot :: Incluir URL autorizada, limites, headers permitidos, cadeia de redirect e bytes/hash. | schemas/web-fetch-v0.json | Snapshot diferencia URL solicitada, final e data de aquisição.
[Implementação] Implementar política SSRF :: Restringir schemes, destinos, DNS e redes privadas conforme autorização. | crates/web/src/url_policy.rs | localhost, metadados, file URLs e destinos não permitidos são rejeitados.
[Implementação] Validar cada redirecionamento :: Reaplicar política e tratamento de resolução antes de seguir destino. | workers/scrapling/redirect_policy.py | URL pública que redireciona ao runtime local não contorna o bloqueio.
[Implementação] Aplicar limites de resposta :: Controlar bytes descomprimidos, tempo, encoding e tipo detectado. | workers/scrapling/http_fetch.py | Resposta infinita/comprimida maliciosa é interrompida sem esgotar recursos do Core.
[Implementação] Integrar sessões restritas :: Isolar cookies/credenciais por contexto e não herdar perfil pessoal do navegador. | workers/scrapling/sessions.py | Pesquisa pública não ganha cookies de outros workspaces ou da sessão do usuário.
[Implementação] Encaminhar conteúdo adquirido :: Enviar HTML ao normalizador e arquivos ao Ingest por propostas validadas. | crates/web/src/fetch_pipeline.rs | Worker não grava snapshots diretamente na área canônica.
[Teste] Exercitar corpus de rede hostil :: Simular rebinding/redirects, tipos enganosos, timeout e resposta excessiva. | tests/web/http-security/ | Todas as exceções de destino são explicitamente autorizadas e auditáveis.
[Teste] Medir HTTP-first :: Comparar sucesso/falha/custo no corpus permitido sem prometer taxa universal. | reports/scrapling/http-eval.json | Falhas são exibidas; não se declara resolver 85% da web sem evidência correspondente.
''')

add(49,'Adicionar aquisição dinâmica com browser confinado','Scrapling / browser','Adaptação',['R01','R06','R08','R09','R25','R30'],['S24','S25','S26','S48'],
'Habilitar páginas dinâmicas sem transformar o app em um navegador irrestrito.','Browser worker lazy com política de rede e lifecycle.','Renderizar SPA de fixture, capturar DOM e encerrar todos os processos após a tarefa.',r'''
[Decisão] Definir critérios de escalada :: Escalar HTTP→browser por falha/tipo real, com consentimento e limites. | docs/web/browser-policy.md | Bloqueio não dispara downloads ou tentativas ilimitadas de bypass.
[Implementação] Provisionar browser homologado :: Fixar versão compatível com motor, assinatura/hash e artefatos por SO. | browser/manifests/ | Instalação é consentida e verificável; browser não é baixado de URL fornecida pelo modelo.
[Implementação] Criar sessão efêmera :: Isolar profile, cache, cookies e downloads por tarefa ou escopo aprovado. | workers/scrapling/browser_session.py | Perfil pessoal e segredos locais não são compartilhados.
[Implementação] Aplicar contenção real :: Conectar perfis por SO à árvore multiprocesso do navegador. | workers/scrapling/browser_sandbox.py | Não se usa no-sandbox como correção silenciosa; capacidade indisponível permanece bloqueada.
[Implementação] Controlar requisições do browser :: Abranger frames, subresources, WebSockets e redirecionamentos relevantes. | workers/scrapling/browser_network.py | JavaScript da página não alcança LocalAI/SearxNG privados nem redes proibidas por caminho alternativo.
[Implementação] Capturar DOM e estado :: Definir condição de prontidão, timeout e resultado parcial. | workers/scrapling/dynamic_fetch.py | Espera por network idle não pode bloquear indefinidamente em página que mantém conexões.
[Implementação] Controlar downloads/ações :: Permitir somente aquisição aprovada, sem upload, shell ou automação de contas. | workers/scrapling/browser_actions.py | Conteúdo remoto não solicita acesso ao Vault ou credenciais por tool call.
[Implementação] Compartilhar pool limitado :: Reusar recursos quando seguro e fechar sessões ociosas. | crates/web/src/browser_pool.rs | Número de widgets não cria número igual de browsers persistentes.
[Teste] Testar páginas hostis e crash :: Simular popup, arquivo de download, loop JS, processo filho e rede interna. | tests/web/browser-security/ | Cancelar mata a árvore autorizada e preserva somente resultados validados.
[Teste] Homologar consumo por plataforma :: Medir pico/idle e suporte das capacidades em alvos reais. | reports/browser/qualification.json | Capacidade só é liberada onde sandbox e budgets do perfil foram aprovados.
''')

add(50,'Construir widget de pesquisa e snapshots','Whiteboard / pesquisa','Próprio',['R11','R14','R24','R25','R26','R31'],['S19','S30','S47','S48','S49'],
'Conectar descoberta à Mesa sem confundir página live, snapshot e síntese autoral.','Widget pesquisável, snapshots e promoção de resultados a células.','Pesquisar, selecionar fontes, capturar, editar síntese e atualizar sem perda autoral.',r'''
[Contrato] Definir estado canônico do widget :: Persistir consulta, filtros, resultados escolhidos, execuções e revisão de síntese. | schemas/research-widget-v0.json | Estado pode ser reconstruído sem browser aberto ou cache do serviço.
[Implementação] Criar UI de consulta :: Exibir destino/consentimento, filtros e execução cancelável. | apps/desktop/src/widgets/research/query.ts | Consulta privada não é enviada ao digitar sem ação/política explícita.
[Implementação] Renderizar resultados seguros :: Mostrar títulos/snippets saneados e origem sem executar HTML. | apps/desktop/src/widgets/research/results.ts | Resultado de buscador não recebe aparência de texto completo já verificado.
[Implementação] Selecionar e adquirir fontes :: Disparar fetch de itens aprovados e mostrar status individual. | crates/widgets/src/research/acquire.rs | Falha de uma fonte não perde seleção nem captura das demais.
[Implementação] Persistir snapshots :: Salvar HTML/texto/assets autorizados com URL final, data, hash e mapa de fontes. | crates/widgets/src/research/snapshot.rs | Consulta posterior à web não altera o snapshot citado.
[Implementação] Promover a Ingest/células :: Criar recorte ou referência de material sem duplicação destrutiva. | crates/widgets/src/research/promote.rs | Conteúdo e origem chegam ao mesmo protocolo de proveniência usado na Peça.
[Implementação] Integrar síntese opcional :: Usar fontes adquiridas e Context/GenerationProvider, não apenas snippets. | crates/widgets/src/research/summarize.rs | Afirmações indicam quais fontes foram efetivamente lidas e cobertura parcial.
[Implementação] Tratar atualização :: Comparar resultados/snapshots novos com síntese existente. | apps/desktop/src/widgets/research/refresh.ts | Atualização não sobrescreve texto editado nem citações anteriores silenciosamente.
[Teste] Validar uso offline :: Abrir widget salvo e navegar pelas fontes preservadas com rede bloqueada. | tests/widgets/research-offline/ | Pesquisa nova fica indisponível com explicação; leitura de snapshots continua funcional.
[Teste] Executar jornada de pesquisa :: Buscar→capturar→recortar→conectar→inserir na Peça e verificar fonte/revisão. | reports/widget/research-journey.md | Cadeia integral permanece válida após reinício e limpeza de caches.
''')

add(51,'Integrar copiloto editorial e BYOK','Copiloto / GenerationProvider','Integração',['R08','R13','R23','R27','R31'],['S21','S32','S46','S50'],
'Gerar propostas de escrita contextualizadas sem conceder edição autônoma irrestrita.','Copiloto local/BYOK com diff, aceite e rollback.','Solicitar revisão de parágrafo, comparar proposta e aceitar somente blocos selecionados.',r'''
[Contrato] Definir ações editoriais :: Reescrever, resumir, criticar e expandir como propostas com revisão-base. | schemas/editorial-proposal-v0.json | Modelo não recebe operação genérica de substituir arquivo ou executar comando.
[Implementação] Criar adapters BYOK :: Integrar protocolos dos provedores aprovados preservando capabilities distintas. | adapters/generation/remote/ | Compatibilidade OpenAI não é presumida para toda API; ausência de tools/schema é explícita.
[Implementação] Gerenciar segredos e consentimento :: Aplicar escopo da Peça/fontes e armazenar chave no SO. | crates/model-gateway/src/remote_policy.rs | Trocar para remoto não envia automaticamente o contexto inteiro usado no modo local.
[Implementação] Montar contexto editorial :: Combinar seleção, objetivos e evidências autorizadas sob budget. | crates/copilot/src/context.rs | Conteúdo fora do escopo não entra por link/citação indireta sem autorização.
[Implementação] Executar geração cancelável :: Exibir streaming e estado parcial sem aplicar texto durante geração. | crates/copilot/src/session.rs | Cancelar mantém a Peça original e não registra proposta parcial como aceita.
[Implementação] Criar diff por blocos :: Mostrar adições/remoções e referências alteradas. | apps/desktop/src/copilot/diff.ts | Usuário distingue mudança de argumento de simples formatação.
[Implementação] Aplicar proposta atomicamente :: Validar base_revision e permitir aceite parcial conforme contrato. | crates/copilot/src/apply.rs | Peça editada durante geração gera conflito/rebase explícito, não overwrite.
[Implementação] Registrar proveniência de IA :: Gravar provedor/modelo e evento aceito sem expor segredos. | crates/copilot/src/provenance.rs | IA é marcada como origem da proposta, não fonte independente do conteúdo factual.
[Teste] Testar falhas e injeção :: Cobrir resposta sem referência, patch inválido, prompt hostil e provedor offline. | tests/copilot/ | Nenhuma falha concede escrita fora do bloco/escopo autorizado.
[Teste] Validar revisão e rollback :: Aceitar parte, desfazer e reconstruir histórico em modo offline. | reports/copilot/editorial-acceptance.md | Ação autoral é recuperável e o resultado não depende de conversa hospedada externamente.
''')

add(52,'Implementar intenções, chat global e sugestões','IntentService / chat','Próprio',['R08','R18','R21','R23','R28'],['S35','S41','S46','S51'],
'Conectar metas ao acervo sem ampliar o escopo de agentes implicitamente.','Intenções canônicas, chat contextual e sugestões revisáveis.','Material novo relevante produz sugestão explicável que só cria/altera board após confirmação.',r'''
[Contrato] Definir intenções :: Persistir objetivos, hipóteses, temas, estado e limites de escopo. | schemas/intention-v0.json | Intenção possui identidade/revisão e não existe apenas em memória de chat.
[Implementação] Criar UI de metas :: Editar, pausar e arquivar intenções com linguagem clara. | apps/desktop/src/intentions/ | Pausar intenção interrompe novos jobs proativos sem apagar seus dados.
[Implementação] Recuperar afinidades :: Usar busca aprovada para selecionar materiais candidatos por intenção. | crates/intentions/src/candidates.rs | Threshold antigo de 0,85 não é aplicado sem calibração para o modelo escolhido.
[Implementação] Avaliar relevância opcional :: Usar Jev ou alternativa local conforme política e registrar critério. | crates/intentions/src/evaluate.rs | Intenção sensível não é enviada remotamente apenas porque Jev está configurado globalmente.
[Implementação] Deduplicar notificações :: Aplicar cooldown, motivo e limite de volume sem gerar spam. | crates/intentions/src/notifications.rs | Mesmo item reindexado não dispara sugestões duplicadas incessantemente.
[Implementação] Criar proposta de board :: Apresentar candidatos, relações sugeridas e fontes para confirmação. | crates/intentions/src/board_proposal.rs | Arestas propostas por IA não aparecem como desenhadas pelo usuário antes do aceite.
[Implementação] Criar chat global com escopo :: Selecionar workspace/acervo permitido e mostrar o escopo ativo. | apps/desktop/src/chat/ | Global não significa acesso irrestrito automático a todos os arquivos do computador.
[Implementação] Persistir conversas/propostas :: Definir o que é histórico autoral versus execução efêmera. | crates/chat/src/history.rs | Conversa importante pode ser exportada; logs internos sensíveis não viram conteúdo do Vault por acidente.
[Teste] Testar mudança de intenção/escopo :: Alterar meta e revogar acesso enquanto job está em execução. | tests/intentions/scope/ | Resultado antigo é rejeitado/reclassificado e não publica sugestão desautorizada.
[Teste] Avaliar utilidade e explicação :: Usar corpus de intenções com positivos/negativos e feedback do usuário. | reports/intentions/evaluation.md | Sugestões exibem motivo/evidências e podem ser rejeitadas sem reorganização forçada do acervo.
''')

add(53,'Integrar o ciclo completo e congelar funcionalidades','Produto / alpha integrada','Integração',['R02','R07','R11','R13','R14','R26','R28','R34'],['S30','S36','S41','S46','S50','S51','S52'],
'Consolidar os motores numa experiência única antes da fase de homologação.','v0.9.0 alpha do ciclo analítico completo e inventário de lacunas.','Capturar fonte, pesquisar, conectar, produzir Peça e comprovar origem e reconstrução.',r'''
[Implementação] Unificar IDs e navegação :: Resolver links entre Ingest, widget, célula, Peça e histórico por contrato comum. | crates/app-core/src/navigation.rs | Links não dependem de nomes de tela ou paths absolutos frágeis.
[Implementação] Harmonizar estados de operação :: Padronizar fila, erro, offline, revisão antiga e solicitação de permissão. | apps/desktop/src/status-system/ | Mesmo estado não recebe significados contraditórios em fases diferentes.
[Implementação] Revisar fluxos de promoção :: Tornar clara a diferença entre referência, recorte, cópia e fork-on-insert. | apps/desktop/src/flows/promote/ | Usuário consegue prever quais conteúdos serão independentes depois de cada ação.
[Implementação] Unificar painel de recursos :: Mostrar runtime/modelos/jobs e permitir pausar tarefas de fundo. | apps/desktop/src/settings/resources/ | Diagnóstico inclui workers e browser, não apenas contador do frontend.
[Teste] Rodar jornada integral local :: Usar pacote de dados/modelos provisionado e rede bloqueada. | reports/v0.9/local-journey.md | Autoria, busca e contexto locais funcionam; web/Jev/BYOK mostram indisponibilidade sem fallback oculto.
[Teste] Rodar jornada conectada consentida :: Habilitar pesquisa/Jev/BYOK separadamente e registrar egress autorizado. | reports/v0.9/connected-journey.md | Ativar um provedor não autoriza todos os demais automaticamente.
[Teste] Apagar caches durante ensaio seguro :: Reabrir, reconstruir e comparar autoria/proveniência. | reports/v0.9/reconstruction.md | Nenhum motor contém a única cópia de conhecimento aceito.
[Governança] Congelar funcionalidades v1 :: Inventariar requisitos ainda pendentes de homologação/portabilidade. | docs/release/v1-freeze.md | Lacunas não são renomeadas como feature pronta; escopo novo vai para proposta pós-v1.
[Governança] Triar defeitos :: Classificar integridade/segurança, função essencial, recursos e polimento. | reports/v0.9/defect-register.csv | Defeito crítico tem owner e bloqueio; não é adiado a v2 para concluir v1 artificialmente.
[Release] Publicar alpha integrada :: Distribuir somente capacidades testadas e pedir feedback com diagnóstico saneado. | release/v0.9.0/ | Alpha não é anunciada como release estável ou como prova de todas as metas do RFC.
''')

add(54,'Executar hardening e auditoria adversarial','Segurança / todos os motores','Próprio',['R05','R06','R08','R30','R34'],['S53'],
'Testar composição real, não apenas segurança isolada de cada adapter.','Relatório de segurança com correções bloqueadoras concluídas.','Ataques de arquivo, web, prompt e processo falham sem afetar outros workspaces.',r'''
[Teste] Revalidar modelo de ameaças :: Atualizar superfícies após todas as integrações e revisar privilégios efetivos. | docs/security/threat-model-v1.md | Nenhum runtime/endpoint novo fica fora do inventário de fronteiras.
[Teste] Fuzzar codecs/protocolos :: Cobrir Markdown, YAML, JSON, MPK, respostas de workers e patches editoriais. | tests/fuzz/ | Crashes e consumo excessivo reproduzíveis viram regressões, não arquivos descartados sem análise.
[Teste] Repetir ataques de filesystem :: Executar TOCTOU/junction/symlink nas operações reais dos motores. | reports/security/fs-integration.json | Proteção do Core não é contornada por parser/browser com acesso direto amplo.
[Teste] Repetir ataques de rede :: Testar SSRF, serviços locais, redirects e subrequests de browser. | reports/security/network-integration.json | LocalAI, SearxNG e segredos não são alcançados por conteúdo adquirido sem autorização.
[Teste] Red-team de prompt injection :: Misturar instruções hostis com documentos/citações e propostas de geração. | reports/security/prompt-injection.json | Ataque pode afetar texto gerado, mas não altera permissões nem executa ações proibidas.
[Teste] Auditar credenciais/logs :: Procurar chaves, conteúdos privados e metadados sensíveis em exports e diagnóstico. | reports/security/secrets.json | Dados-semente não vazam; logging detalhado exige opt-in e saneamento.
[Implementação] Corrigir vulnerabilidades encontradas :: Priorizar causa raiz e adicionar teste que falha antes do patch. | security/fixes/ | Achado crítico não fica apenas documentado como risco aceito para publicar estável.
[Upstream] Auditar dependências e patches :: Conferir avisos, CVEs relevantes e necessidade de atualização controlada. | compliance/security-review-v1.json | Atualização passa regressões; ausência de CVE não é tratada como garantia de segurança.
[Teste] Validar fail-closed :: Remover mecanismo obrigatório de sandbox e tentar iniciar recurso dependente. | tests/security/missing-capability/ | Função é bloqueada claramente em vez de rodar sem contenção.
[Governança] Fechar gate de segurança :: Revisar evidências manualmente e registrar limites residuais não críticos. | reports/security/v1-signoff.md | Aprovação identifica exatamente versões/perfis testados e não afirma segurança absoluta.
''')

add(55,'Homologar performance e degradação por hardware','ResourceSupervisor / benchmarks','Próprio',['R09','R12','R20','R23','R34'],['S54'],
'Medir os budgets do RFC no aplicativo completo e corrigir gargalos com evidência.','Perfis de desempenho homologados e política de degradação.','Repetir benchmarks com dados brutos e observar retorno real a idle após tarefas pesadas.',r'''
[Teste] Fixar ambiente de homologação :: Registrar hardware, SO, drivers, armazenamento, dataset e versões dos modelos. | bench/profiles/v1.yaml | Resultado de VM/CI não é confundido com perfil de desktop real.
[Teste] Medir idle agregado :: Contar Core, WebView, GPU/auxiliares pertinentes e serviços sob gestão. | reports/perf/idle-v1.json | Meta de 350 MB é avaliada no perfil aprovado, sem omitir processos pesados.
[Teste] Medir startup e ack :: Distinguir primeiro uso, caches quentes/frios e aceitação versus conclusão de comando. | reports/perf/startup-ipc-v1.json | Meta de 2,2 s/150 ms possui definição e dados por percentil.
[Teste] Medir indexação leve :: Validar 650 MB no perfil definido e registrar OCR/STT separadamente. | reports/perf/indexing-v1.json | Pipelines pesados não recebem exceção implícita ao requisito.
[Teste] Medir inferência e unload :: Avaliar 2,8 GB, contexto, simultaneidade e liberação após ociosidade. | reports/perf/inference-v1.json | Pesos, KV/cache e runtime são contabilizados; não há claim baseado só no tamanho GGUF.
[Teste] Medir cena complexa :: Coletar frame time em 100/1.000 nós com textos/imagens/arestas. | reports/perf/canvas-v1.json | LOD não ganha desempenho apagando dados ou desativando interação sem aviso.
[Implementação] Corrigir gargalos demonstrados :: Atacar hotspots de startup, renderer, indexador e serialização identificados no profiler. | perf/fixes-v1/ | Otimização mantém suítes funcionais e mostra comparação antes/depois.
[Implementação] Aplicar backpressure :: Pausar/reduzir jobs quando recursos se aproximam do perfil autorizado. | crates/resources/src/backpressure.rs | UI continua utilizável e cancelamento tem efeito; não há thrashing contínuo de modelos.
[Implementação] Explicar degradação :: Informar capacidade indisponível, pacote menor ou tarefa pendente sem fallback externo. | apps/desktop/src/resources/degradation.ts | Hardware insuficiente não dispara nuvem nem altera conteúdo por truncamento oculto.
[Decisão] Ratificar perfil final :: Corrigir até cumprir ou reabrir ADR-007 explicitamente antes do release. | reports/perf/v1-signoff.md | Meta não atingida não recebe selo verde; desvios não são escondidos no changelog.
''')

add(56,'Empacotar e validar Linux e Windows','Distribuição / Linux / Windows','Integração',['R01','R06','R09','R30','R33'],['S54','S55'],
'Produzir pacotes instaláveis sem ambientes de desenvolvimento globais.','Pacotes Linux x86_64/aarch64 e Windows x86_64 com capacidades declaradas.','Instalar em máquina limpa, provisionar perfil permitido, usar e remover o app.',r'''
[Implementação] Definir layout de pacotes :: Separar app, runtimes, modelos e pacotes opcionais por arquitetura. | packaging/layout-v1.json | Pacote não depende de arquivos presentes apenas na máquina de build.
[Implementação] Construir Linux x86_64 :: Empacotar Core/UI e workers homologados com dependências nativas. | packaging/linux/x86_64/ | Instalação limpa roda autoria e capacidades declaradas sem toolchains globais.
[Implementação] Construir Linux aarch64 :: Ajustar bibliotecas/backends e não reutilizar binários x86 por engano. | packaging/linux/aarch64/ | Smoke nativo e verificação de arquitetura passam nos componentes distribuídos.
[Implementação] Construir Windows x86_64 :: Empacotar dependências, helpers e WebView conforme política aprovada. | packaging/windows/x86_64/ | Não exige privilégio ou runtime inesperado sem documentação/instalação consentida.
[Teste] Validar isolamento instalado :: Repetir testes essenciais de sandbox no pacote, não só no ambiente dev. | reports/packaging/linux-windows-security.json | Paths finais, ACLs e helpers preservam as garantias aprovadas.
[Teste] Validar instalação/atualização/remoção :: Testar paths Unicode, usuário sem admin e reinstalação. | tests/packaging/linux-windows/ | Remoção não apaga Vaults pessoais sem confirmação explícita.
[Teste] Validar pacotes opcionais offline :: Instalar por arquivos/manifestos locais com rede bloqueada. | reports/packaging/offline-linux-windows.json | Capacidades funcionam sem download tardio oculto.
[Licença] Gerar SBOM e avisos :: Inventariar binários, Python privado, browser, DLLs e modelos incluídos. | compliance/packages/linux-windows/ | Licenças/fontes correspondentes acompanham os artefatos efetivamente distribuídos.
[Teste] Rodar jornadas nativas :: Repetir Captura→Mesa→Peça, busca/contexto e web nos perfis habilitados. | reports/targets/linux-windows-v1.json | Build verde não substitui execução real nos três alvos.
[Release] Produzir artefatos beta :: Assinar/verificar conforme política e registrar hashes de cada pacote. | release/v1.0.0-beta.1/linux-windows/ | Usuário consegue verificar origem; limitações de assinatura são explícitas, não bypass silencioso.
''')

add(57,'Empacotar macOS e fechar beta multiplataforma','Distribuição / macOS','Integração',['R01','R06','R09','R30','R33'],['S54','S55','S56'],
'Validar Apple Silicon/Intel e consolidar a matriz real de capacidades.','v1.0.0-beta.1 para os cinco alvos previstos.','Instalação limpa em macOS preserva helpers, permissões, modelos e dados do usuário.',r'''
[Implementação] Construir Apple Silicon :: Empacotar binários/model backends compatíveis e paths de recursos. | packaging/macos/arm64/ | Nenhum backend x86 é carregado implicitamente para mascarar falta de suporte.
[Implementação] Construir macOS Intel :: Definir modelos/backends CPU/GPU realmente disponíveis. | packaging/macos/x86_64/ | Aceleração indisponível não é prometida por paridade com Apple Silicon.
[Implementação] Configurar helpers/assinatura :: Aplicar entitlements e política de distribuição aprovada a todos os executáveis. | packaging/macos/signing/ | Helper não perde contenção ao ser movido para o bundle final.
[Teste] Validar instalação limpa :: Executar sem ambientes Python/Go/Rust de desenvolvimento. | tests/packaging/macos-install/ | Ausência de dependência é detectada antes do uso, sem instrução opaca para desativar segurança.
[Teste] Repetir sandbox instalado :: Tentar acesso a dados privados, rede e filhos não autorizados. | reports/packaging/macos-security.json | Permissões reais do bundle correspondem aos perfis aprovados.
[Teste] Validar modelos/pacotes offline :: Provisionar pelo fluxo documentado e bloquear rede. | reports/packaging/macos-offline.json | Geração/embeddings homologados funcionam sem chamada externa.
[Teste] Rodar jornadas por arquitetura :: Exercitar editor, Mesa, ingest, busca, contexto e web habilitada. | reports/targets/macos-v1.json | Teste de uma arquitetura não vale como aprovação da outra.
[Teste] Consolidar matriz dos cinco alvos :: Comparar capacidades essenciais e opcionais com evidências. | reports/targets/v1-capabilities.json | Qualquer lacuna essencial bloqueia stable até correção ou replanejamento formal de escopo.
[Documentação] Publicar guia beta :: Explicar instalação, requisitos, limites e coleta de diagnóstico local saneado. | docs/releases/v1-beta.md | Usuário identifica facilmente qual pacote/modelo pode usar em seu hardware.
[Release] Publicar beta multiplataforma :: Empacotar notas, SBOM, hashes e fontes aplicáveis. | release/v1.0.0-beta.1/ | Beta mantém estado experimental explícito e não oculta falhas conhecidas.
''')

add(58,'Implementar migrações, backup e Git opcional','Compatibilidade / portabilidade','Próprio',['R02','R05','R29','R32','R37'],['S22','S57'],
'Tornar evolução e cópia dos dados seguras antes do primeiro stable.','Migração transacional, backup restaurável e integração Git consentida.','Migrar cópia de Vault antigo, restaurar backup e fazer commit/push apenas por ação autorizada.',r'''
[Decisão] Ratificar retenção/Git/auditoria :: Resolver ADR-016 incluindo arquivos canônicos, caches excluídos e limites de anchoring. | docs/adr/016-history-portability.md | Git opcional não é chamado de backup remoto sem configuração real.
[Implementação] Criar planejador de migração :: Detectar versão e mostrar plano antes de executar mudanças. | crates/migrations/src/plan.rs | Versão desconhecida abre read-only ou é recusada sem regravação destrutiva.
[Implementação] Executar migração recuperável :: Criar ponto de restauração e publicar migração com protocolo durável. | crates/migrations/src/run.rs | Interrupção permite retomar/voltar; formatos intermediários não são anunciados como prontos.
[Implementação] Versionar caches separadamente :: Reconstruir índices/MPK sem obrigar migração autoral. | crates/migrations/src/derived.rs | Atualização de engine vetorial não muda o Markdown sem necessidade.
[Implementação] Concluir backup/restauração :: Validar escopo, manifest, integridade e dry-run do restauro. | crates/backup/ | Backup de dados confirmados reconstrói autoria e histórico aprovado numa raiz limpa.
[Implementação] Integrar Git local :: Commitar arquivos autorizados com hooks/filters e execução de comandos sob política segura. | adapters/git/local.rs | Repositório importado não executa hook/filtro arbitrário por simples abertura.
[Implementação] Integrar remotos opt-in :: Configurar credenciais do SO, preview de conteúdo e confirmação de push/pull. | adapters/git/remote.rs | Nenhum remoto é criado ou sincronizado automaticamente; conflitos são explícitos.
[Implementação] Definir retenção e purga :: Separar limpar cache de apagar histórico/assets não referenciados. | crates/history/src/retention.rs | Purga com efeito autoral exige confirmação e explica perda de rollback/auditoria.
[Teste] Testar matriz de versões :: Migrar fixtures de versões anteriores e simular downgrade incompatível. | tests/migrations/v1/ | Dados não são silenciosamente perdidos; impossibilidade de downgrade tem alternativa de export/restore.
[Teste] Restaurar em outro sistema :: Levar backup entre SOs e verificar IDs, fontes, paths e referências. | reports/portability/v1-restore.md | Portabilidade não depende de índices ou diretórios específicos da máquina de origem.
''')

add(59,'Concluir acessibilidade, documentação e piloto','Produto / qualidade de uso','Próprio',['R01','R07','R08','R31','R33','R34'],['S57','S58'],
'Validar que o produto pode ser utilizado e mantido sem conhecimento de seus motores internos.','v1.0.0-rc.1 com documentação e evidências de piloto.','Usuário novo instala, captura, produz Peça e recupera erro seguindo somente os guias.',r'''
[Teste] Auditar teclado/foco :: Cobrir Ingest, canvas alternativo acessível, editor, diálogo e copiloto. | reports/a11y/keyboard-v1.md | Jornada essencial não contém armadilha de foco ou ação somente por gesto espacial.
[Teste] Auditar semântica/contraste :: Verificar nomes acessíveis, status, escalas e alternativas aos sinais de cor. | reports/a11y/interface-v1.md | Erros e relações não dependem exclusivamente de cor ou imagem.
[Implementação] Corrigir bloqueios de uso :: Resolver defeitos críticos de navegação, estados vazios e mensagens de recuperação. | ux/fixes-v1/ | Erro comum oferece ação concreta sem expor jargão interno obrigatório.
[Documentação] Escrever guia de primeiros passos :: Explicar modos offline/conectado e ciclo Captura→Mesa→Peça. | docs/user/getting-started.md | Guia não exige familiaridade com PageIndex, LocalAI, SearxNG ou banco vetorial.
[Documentação] Escrever guia de dados/recuperação :: Documentar formatos, backup, conflito, migração e reconstrução. | docs/user/data-and-recovery.md | Usuário distingue cache descartável de histórico autoral.
[Documentação] Escrever guia de privacidade :: Mostrar envio Jev/BYOK/web, chaves, logs e revogação. | docs/user/privacy.md | Não promete que todos os dados ficam locais quando o usuário habilita serviços externos.
[Documentação] Escrever guia de contribuição :: Orientar builds, fixtures, contratos, patches upstream e licenças. | docs/contributing/ | Novo contribuinte consegue executar suíte básica sem credenciais pagas.
[Teste] Executar piloto com corpus permitido :: Observar jornadas reais ou testes estruturados do mantenedor, distinguindo as modalidades. | reports/pilot/v1.md | Feedback não é inventado; origem e limitações da amostra são registradas.
[Governança] Revisar pendências da RC :: Classificar bugs, segurança, portabilidade e recursos contra critérios fixados. | reports/releases/v1-rc-readiness.md | Nenhum defeito crítico fica escondido como trabalho de versão futura.
[Release] Publicar candidata :: Anexar migrações, rollback, changelog, avisos e matriz de suporte. | release/v1.0.0-rc.1/ | Fonte e binários correspondem à mesma revisão e evidências da RC.
''')

add(60,'Publicar Sandland v1.0.0','Release / escopo integral','Próprio',['R01','R02','R05','R06','R07','R09','R33','R34'],['S54','S55','S56','S57','S58','S59'],
'Promover a stable somente o escopo aprovado e demonstrado.','v1.0.0 estável, conjunto de evidências e política de manutenção.','Instalar stable limpa, restaurar Vault de referência e concluir ciclo completo sem perda de dados.',r'''
[Governança] Auditar rastreabilidade :: Conferir requisitos R01–R34/R37 aplicáveis contra testes e entregáveis reais. | reports/releases/v1-traceability.md | Cada requisito essencial tem evidência, e não apenas tarefa marcada concluída.
[Teste] Rodar suíte final na revisão candidata :: Reexecutar contratos, e2e, segurança, migrações e recursos relevantes. | reports/releases/v1-final-suite.json | Resultados pertencem ao commit que será publicado, não a uma versão anterior parecida.
[Teste] Repetir destruição de caches :: Executar prova oficial de soberania no pacote final. | reports/releases/v1-file-truth.json | Todos os dados autorais previstos reaparecem sem depender dos motores de índice.
[Teste] Repetir modo desconectado :: Bloquear rede em instalação com pacotes provisionados. | reports/releases/v1-offline.json | Funções locais essenciais funcionam e não há egress inesperado.
[Licença] Fechar conformidade :: Revisar SBOM, notices, fontes correspondentes e licenças dos modelos distribuídos. | compliance/releases/v1.0.0/ | Natureza não comercial não é usada como isenção de obrigação ou custo de API.
[Release] Congelar e verificar artefatos :: Gerar hashes/assinaturas, registrar ambiente e associar fontes/lockfiles. | release/v1.0.0/manifest.json | Cada arquivo baixável corresponde a origem e versão auditáveis.
[Documentação] Publicar notas de versão :: Explicar capacidades, mínimos, limitações, atualização e rollback. | docs/releases/v1.0.0.md | Não há claim de performance/segurança que exceda a matriz homologada.
[Governança] Definir suporte e vulnerabilidades :: Criar canal, política de triagem e processo de patch emergencial. | SECURITY.md | Relato crítico pode interromper roadmap e gerar patch sem aguardar próximo sprint de feature.
[Decisão] Fazer go/no-go humano :: Revisar blockers, demonstração e disponibilidade de artefatos. | reports/releases/v1-go-no-go.md | Autor assina decisão; agente não aprova automaticamente a própria implementação.
[Release] Publicar stable e verificar distribuição :: Testar download/instalação a partir do canal real sem expor credenciais. | reports/releases/v1-post-publish.md | Se artefato estiver incorreto, canal é corrigido/revertido e problema não é ignorado.
''')
