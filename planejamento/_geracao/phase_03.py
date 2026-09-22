from plan_base import add

add(23,'Construir protocolo e supervisor de workers','JobScheduler / WorkerHost','Próprio',['R03','R05','R06','R08','R09'],['S09','S11','S22'],
'Permitir motores poliglotas sem conceder autoridade sobre o Vault.','WorkerHost, fila persistível e protocolo versionado.','Worker de teste recebe snapshot autorizado, falha/reinicia e devolve proposta validada.',r'''
[Decisão] Aprovar protocolo e isolamento :: Resolver ADR-009; selecionar pipes/sockets/API local e escopo por tarefa. | docs/adr/009-workers.md | Processo auxiliar não compartilha implicitamente todos os direitos do Core.
[Contrato] Definir envelope de job :: Incluir IDs, revisão, input_refs, capacidade, orçamento, deadline e cancelamento. | schemas/job-v0.json | Job de revisão antiga é distinguível; payload inválido não inicia worker.
[Implementação] Implementar fila e estados :: Planejar pendente, em execução, retry, cancelado, bloqueado e concluído. | crates/jobs/src/state.rs | Reinício não duplica efeitos nem perde captura associada ao job.
[Implementação] Criar transporte local :: Fazer handshake, versão, limites de mensagem e identificação do processo. | crates/workers/src/transport.rs | Worker incompatível é recusado antes de receber dados.
[Implementação] Provisionar entradas restritas :: Entregar handles/cópias snapshot e diretório temporário exclusivo. | crates/workers/src/inputs.rs | Worker não ganha o path do Vault inteiro como atalho.
[Implementação] Validar resultados :: Checar esquema, hashes, revisão e destino antes de aceitar proposta. | crates/workers/src/results.rs | Resultado atrasado/malformado não publica conteúdo sobre revisão nova.
[Implementação] Implementar cancelamento :: Encerrar tarefa, limpar temporários e controlar descendentes. | crates/workers/src/cancel.rs | Cancelamento não apaga originais nem deixa processo órfão conhecido.
[Implementação] Implementar quotas :: Aplicar limites de CPU/memória/bytes/tempo por perfil e registrar consumo. | crates/workers/src/budget.rs | Worker que excede perfil falha de modo recuperável com causa identificada.
[Implementação] Criar backoff/idempotência :: Repetir apenas erros recuperáveis, com limite e chave de tarefa. | crates/jobs/src/retry.rs | Retry não duplica documentos, cobrança autorizada ou eventos autorais.
[Teste] Simular worker hostil :: Tentar path extra, resposta excessiva, processo filho e resposta fora de ordem. | tests/workers/protocol/ | Harness demonstra rejeição e nenhuma mutação canônica não autorizada.
''')

add(24,'Implementar perfis de sandbox Linux','SandboxManager / Linux','Próprio',['R01','R06','R08','R09'],['S23'],
'Conter parsing, inferência e browser por capacidades reais do kernel.','Perfis Linux e matriz de disponibilidade de segurança.','Worker sem rede não lê sentinela fora do escopo nem cria conexão externa.',r'''
[Implementação] Detectar recursos do kernel :: Verificar Landlock/ABI, seccomp e primitives necessárias antes de lançar job. | crates/sandbox/src/linux/probe.rs | Ausência de recurso obrigatório bloqueia a tarefa com explicação.
[Implementação] Definir perfis mínimos :: Separar parser, inferência CPU/GPU e aquisição web. | sandbox/linux/profiles/ | Um perfil com rede não é reaproveitado implicitamente para leitura de documentos privados.
[Implementação] Aplicar restrição filesystem :: Conceder leitura de entradas/modelos/libs e escrita apenas temporária. | crates/sandbox/src/linux/fs.rs | Diretório de outro workspace permanece inacessível mesmo com path conhecido.
[Implementação] Aplicar política de syscalls :: Reduzir superfície conforme perfil sem tratar Landlock como substituto de seccomp. | crates/sandbox/src/linux/syscalls.rs | Testes negativos de execução/processos e positivos das funções homologadas passam.
[Implementação] Conter rede :: Bloquear saída onde exigido e definir fronteira de broker para casos autorizados. | crates/sandbox/src/linux/network.rs | Worker local não pode contornar EgressPolicy usando sua própria biblioteca HTTP.
[Implementação] Controlar árvore de processos :: Registrar descendentes, limites e encerramento em falha. | crates/sandbox/src/linux/process_tree.rs | Matar tarefa também encerra filhos autorizados e não concede criação ilimitada.
[Implementação] Tratar GPU explicitamente :: Documentar devices/driver necessários e negar acessos não necessários. | sandbox/linux/gpu-capabilities.yaml | Perfil CPU funciona sem GPU; perfil GPU não recebe filesystem geral por conveniência.
[Teste] Exercitar escapes de filesystem :: Cobrir links, mounts disponíveis, descriptors herdados e temporários compartilhados. | tests/sandbox/linux/fs/ | Sentinelas externas não são lidas/escritas; limitações são declaradas e bloqueadoras quando aplicável.
[Teste] Exercitar rede/filhos :: Usar serviços de teste locais e processo descendente tentando sair do perfil. | tests/sandbox/linux/process-network/ | Proibições também valem para descendentes e não só para o PID inicial.
[Documentação] Publicar matriz Linux :: Registrar kernel/arquitetura, capacidades e comportamentos desabilitados. | reports/sandbox/linux-matrix.md | Não se alega proteção idêntica em kernels não testados.
''')

add(25,'Implementar perfis de sandbox macOS','SandboxManager / macOS','Próprio',['R01','R06','R08','R09','R30'],['S23'],
'Validar isolamento macOS sem depender de sandbox-exec como promessa futura.','Helper/perfis macOS e evidências nativas Intel/Apple Silicon conforme disponibilidade.','Processo auxiliar autorizado lê a entrada e falha ao acessar outro workspace ou rede proibida.',r'''
[Spike] Escolher mecanismo suportável :: Avaliar App Sandbox/helpers/XPC, assinatura e distribuição; registrar limites. | docs/security/macos-sandbox-design.md | Decisão não apresenta sandbox-exec deprecated como solução garantida de longo prazo.
[Implementação] Criar helper restrito :: Separar runtime confiável da comunicação com processos de parsing/inferência. | platform/macos/helper/ | Helper não herda toda a API Tauri nem credenciais do processo principal.
[Implementação] Definir entitlements :: Conceder somente arquivos, comunicação e recursos necessários por perfil. | platform/macos/entitlements/ | Diferenças parser/inferência/web são visíveis e revisadas.
[Implementação] Transferir entradas por capability :: Resolver documentos selecionados e fontes aprovadas por broker. | crates/sandbox/src/macos/inputs.rs | Escopo do workspace não vira acesso geral ao diretório pessoal.
[Implementação] Restringir saídas temporárias :: Criar diretórios por tarefa e validar resultados no Core. | crates/sandbox/src/macos/output.rs | Worker não publica diretamente nos arquivos Markdown canônicos.
[Implementação] Controlar rede/IPC :: Separar conexão necessária ao runtime local de saída externa. | platform/macos/policies/ | Modelo local não possui capacidade de egress externo não autorizada.
[Implementação] Integrar ciclo de vida :: Cancelar helpers e backends com cleanup e accounting. | crates/sandbox/src/macos/lifecycle.rs | Encerramento da tarefa não deixa runtime invisível consumindo recursos.
[Teste] Executar adversarial nativo :: Testar leitura/gravação externa, redes e propagação a filhos. | tests/sandbox/macos/ | Aprovação requer resultado em macOS real/ambiente válido, não apenas compilação cross-target.
[Teste] Validar empacotamento de helper :: Testar assinatura/permissões e máquina sem ferramentas de desenvolvimento. | reports/sandbox/macos-package.md | Pacote inicia com privilégios documentados e não exige desativar proteção do sistema silenciosamente.
[Decisão] Registrar suporte e bloqueios :: Distinguir capacidades homologadas por arquitetura e perfil. | reports/sandbox/macos-matrix.md | Função não confinada permanece desabilitada até solução ou revisão explícita de requisito.
''')

add(26,'Implementar perfis de sandbox Windows','SandboxManager / Windows','Próprio',['R01','R06','R08','R09','R30'],['S23'],
'Combinar contenção efetiva com controle de recursos e descendentes.','Perfis Windows com AppContainer/capabilities ou alternativa auditada.','Worker controlado não acessa paths/rede fora do permitido e toda sua árvore encerra.',r'''
[Spike] Definir fronteira Windows :: Avaliar AppContainer, tokens, ACLs e broker para motores selecionados. | docs/security/windows-sandbox-design.md | Job Objects não são apresentados como única barreira de arquivos/rede.
[Implementação] Criar identidade restrita :: Configurar contexto de segurança por worker sem herdar permissões gerais do usuário. | crates/sandbox/src/windows/identity.rs | Remover admin não é considerado suficiente; acesso efetivo é testado.
[Implementação] Conceder entradas explicitamente :: Aplicar ACL/capability/handles necessários aos snapshots e modelos. | crates/sandbox/src/windows/inputs.rs | Outro workspace e segredos do app continuam negados.
[Implementação] Conter filesystem :: Tratar reparse points, paths especiais e temporários dentro do perfil. | crates/sandbox/src/windows/fs.rs | Reparse point preparado não redireciona escrita para fora do destino autorizado.
[Implementação] Conter rede :: Definir proibições/capacidades efetivas, inclusive comunicação local necessária. | crates/sandbox/src/windows/network.rs | Worker offline não acessa serviço externo; não há dependência oculta de privilégio admin para cada job.
[Implementação] Usar Job Objects para lifecycle :: Limitar recursos e agrupar descendentes sem impedir indiscriminadamente browser/backends legítimos. | crates/sandbox/src/windows/jobs.rs | Perfil browser admite somente árvore esperada; cancelamento encerra todos os processos do job.
[Implementação] Controlar DLLs e executáveis :: Resolver binários em diretório gerenciado e verificar origem. | crates/sandbox/src/windows/loader.rs | PATH e diretório de trabalho hostis não substituem biblioteca/backend autorizado.
[Teste] Executar corpus adversarial :: Cobrir rede, privilégios, arquivos, junctions e criação de processos. | tests/sandbox/windows/ | Evidência demonstra negação real em Windows, não apenas retorno esperado de mock.
[Teste] Testar instalação sem admin :: Instalar perfil suportado, executar job e desinstalar sem resíduos privilegiados. | reports/sandbox/windows-install.md | Requisitos de privilégio são explícitos e funções inseguras falham fechadas.
[Decisão] Consolidar matriz multiplataforma :: Comparar garantias Linux/macOS/Windows e registrar capacidades desabilitadas. | reports/sandbox/capability-matrix.json | Jobs futuros só são habilitados onde o perfil necessário foi aprovado.
''')

add(27,'Desconstruir Docling e integrar extração textual','Docling / ExtractProvider','Desconstrução',['R14','R15','R30','R33','R34'],['S23','S24','S25','S26'],
'Reutilizar parsing sem tornar o worker responsável pelo formato ou armazenamento Sandland.','Worker Docling mínimo para documentos textuais.','Importar PDF/DOCX de fixture e obter blocos com referências ao original.',r'''
[Upstream] Fixar revisão Docling :: Registrar versão, deps, modelos necessários e licenças de cada artefato. | upstream/docling/manifest.json | Nenhum peso é baixado automaticamente sem passar pelo catálogo aprovado.
[Upstream] Reproduzir baseline de parsing :: Converter fixtures textuais upstream antes de criar o adapter. | reports/docling/baseline.md | Resultado original, versões e limitações estão registrados para comparação diferencial.
[Upstream] Recortar pipelines :: Selecionar parsing textual/tabelas; deixar OCR/VLM e servidor opcional fora do caminho leve. | workers/docling/pipeline-profile.yaml | Importação simples não carrega todo o catálogo de modelos ou serviços.
[Contrato] Definir NormalizedDocument :: Mapear blocos, hierarquia, páginas, tabelas, assets e diagnósticos. | schemas/normalized-document-v0.json | Contrato representa extração parcial e localizadores sem fingir cobertura completa.
[Implementação] Criar adapter Docling :: Converter estruturas da revisão fixada ao contrato, sem imports internos espalhados. | workers/docling/adapter.py | Mudança de versão upstream é confinada ao adapter e detectada por testes.
[Implementação] Preservar fonte e localizadores :: Relacionar conteúdo extraído ao asset/hash/página/bloco. | workers/docling/provenance.py | Trecho exibido pode ser rastreado ao documento original da revisão correta.
[Implementação] Integrar worker ao scheduler :: Receber entrada autorizada e devolver resultados por IPC. | workers/docling/entrypoint.py | Worker não escreve no Vault nem cria sua própria base canônica.
[Implementação] Tratar erros documentais :: Classificar formato não suportado, criptografado, corrompido e extração incompleta. | workers/docling/errors.py | Captura original permanece preservada e pode ser reprocessada.
[Teste] Comparar baseline e adapter :: Verificar perdas de texto/estrutura e schema em corpus de avaliação. | tests/extract/docling-text/ | Diferenças aceitas são documentadas; não há regressão silenciosa de tabelas/fontes.
[Teste] Medir processo leve :: Medir startup, pico, cancelamento e retorno ao estado sem worker. | reports/docling/text-resources.json | Orçamento do perfil é observado ou a integração é bloqueada/replanejada explicitamente.
''')

add(28,'Adicionar OCR e robustez da extração','Docling / OCR','Adaptação',['R09','R14','R15','R30'],['S27'],
'Expandir o processamento sem prometer que todo OCR cabe no perfil leve.','Pipeline OCR homologado e processamento recuperável por documento.','Importar PDF escaneado, cancelar, retomar e citar texto com localização.',r'''
[Decisão] Selecionar perfil OCR :: Comparar alternativas disponíveis na revisão fixada, idiomas, memória e licenças. | docs/models/ocr-profile.md | Perfil aprovado explicita hardware mínimo e não aumenta teto de RAM silenciosamente.
[Implementação] Provisionar artefatos OCR :: Instalar por manifest/hash e permitir uso com rede bloqueada depois do provisionamento. | workers/docling/ocr-manifest.json | Artefato faltante gera tarefa de instalação consentida, não download oculto durante parsing.
[Implementação] Detectar necessidade de OCR :: Distinguir camada textual suficiente, páginas imagem e extração suspeita. | workers/docling/ocr-routing.py | Documento textual comum evita custo OCR; classificação incorreta pode ser corrigida pelo usuário.
[Implementação] Processar com limites :: Definir unidade/paginação e concorrência permitida pelo motor sem alterar semântica. | workers/docling/ocr-jobs.py | Documento grande não cresce sem limite até derrubar o Core.
[Implementação] Preservar coordenadas :: Associar OCR e tabelas à página/asset original com status de precisão. | workers/docling/ocr-locators.py | Texto extraído não inventa coordenada ou evidência inexistente quando o backend não a fornece.
[Implementação] Registrar checkpoints :: Retomar unidades concluídas quando a API/pipeline escolhido permitir. | crates/ingest/src/extraction_checkpoint.rs | Retry não duplica blocos e fallback de reprocessamento completo é explícito quando necessário.
[Implementação] Expor qualidade/limitações :: Marcar baixa legibilidade, leitura parcial e páginas não processadas. | apps/desktop/src/ingest/extraction-quality.ts | Resumo futuro consegue distinguir documento completo de conteúdo parcialmente extraído.
[Teste] Testar corpus visual difícil :: Cobrir colunas, rotação, tabelas, imagens e diferentes idiomas. | tests/extract/ocr/ | Erros conhecidos são mensurados e não mascarados como compreensão visual perfeita.
[Teste] Testar limites e cancelamento :: Simular timeout/OOM e interrupção após algumas páginas. | reports/docling/ocr-failure.json | App preserva original, encerra worker e fornece reprocessamento controlado.
[Documentação] Publicar capacidades por perfil :: Relacionar formatos, idiomas, limites e recursos necessários. | docs/user/document-extraction.md | Usuário sabe quando OCR está indisponível ou requer pacote adicional.
''')

add(29,'Integrar Readability/Turndown para clipping seguro','Readability / Turndown','Desconstrução',['R08','R14','R17','R30'],['S23','S24','S25','S26'],
'Normalizar HTML sem renderizar páginas hostis na janela privilegiada.','Normalizador HTML→documento Sandland e captura manual de HTML.','Processar página de fixture com scripts/trackers sem executar scripts ou acessar rede.',r'''
[Upstream] Fixar bibliotecas e baseline :: Registrar versões/licenças e normalizar corpus de artigos antes de customizar. | upstream/web-normalizer/manifest.json | Saída upstream e erros conhecidos ficam disponíveis para comparação.
[Decisão] Escolher ambiente DOM isolado :: Definir renderer/helper sem IPC privilegiado e sem recursos remotos automáticos. | docs/architecture/html-normalizer.md | Plano não presume DOM completo em Web Worker comum nem usa janela principal como parser hostil.
[Implementação] Construir entrada inerte :: Aplicar limites de tamanho, encoding e tratamento de URL base. | workers/html-normalizer/input.ts | HTML malformado não escapa para navegação, scripts ou resolução arbitrária de arquivos.
[Implementação] Integrar Readability :: Extrair conteúdo principal e metadata mantendo diagnóstico de falha. | workers/html-normalizer/readability.ts | Página não reconhecida não é substituída por texto vazio como se captura tivesse sucesso.
[Implementação] Integrar Turndown :: Aplicar regras aprovadas para títulos, listas, tabelas, código e links. | workers/html-normalizer/turndown.ts | Conversão preserva conteúdo do perfil e usa Markdown válido em fixtures.
[Implementação] Sanitizar resultado :: Bloquear scripts/handlers/schemes perigosos antes da apresentação. | workers/html-normalizer/sanitize.ts | Readability não é tratado como sanitizador e payload XSS não executa.
[Implementação] Tratar assets externos :: Devolver lista de recursos candidatos para aquisição separada/autorizada. | workers/html-normalizer/assets.ts | Parser não baixa imagens/trackers por conta própria.
[Implementação] Normalizar fonte :: Registrar URL de origem, URL final informada, data e hash do HTML recebido. | workers/html-normalizer/provenance.ts | Snapshot é distinguível de página live e tem cadeia de origem verificável.
[Teste] Executar corpus hostil :: Cobrir iframes, meta refresh, links locais, CSS remoto e conteúdo oculto. | tests/extract/html-security/ | Zero chamadas de rede inesperadas e nenhum acesso à bridge Tauri.
[Teste] Comparar fidelidade e lifecycle :: Avaliar artigos, páginas não-artigo, idiomas e ciclos de criação/destruição. | reports/clipping/normalizer.md | Perdas e fallback são explícitos; renderer auxiliar não permanece órfão.
''')

add(30,'Entregar Ingest sem fricção e leitura do acervo','IngestCoordinator / UI','Integração',['R02','R14','R15','R17','R31'],['S27','S28','S29'],
'Unificar captura, extração e estados visíveis sem exigir organização antecipada.','v0.5.0: Ingest de arquivos e HTML local com leitura e promoção à Mesa.','Soltar documento, continuar trabalhando e promover um recorte após extração.',r'''
[Implementação] Criar entrada drag/drop e seleção :: Aceitar arquivos e texto sem formulário taxonômico obrigatório. | apps/desktop/src/ingest/capture.ts | Item recebe ID/original persistido antes de qualquer modelo ser chamado.
[Implementação] Implementar deduplicação :: Usar hash/fonte com política clara para nova captura versus referência existente. | crates/ingest/src/deduplicate.rs | Capturas legítimas em datas diferentes não são fundidas sem preservar sua origem.
[Implementação] Orquestrar roteamento :: Escolher parser por tipo verificado, não só por extensão do nome. | crates/ingest/src/router.rs | Tipo enganoso ou não suportado é recusado/registrado sem execução arbitrária.
[Implementação] Criar UI da fila :: Mostrar captura salva, extração pendente, parcial, falhou e reprocessar. | apps/desktop/src/ingest/jobs.ts | Falha de OCR não faz documento desaparecer do acervo.
[Implementação] Publicar extração pelo Core :: Validar resultado e persistir versão de conteúdo/mapa de fontes. | crates/ingest/src/commit_extraction.rs | Worker atrasado não sobrescreve correção manual nem resultado de pipeline mais recente.
[Implementação] Criar leitor de material :: Exibir texto, origem, diagnóstico e navegação ao asset por broker. | apps/desktop/src/ingest/reader.ts | Conteúdo ativo é neutralizado e origem permanece disponível offline.
[Implementação] Criar recorte para célula :: Promover seleção com fonte/revisão preservadas. | crates/ingest/src/promote.rs | Recorte novo não modifica fonte imutável e conecta-se à proveniência da Peça.
[Implementação] Gerenciar reprocessamento :: Separar nova extração, enriquecimento e alterações autorais aceitas. | crates/ingest/src/reprocess.rs | Reprocessar não apaga anotação/metadata editada pelo usuário.
[Teste] Executar ingestão em lote :: Misturar formatos, duplicatas, arquivos inválidos e cancelamentos. | reports/v0.5/batch-ingest.json | Fila progride com falhas isoladas, sem deadlock nem corrupção global.
[Release] Publicar alpha de acervo :: Incluir pacote de capacidades por plataforma e instruções sem IA. | release/v0.5.0/ | Usuário captura, lê e organiza materiais sem chave de API.
''')

add(31,'Recortar e gerenciar LocalAI','LocalAI / RuntimeProvider','Desconstrução',['R03','R07','R08','R09','R27','R30'],['S23','S24','S25','S26','S30'],
'Adotar infraestrutura de inferência sem trazer outro sistema de agentes/memória.','Runtime LocalAI gerenciado e pacote mínimo por alvo de desenvolvimento.','Iniciar runtime autorizado, chamar backend de teste e encerrar sua árvore inteira.',r'''
[Upstream] Fixar LocalAI e backends candidatos :: Registrar código Go, imagens/binários, deps e interfaces usadas. | upstream/localai/manifest.json | Todos os artefatos têm origem e versão; API compatível não substitui inventário.
[Upstream] Reproduzir inferência mínima :: Executar modelo de teste aprovado fora do app e registrar resultados. | reports/localai/baseline.md | Configuração upstream verificável existe antes de customizações.
[Upstream] Delimitar superfície :: Excluir do produto agentes, shell, memória/RAG e painel administrativo não necessários. | docs/upstream/localai-boundaries.md | Nenhuma ferramenta do modelo alcança API administrativa ou execução arbitrária.
[Decisão] Ratificar runtime :: Resolver ADR-010 incluindo distribuição, atualização e supervisão. | docs/adr/010-runtime.md | Instalação não exige Docker/Python/Go globais de forma implícita.
[Implementação] Criar RuntimeProvider :: Iniciar, verificar saúde, obter capacidades e encerrar runtime por protocolo. | adapters/localai/ | Core não depende de detalhes privados do LocalAI fora do adapter.
[Implementação] Proteger API local :: Bind restrito, autenticação de sessão e acesso apenas via Core. | adapters/localai/security.rs | Página adquirida ou processo sem token não pode usar a API como ponte de execução.
[Implementação] Aplicar perfis :: Gerar configuração somente para backends aprovados e controlar downloads. | adapters/localai/profiles.rs | Modelo solicitado pelo agente não instala backend desconhecido.
[Implementação] Integrar lifecycle :: Controlar descendentes, stdout/erros saneados e falha de startup. | crates/workers/src/localai_host.rs | Falha de modelo não derruba a UI nem deixa processo invisível.
[Teste] Testar runtime offline :: Provisionar artefatos e bloquear rede durante execução. | tests/runtime/localai-offline/ | Geração local não depende de catálogo remoto, telemetria ou download tardio.
[Teste] Medir footprint sem/com modelo :: Contabilizar runtime e backend filhos separadamente. | reports/localai/resources.json | Relatório distingue overhead do servidor, pesos e contexto; sem alegação de teto não medido.
''')

add(32,'Homologar modelos e construir ModelGateway','ModelGateway / catálogo','Próprio',['R07','R08','R09','R20','R27','R30'],['S31'],
'Escolher modelos por função, idioma e hardware em vez de por nome fixado sem prova.','Catálogo homologado, instalação verificada e gateway de geração/embeddings.','Instalar pacote aprovado, detectar corrupção e obter resposta local cancelável.',r'''
[Decisão] Selecionar candidatos por papel :: Avaliar resumo, embeddings, copiloto e transcrição em português e hardware-alvo. | docs/adr/011-models.md | Nome do modelo e quantização têm evidência; equivalente local a Jev não é presumido.
[Implementação] Criar manifests de modelos :: Registrar hash completo, licença, origem, tokenizer, dimensão e capacidades. | models/manifests/ | Placeholders/truncamentos são rejeitados pelo validador.
[Implementação] Implementar instalação consentida :: Download retomável, verificação e publicação atômica de artefatos. | crates/models/src/install.rs | Arquivo parcial/corrompido não é carregado; modo offline aceita pacote pré-provisionado.
[Implementação] Resolver diretórios por SO :: Usar pastas apropriadas e cache compartilhado fora de cada Vault. | crates/models/src/paths.rs | Não há dependência de home/path hardcoded nem duplicação automática por workspace.
[Contrato] Definir capabilities de modelo :: Distinguir generate, embed, tools, schema e transcribe sem assumir paridade. | schemas/model-capabilities.json | Chamada incompatível falha antes de enviar conteúdo ou instalar recursos.
[Implementação] Criar GenerationProvider :: Normalizar streaming, erros, cancelamento e métricas sem apagar identidade do provedor. | crates/model-gateway/src/generate.rs | Cancelamento propaga e conteúdo parcial é distinguido de resposta concluída.
[Implementação] Criar EmbeddingProvider :: Fixar preprocessing/pooling/normalização e retornar manifesto junto ao vetor. | crates/model-gateway/src/embed.rs | Vetor de mesmo tamanho mas outro espaço não é aceito como compatível.
[Implementação] Integrar descarregamento :: Usar limites/watchdog do runtime e verificar tarefas ainda ativas. | crates/models/src/lifecycle.rs | Ociosidade de cinco minutos libera recursos conforme perfil, sem cortar inferência em andamento.
[Teste] Testar quotas e starvation :: Concorrer embeddings/resumo com baixo orçamento e prioridades de UI. | tests/runtime/scheduling/ | Fila limita modelos residentes e não provoca alternância patológica sem diagnóstico.
[Teste] Homologar catálogo inicial :: Rodar qualidade, memória e falhas de cada perfil selecionado. | reports/models/qualification.json | Modelo não aprovado fica indisponível/experimental, sem ser anunciado como padrão homologado.
''')

add(33,'Implementar resumo executivo e enriquecimento gerativo','LocalAI / sumarização','Adaptação',['R14','R19','R23','R27'],['S27','S28','S30','S32'],
'Produzir o resumo de 2–5 frases sem confundir síntese com evidência original.','SummaryService com pipeline longo, proveniência e revisão humana.','Resumir documento, corrigir o texto e reprocessar sem perder a correção.',r'''
[Contrato] Definir resultado de resumo :: Registrar texto, revisão, modelo, prompt, cobertura e fontes relevantes. | schemas/summary-result.json | Ausência de cobertura completa é representável e não mascarada.
[Implementação] Criar prompt/validador curto :: Pedir síntese densa e controlar extensão por idioma sem exigir slogans. | crates/enrichment/src/executive_summary.rs | Resultado de 2–5 frases é verificável; excesso/insuficiência aciona tratamento explícito.
[Implementação] Implementar rota de documento curto :: Ler conteúdo normalizado autorizado e chamar modelo leve. | crates/enrichment/src/summary_short.rs | Não usa apenas título ou snippet como se tivesse lido o corpo.
[Implementação] Implementar rota hierárquica :: Resumir seções sob budget e consolidar sem duplicar chamadas desnecessárias. | crates/enrichment/src/summary_long.rs | Documento além do contexto não é truncado silenciosamente aos primeiros tokens.
[Implementação] Preservar evidências/limitações :: Associar afirmações importantes ou referências disponíveis ao material de entrada. | crates/enrichment/src/summary_provenance.rs | Resumo não recebe status de fonte primária e não inventa localização ausente.
[Implementação] Persistir gerado versus editado :: Separar proposta automática e versão confirmada pelo usuário. | crates/enrichment/src/summary_state.rs | Reindexação/reexecução não substitui resumo editado.
[Implementação] Criar UI de revisão :: Exibir origem automática, regenerar, editar, aceitar e restaurar. | apps/desktop/src/ingest/summary.ts | Ação é reversível e não depende de alterar o original capturado.
[Implementação] Controlar cache por revisão :: Reutilizar somente quando conteúdo/modelo/prompt compatíveis. | crates/enrichment/src/summary_cache.rs | Alterar o texto invalida resumo; mover cartão não o invalida.
[Teste] Avaliar factualidade no corpus :: Medir omissões, números errados, idioma e abstenção em extração parcial. | tests/eval/summaries/ | Falhas são registradas contra conjunto reservado, não resolvidas apenas mudando o exemplo de demonstração.
[Teste] Testar ausência de modelo/OOM :: Manter captura e permitir revisão futura ou escrita manual do resumo. | reports/summary/failure.md | Enriquecimento opcional não torna o acervo indisponível.
''')

add(34,'Integrar Jev como motor de decisões','Jev / DecisionProvider','Integração',['R07','R08','R18','R33','R34'],['S23','S30','S32'],
'Implementar decisões tipadas e egress explícito, sem tentar executar Jev no LocalAI.','Adapter Jev testável offline e chamadas reais estritamente opt-in.','Classificar fixture autorizada e negar a mesma chamada quando a política de rede proíbe.',r'''
[Upstream] Fixar contrato da API :: Registrar SDK/endpoint/modelo, termos, limites e formato de respostas observados. | upstream/jev/contract.json | Integração identifica oferta hospedada; código aberto do cliente não é confundido com pesos disponíveis.
[Contrato] Definir DecisionRequest/Result :: Modelar Choice, Score e Noul com campos distintos e proveniência. | schemas/decision-v0.json | Noul não ganha confidence inventada e score local não é rotulado como Jev.
[Implementação] Implementar armazenamento de chave :: Usar cofre de credenciais do SO e não expor segredo à UI/logs. | adapters/jev/credentials.rs | Export do Vault e Git não contêm chave nem headers sensíveis.
[Implementação] Aplicar EgressPolicy antes da chamada :: Verificar consentimento, escopo e conteúdo autorizado por tarefa. | adapters/jev/egress.rs | Classificador não recebe dados para decidir se o próprio envio é permitido.
[Implementação] Criar cliente com limites :: Tratar timeout, rate limit, retries controlados, custo/uso disponível e cancelamento. | adapters/jev/client.rs | Retry não ocorre ilimitadamente nem ignora revogação de consentimento.
[Implementação] Mapear categoria Choice :: Incluir candidatos e opção nenhuma/insuficiente quando pertinente. | adapters/jev/category.rs | Categoria fora dos candidatos não é inventada por parsing de texto livre.
[Implementação] Mapear tags Noul :: Avaliar pertencimento independente por candidato em lote quando suportado. | adapters/jev/tags.rs | Tags não se tornam mutuamente exclusivas e probability não é tratada como verdade garantida.
[Implementação] Mapear rubricas Score :: Versionar níveis e preservar distribuições/identidade do critério. | adapters/jev/scores.rs | Mudar rubrica invalida comparação indevida com scores antigos.
[Teste] Criar transporte fake e testes de falha :: Simular respostas válidas, recusa, timeout, segredo ausente e payload inválido. | tests/decision/jev-contract/ | CI passa sem chave e sem chamada faturável; falhas não aplicam decisões parciais indevidas.
[Teste] Executar smoke real consentido :: Usar corpus não sensível e orçamento aprovado; registrar versão e métricas. | reports/jev/live-smoke.md | Teste não executado permanece pendente; resultado real não é fabricado a partir do mock.
''')

add(35,'Implementar taxonomia e fallback de classificação','TaxonomyService / Jev','Próprio',['R02','R07','R18','R20','R28'],['S32','S34'],
'Colocar políticas de classificação no produto, e não dentro de prompts opacos.','Taxonomia canônica, revisão/aliases e fallback offline.','Aplicar sugestão, desfazer e manter classificação utilizável sem a API.',r'''
[Implementação] Persistir taxonomia por IDs :: Guardar termos, idioma, descrições, aliases e origem das decisões. | crates/taxonomy/src/store.rs | Apagar DB não perde termos/aliases aprovados.
[Implementação] Recuperar candidatos :: Combinar léxico e embeddings aprovados sem misturar espaços incompatíveis. | crates/taxonomy/src/candidates.rs | Categoria correta fora do top-k pode levar a expansão/abstenção, não escolha forçada silenciosa.
[Decisão] Calibrar política de aceitação :: Resolver ADR-012 com conjunto separado de avaliação e tolerância a erro. | docs/adr/012-taxonomy-policy.md | Thresholds não são copiados cegamente do RFC nem tratados como universais.
[Implementação] Aplicar decisões reversíveis :: Diferenciar sugerido, aceito automaticamente, confirmado e rejeitado. | crates/taxonomy/src/decisions.rs | Cada mutação tem origem e pode ser revertida sem apagar decisão humana posterior.
[Implementação] Propor novos termos :: Usar extração/modelo generativo com validação; Jev avalia candidatos quando autorizado. | crates/taxonomy/src/new_terms.rs | Jev não é chamado para gerar string livre inexistente em suas primitivas.
[Implementação] Proteger merges/aliases :: Transformar distância/similaridade em sugestão revisável, não fusão silenciosa. | crates/taxonomy/src/aliases.rs | Termos parecidos mas diferentes permanecem distintos sem aprovação.
[Implementação] Implementar fallback local :: Usar modelo homologado com schema e registrar ausência de calibração equivalente. | crates/taxonomy/src/local_fallback.rs | Probabilidades/confidence não são inventadas para imitar resultado Jev.
[Implementação] Implementar fallback sem modelo :: Regras léxicas ou pendência/revisão manual sem precisar carregar embeddings por OOM. | crates/taxonomy/src/heuristic_fallback.rs | Máquina sem RAM/rede continua capturando material e mostra estado correto.
[Implementação] Criar UI de revisão taxonômica :: Filtrar sugestões, editar, confirmar e desfazer em lote com preview. | apps/desktop/src/taxonomy/ | Bulk action não altera itens fora do conjunto aprovado nem esconde baixa confiança.
[Teste] Avaliar português e casos ambíguos :: Medir categoria, multilabel, abstention e merges indevidos. | reports/taxonomy/evaluation.json | Regressões em corpus reservado impedem habilitar automação mais agressiva.
''')

add(36,'Transcrever mídia e concluir enriquecimento do Ingest','Transcrição / Ingest integrado','Integração',['R09','R14','R16','R18','R19','R27'],['S30','S32','S33','S35'],
'Fechar o conjunto de enriquecimentos com áudio/vídeo e estados independentes.','v0.6.0 com resumo, classificação e transcrição local homologada.','Importar áudio e documento, continuar editando e acompanhar cada resultado separadamente.',r'''
[Upstream] Fixar backend de transcrição :: Selecionar rota LocalAI/Whisper homologada e dependências de decodificação. | upstream/transcription/manifest.json | Licenças e binários auxiliares estão inventariados; suporte não é presumido por extensão.
[Contrato] Definir transcript e timestamps :: Modelar segmentos, idioma, revisão da fonte e cobertura parcial. | schemas/transcript-v0.json | Segmento cita tempo e arquivo/hash corretos; diarização não é prometida sem implementação.
[Implementação] Preparar áudio com limites :: Decodificar/normalizar em worker autorizado e controlar arquivos temporários. | workers/transcription/input/ | Arquivo corrompido ou excessivo não trava Core nem executa comando montado com entrada não confiável.
[Implementação] Executar transcrição :: Integrar backend por jobs e orçamento, com cancelamento real. | workers/transcription/adapter/ | Tarefa interrompida libera recursos e conserva original/segmentos aproveitáveis conforme contrato.
[Implementação] Persistir texto corrigível :: Manter transcrição gerada e revisão humana separadas. | crates/ingest/src/transcripts.rs | Reprocessamento não apaga correções manuais nem perde timestamps originais.
[Implementação] Unificar estados de enriquecimento :: Acompanhar extração, resumo, classificação e embeddings independentemente. | crates/ingest/src/enrichment_state.rs | Falha em uma etapa não falsifica sucesso das demais ou bloqueia leitura do item.
[Implementação] Evitar competição de modelos :: Priorizar UI e reagendar OCR/STT/geração conforme perfil. | crates/jobs/src/model_scheduling.rs | Workloads simultâneos não ultrapassam política aprovada por serem jobs independentes.
[Teste] Avaliar transcrição portuguesa :: Medir erros e alinhamento temporal no corpus reservado. | reports/transcription/qualification.json | Limitações de idioma/ruído são expostas; transcrição não é tratada como citação perfeita sem revisão.
[Teste] Executar jornada sem chave/rede :: Classificar por fallback, resumir localmente e transcrever com artefatos instalados. | reports/v0.6/offline-enrichment.md | Ausência Jev/BYOK não provoca requisição escondida ou perda de captura.
[Release] Publicar alpha assistida :: Empacotar perfis aprovados, diagnóstico de modelo e documentação de consentimento. | release/v0.6.0/ | Capacidade só é anunciada nos alvos/perfis em que foi testada.
''')
