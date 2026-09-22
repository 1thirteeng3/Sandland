from plan_base import add

add(12,'Extrair o recorte editorial do BlockSuite','BlockSuite / upstream','Desconstrução',['R03','R10','R30','R33','R34'],['S03','S04','S06','S11'],
'Reproduzir a edição mínima sem importar o produto AFFiNE.','Pacote editorial isolado e ADR do recorte.','Editor mínimo funciona fora do monorepo de origem e sua origem/licença é rastreável.',r'''
[Upstream] Revalidar origem e revisão :: Escolher release/commit e árvore standalone/AFFiNE após auditar o recorte. | upstream/blocksuite/manifest.json | Revisão imutável e licenças por arquivo registradas; nenhuma decisão depende de canary flutuante.
[Upstream] Reproduzir exemplo upstream :: Executar editor mínimo com fixtures antes de modificar código. | reports/blocksuite/upstream-baseline.md | Ambiente, comandos e comportamento original registrados; falha de origem não é atribuída ao adapter.
[Upstream] Mapear grafo mínimo :: Rastrear store, std, rich-text, modelos, blocos e dependências workspace. | upstream/blocksuite/dependency-graph.json | Todas as dependências de runtime têm origem; copiar três diretórios não é presumido suficiente.
[Upstream] Classificar componentes :: Separar edição, sync, gfx, copilot, cloud e componentes dependentes da aplicação. | docs/upstream/blocksuite-boundaries.md | Funcionalidades excluídas não reentram por import agregador não auditado.
[Implementação] Criar pacote facade :: Expor criação/destruição, snapshot, comandos e eventos necessários ao Sandland. | adapters/blocksuite/ | App não importa internals espalhados por toda a UI.
[Implementação] Fechar resolução de pacotes :: Resolver workspace deps, assets e registro de custom elements sem duplicar Yjs. | adapters/blocksuite/build/ | Build fora da árvore AFFiNE é reproduzível e não contém imports não resolvidos.
[Implementação] Definir patch set mínimo :: Isolar mudanças indispensáveis e manter comparação contra upstream. | upstream/blocksuite/patches/ | Cada patch tem motivação/teste; removê-lo revela apenas a diferença documentada.
[Teste] Testar ciclo mount/unmount :: Montar/desmontar editores repetidamente e verificar observers/listeners. | tests/editor/lifecycle/ | Editor destruído não recebe eventos nem mantém documentos órfãos conhecidos.
[Decisão] Aprovar ADR editorial :: Fixar versão, recorte, risco, estratégia de atualização e fronteira com PixiJS. | docs/adr/008-editor-engine.md | Autor ratifica recorte; segundo canvas completo não é incorporado.
[Teste] Medir pacote isolado :: Registrar bundle, memória e dependências carregadas sem app completo. | reports/blocksuite/isolation.json | Relatório permite atribuir overhead ao editor e não declara meta de produto já satisfeita.
''')

add(13,'Implementar ponte Markdown ↔ BlockSuite','EditorBridge / formato','Adaptação',['R02','R04','R10','R13','R32'],['S02','S12'],
'Conectar o modelo editorial ao formato autoral sem perda silenciosa.','Serializer/deserializer e fixtures de round-trip.','Abrir Markdown externo, editar no BlockSuite e preservar a semântica e metadados.',r'''
[Contrato] Mapear blocos suportados :: Associar cada tipo aprovado a modelos/views e representação Markdown. | adapters/blocksuite/block-map.json | Todo bloco habilitado possui entrada/saída; tipo experimental não aparece como persistível sem codec.
[Implementação] Importar texto estrutural :: Converter títulos, parágrafos, listas e blocos de código preservando conteúdo. | adapters/blocksuite/import/basic.ts | Fixtures com listas aninhadas, código e caracteres especiais mantêm significado.
[Implementação] Importar estruturas ricas :: Tratar tabelas, citações, links e mídias no perfil suportado. | adapters/blocksuite/import/rich.ts | Elementos não suportados são preservados/opacos ou recusados explicitamente.
[Implementação] Exportar Markdown :: Serializar a estrutura em ordem determinística sem depender da seleção visual. | adapters/blocksuite/export/markdown.ts | Exportação de documento completo não omite conteúdo fora da viewport.
[Implementação] Preservar frontmatter :: Manter campos conhecidos/desconhecidos separados do estado de edição. | adapters/blocksuite/frontmatter.ts | Abrir/salvar não remove propriedades de ferramentas externas nem muda tipos indevidamente.
[Implementação] Preservar identidade de blocos :: Associar IDs ao formato aprovado e reconciliar alterações externas. | adapters/blocksuite/identity.ts | Referência permanece estável em edição simples; mudança ambígua gera estado de reancoragem.
[Implementação] Resolver assets por referência :: Trocar paths absolutos por IDs/hash e broker de leitura. | adapters/blocksuite/assets.ts | Conteúdo colado não injeta leitura de arquivo fora do Vault.
[Implementação] Preservar conteúdo desconhecido :: Implementar nó opaco/estado read-only para extensão não compreendida. | adapters/blocksuite/unknown-block.ts | Documento novo aberto em versão antiga não perde extensão ao salvar conteúdo conhecido.
[Teste] Executar golden round-trip :: Comparar entrada, snapshot intermediário e saída com critérios sintáticos/semânticos definidos. | tests/editor/roundtrip/ | Formatação permitida pode normalizar, mas textos, IDs e referências não desaparecem.
[Teste] Fuzzar import/export :: Gerar Markdown/frontmatter truncados e combinações de blocos. | tests/editor/codec-fuzz/ | Erro é recuperável e não produz publicação parcial ou loop sem limite.
''')

add(14,'Integrar comandos, salvamento e undo editorial','BlockSuite / domínio','Integração',['R04','R05','R10','R31'],['S08','S10','S13'],
'Fazer o editor participar do protocolo de revisão do Sandland.','Edição de notas com salvamento confiável e undo de produto.','Editar, desfazer, conflitar com editor externo e recuperar sem cópia oculta no Yjs.',r'''
[Implementação] Adaptar eventos de edição :: Transformar mudanças do editor em operações com base_revision. | adapters/blocksuite/change-adapter.ts | Duas sessões desatualizadas não publicam sobre a mesma revisão sem conflito.
[Implementação] Implementar fila de autosave :: Agregar mudanças sem bloquear renderização e enviar ao escritor único. | apps/desktop/src/editor/save-queue.ts | Digitação permanece responsiva e erro de disco não é exibido como salvo.
[Implementação] Exibir estados de persistência :: Diferenciar editando, pendente, confirmado, conflito e falha. | apps/desktop/src/editor/save-status.ts | Fechar com dados não confirmados alerta e conserva material recuperável.
[Implementação] Integrar undo/redo :: Delimitar histórico de edição em sessão e eventos semânticos duráveis. | crates/editor-domain/src/history.rs | Undo não atravessa silenciosamente uma alteração externa ou apaga proveniência.
[Implementação] Integrar comandos e atalhos :: Mapear seleção, inserção, formatação e navegação ao shell. | apps/desktop/src/editor/commands.ts | Atalho de editor não dispara simultaneamente comando de canvas ou aplicação.
[Implementação] Controlar clipboard :: Importar texto/HTML/assets por pipeline seguro e apresentar resultado previsível. | apps/desktop/src/editor/clipboard.ts | Colagem de HTML não executa script e referências locais não autorizadas são negadas.
[Implementação] Retomar rascunho após falha :: Reconstruir documento a partir de revisão/checkpoint autorizado. | apps/desktop/src/editor/recovery.ts | Reabertura não depende de IndexedDB/Yjs como única fonte do conteúdo salvo.
[Implementação] Tratar documento read-only :: Exibir fontes imutáveis e sugerir criar nota/recorte para editar. | apps/desktop/src/editor/readonly.ts | Formatação/clipboard não conseguem modificar fonte marcada somente leitura pelo domínio.
[Teste] Exercitar IME e seleção :: Cobrir composição de acentos, teclados diferentes e seleção extensa. | tests/editor/input/ | Autosave e rerender não interrompem composição nem deslocam caret indevidamente.
[Teste] Validar conflitos ponta a ponta :: Editar o mesmo arquivo fora do app durante digitação. | reports/editor/concurrency.md | Usuário resolve o conflito e ambas as versões podem ser recuperadas.
''')

add(15,'Entregar notas locais e homologar editor','Editor / Tauri','Integração',['R07','R09','R10','R31','R34'],['S14'],
'Concluir um produto de autoria textual útil antes de adicionar a mesa.','v0.2.0 alpha de notas locais.','Instalação limpa permite criar, editar, pesquisar por título e reabrir notas sem rede.',r'''
[Implementação] Criar lista e navegação de notas :: Usar inventário local e identidade, sem indexação semântica obrigatória. | apps/desktop/src/notes/ | Renomeação preserva seleção e documento aberto.
[Implementação] Criar ações de documento :: Novo, duplicar, mover, lixeira e restaurar usando o Core. | crates/editor-domain/src/doc_actions.rs | Duplicata possui nova identidade e restauração não sobrepõe outro arquivo existente.
[Implementação] Implementar busca simples por título :: Consultar inventário sem antecipar o backend vetorial. | apps/desktop/src/notes/title-search.ts | Funciona com modelos ausentes e não envia texto a serviço externo.
[Implementação] Construir leitura segura :: Renderizar links/assets e bloquear conteúdo ativo não pertencente ao app. | apps/desktop/src/reader/ | Abrir nota hostil não acessa IPC privilegiado nem recursos remotos por padrão.
[Implementação] Polir foco e teclado :: Definir sequência entre lista, editor, toolbar e estados de erro. | apps/desktop/src/notes/focus.ts | Jornada criar→editar→salvar→reabrir pode ser feita com teclado.
[Teste] Comparar renderização entre WebViews :: Executar corpus visual e de input nos alvos disponíveis. | tests/editor/webview-matrix/ | Divergência tem issue e capacidade limitada; ausência de teste não é tratada como aprovação.
[Teste] Medir sessões repetidas :: Alternar documentos e destruir instâncias medindo memória residual. | reports/v0.2/editor-resources.json | Crescimento acumulativo fora do perfil aprovado bloqueia a release.
[Teste] Validar autoria offline :: Bloquear rede, reiniciar e repetir edição/importação de Markdown. | reports/v0.2/offline.json | Zero egress inesperado e nenhuma dependência de autenticação remota.
[Documentação] Publicar perfil suportado :: Listar blocos, atalhos, limites e comportamento de alterações externas. | docs/user/notes.md | Usuário sabe quando um bloco é opaco/read-only e como recuperar conteúdo.
[Release] Preparar alpha de autoria :: Anexar corpus, origem BlockSuite, avisos e demonstração de reconstrução. | release/v0.2.0/ | Caches do editor podem ser apagados sem perder notas confirmadas.
''')

add(16,'Construir cena PixiJS e interação básica','PixiJS / Whiteboard','Integração',['R11','R12','R31'],['S06','S07','S15'],
'Criar espaço de trabalho visual sem delegar a semântica do board a um motor externo.','Viewport com cartões, câmera, seleção e previews.','Criar e navegar por cartões mantendo coordenadas consistentes em diferentes zooms.',r'''
[Upstream] Fixar PixiJS e viewport :: Escolher versões e APIs mínimas; registrar origem e controles de teardown. | upstream/pixi/manifest.json | Cena mínima reproduzível sem importar outro produto de whiteboard.
[Implementação] Criar sistema de coordenadas :: Separar screen/world e documentar transformação, escala e precisão. | apps/desktop/src/board/coordinates.ts | Conversão ida/volta é testada em zooms extremos e escalas de tela.
[Implementação] Criar câmera :: Pan, zoom ancorado ao cursor, fit e restauração de viewport. | apps/desktop/src/board/camera.ts | Zoom não altera posições canônicas dos nós.
[Implementação] Renderizar cartões passivos :: Desenhar contorno, título, preview e placeholder de asset. | apps/desktop/src/board/card-renderer.ts | Cartão sem editor montado é legível e mantém identidade de domínio.
[Implementação] Implementar seleção :: Clique, multiseleção, retângulo e limpar seleção com foco previsível. | apps/desktop/src/board/selection.ts | Seleção visual não modifica conteúdo e não inclui nós ocultos indevidos.
[Implementação] Implementar drag/resize :: Aplicar transformações temporárias e confirmar operação semântica ao terminar. | apps/desktop/src/board/manipulation.ts | Cancelar gesto restaura posição; não há commit por pixel movido.
[Implementação] Implementar hit testing :: Separar alvos de cartão, conector, grupo e canvas. | apps/desktop/src/board/hit-test.ts | Zoom/escala não fazem clique em célula acionar widget vizinho.
[Implementação] Criar previews limitados :: Truncar apresentação sem truncar o conteúdo canônico. | apps/desktop/src/board/previews.ts | Texto oculto no preview continua presente e editável no documento.
[Teste] Testar cena determinística :: Reproduzir gestos e capturas em corpus de posições e tamanhos. | tests/board/scene-basic/ | Mesma sequência de comandos produz mesma topologia, sem depender de frame rate.
[Teste] Medir frame time inicial :: Registrar baseline com cenas sintéticas e identificar gargalos antes do LOD. | reports/board/scene-baseline.json | Resultado inclui hardware e payload; 60 FPS não é alegado por observação subjetiva.
''')

add(17,'Persistir topologia, grupos e relações','BoardService / topologia','Próprio',['R02','R05','R11','R22'],['S08','S10','S16'],
'Transformar a cena em conhecimento espacial persistente.','BoardService canônico e cache MPK validado por revisão.','Criar ciclo de relações, agrupar, fechar, apagar MPK e reconstruir a mesma mesa.',r'''
[Implementação] Criar comandos de board :: Adicionar/remover ocorrência e alterar posição/tamanho por revisão. | crates/board/src/commands.rs | Operação visual só fica confirmada após persistência no domínio.
[Implementação] Separar conteúdo/ocorrência :: Referenciar nota por ID e manter propriedades locais no cartão. | crates/board/src/occurrences.rs | Duas ocorrências editam o mesmo conteúdo, mas não compartilham coordenadas.
[Implementação] Implementar grupos :: Criar membership explícito, rótulo e limites geométricos. | crates/board/src/groups.rs | Proximidade não cria relação semântica por conta própria.
[Implementação] Implementar arestas :: Persistir direção, rótulo, origem manual/IA e endpoints válidos. | crates/board/src/edges.rs | Relações apoia/contradiz são distintas; loops/ciclos permitidos têm representação consistente.
[Implementação] Validar referência órfã :: Definir comportamento para nó removido, conteúdo ausente e fonte excluída. | crates/board/src/validation.rs | Estado inconsistente aparece para reparo, sem remoção silenciosa de autoria.
[Implementação] Serializar JSON estável :: Ordenar coleções por regra explícita e evitar ruído desnecessário de diff. | crates/board/src/codec_json.rs | Regravação sem mudança não produz diff semântico nem altera IDs.
[Implementação] Gerar MPK derivado :: Vincular cache ao hash/schema da topologia e reconstruir quando inválido. | crates/board/src/cache_mpk.rs | MPK mais recente por timestamp não vence JSON/journal autoritativo.
[Implementação] Criar undo de operações espaciais :: Agrupar drag/resize e manter alterações de relações reversíveis. | crates/board/src/undo.rs | Desfazer movimento não reverte indevidamente edição textual posterior.
[Teste] Testar topologias complexas :: Cobrir ciclos, referências cruzadas, grupos e múltiplas ocorrências. | tests/board/topology/ | Nenhuma transformação para árvore apaga arestas canônicas.
[Teste] Executar reconstrução do board :: Remover caches e reabrir após edições externas controladas. | reports/board/rebuild.json | Geometria, grupos e relações são equivalentes ao estado confirmado.
''')

add(18,'Integrar overlay BlockSuite à Mesa','PixiJS + BlockSuite','Adaptação',['R04','R10','R11','R12','R31'],['S14','S17'],
'Editar conteúdo no espaço visual sem instanciar um editor completo por nó.','Overlay DOM sincronizado à câmera e política de editores ativos.','Editar uma célula em diferentes zooms e voltar à cena sem perder caret ou conteúdo.',r'''
[Contrato] Definir política de edição ativa :: Fixar limite de editores, modo overlay/painel e interação com seleção de cartão. | docs/contracts/board-editor.md | Número de nós não implica o mesmo número de documentos DOM montados.
[Implementação] Posicionar overlay :: Traduzir coordenadas world→screen e aplicar clipping/dimensões apropriadas. | apps/desktop/src/board/editor-overlay.ts | Pan, zoom e resize de janela mantêm editor alinhado ao cartão selecionado.
[Implementação] Gerenciar foco :: Transferir foco entre canvas e editor sem conflitos de comandos. | apps/desktop/src/board/focus-controller.ts | Escape encerra edição conforme regra e não descarta alterações não salvas.
[Implementação] Integrar caret/seleção :: Manter seleção textual durante atualização de preview e mudanças de layout. | apps/desktop/src/board/text-selection.ts | Rerender do board não reposiciona caret a cada tecla.
[Implementação] Tratar IME no overlay :: Coordenar composição com autosave e término da edição. | apps/desktop/src/board/ime.ts | Acentos e composição não confirmada não são cortados por drag ou blur.
[Implementação] Atualizar preview por revisão :: Renderizar snapshot do conteúdo após mudança sem reabrir todos os editores. | apps/desktop/src/board/preview-cache.ts | Mover cartão não invalida cache textual; editar texto invalida o preview correto.
[Implementação] Limitar montagem por visibilidade :: Destruir instâncias passivas e preservar somente estado necessário. | apps/desktop/src/board/editor-pool.ts | Sessões repetidas não acumulam editores/observers ocultos.
[Implementação] Criar modo de acessibilidade :: Oferecer navegação em lista/estrutura correspondente à mesa. | apps/desktop/src/board/accessible-view.ts | Conteúdo e relações essenciais são acessíveis sem depender exclusivamente do WebGL.
[Teste] Executar matriz de input visual :: Testar zoom, DPI, janelas e seleção atravessando mudanças de viewport. | tests/board/editor-overlay/ | Nenhuma perda de texto/seleção crítica nos cenários homologados.
[Teste] Medir limite ativo :: Abrir grande board, alternar edição e contar instâncias/memória. | reports/board/editor-pool.json | Limite aprovado é respeitado e consumo não cresce linearmente por editores inativos.
''')

add(19,'Concluir navegação, LOD e workspaces','Whiteboard / performance','Próprio',['R01','R09','R11','R12','R31'],['S16','S17','S18'],
'Entregar a Mesa como etapa de exploração independente da IA.','v0.3.0 com pílulas, grupos, conexões e LOD.','Navegar entre boards de 100/1.000 nós e reconstruir a cena após reinício.',r'''
[Implementação] Criar galeria de workspaces :: Abrir, criar, renomear e arquivar pílulas por identidade. | apps/desktop/src/workspaces/ | Trocar workspace salva/expõe pendências e não mistura seleção/conteúdo entre boards.
[Implementação] Implementar culling :: Excluir da renderização elementos fora da câmera preservando dados do domínio. | apps/desktop/src/board/culling.ts | Nó invisível continua persistido e retorna corretamente ao viewport.
[Implementação] Implementar níveis de detalhe :: Usar previews, títulos e proxies conforme zoom e orçamento. | apps/desktop/src/board/lod.ts | Mudança de LOD não altera conteúdo, tamanho persistido ou alvo de interação.
[Implementação] Controlar assets/texturas :: Carregar imagens por demanda e liberar recursos gráficos não usados. | apps/desktop/src/board/texture-cache.ts | Alternar boards pesados não mantém todas as texturas residentes.
[Implementação] Otimizar relações :: Atualizar somente conectores afetados e aplicar estratégia para arestas fora da tela. | apps/desktop/src/board/edge-renderer.ts | Mover um nó não recalcula toda a cena sem necessidade demonstrada.
[Implementação] Criar operações em lote :: Duplicar ocorrências, alinhar e agrupar com uma operação reversível. | crates/board/src/batch.rs | Cancelar/falhar não deixa lote parcialmente aplicado sem recuperação.
[Implementação] Salvar viewport como preferência :: Separar navegação pessoal da topologia autoral quando apropriado. | crates/board/src/view_state.rs | Pan sem mudança de conhecimento não gera ruído indevido no histórico autoral.
[Teste] Medir cenas representativas :: Rodar benchmark de 100/1.000 nós, imagens e arestas densas. | reports/v0.3/frame-times.json | Perfil documenta p95/frame time, recursos e limitações; regressões bloqueiam a tag.
[Teste] Validar isolamento de workspace :: Abrir vários boards e tentar resolver referências fora do escopo permitido. | tests/board/workspace-scope/ | UI e broker respeitam política mesmo com cache de board anterior ainda existente.
[Release] Publicar alpha da Mesa :: Gravar demo, limites, guia de teclado e relatório de reconstrução. | release/v0.3.0/ | Produto permite exploração espacial sem modelos, rede ou conta.
''')

add(20,'Construir editor da Peça e layout editorial','BlockSuite / Peça','Integração',['R04','R10','R13','R31'],['S15','S19'],
'Criar produção textual formal sem misturá-la ao estado visual da Mesa.','Peça independente com painel editorial e navegação.','Abrir Peça vinculada ao workspace e editar enquanto consulta células.',r'''
[Contrato] Definir entidade Peça :: Fixar identidade, revisões, vínculo com workspace e estado editorial. | schemas/piece-v0.json | Peça não é apenas uma célula com flag; pode ser exportada/reaberta independentemente do layout.
[Implementação] Criar lifecycle da Peça :: Novo, abrir, renomear, arquivar e restaurar pelo Core. | crates/piece/src/lifecycle.rs | Paths e nomes não alteram identidade nem colidem silenciosamente.
[Implementação] Montar editor editorial :: Reutilizar facade BlockSuite com perfil e comandos apropriados à redação. | apps/desktop/src/piece/editor.ts | Não se cria outro motor de texto com codec divergente do usado nas notas.
[Implementação] Criar painel dockado :: Abrir/retrair/redimensionar respeitando largura e foco do canvas. | apps/desktop/src/piece/dock.ts | Redimensionar não altera conteúdo nem dispara eventos espaciais de cartões.
[Implementação] Criar outline e navegação :: Derivar headings da Peça e navegar por blocos estáveis. | apps/desktop/src/piece/outline.ts | Reordenar headings atualiza navegação sem quebrar IDs de citações preservados.
[Implementação] Integrar autosave editorial :: Usar fila do Core e estado claro de publicação durável. | apps/desktop/src/piece/save.ts | Fechar painel não destrói edição pendente; conflito externo usa o mesmo protocolo.
[Implementação] Criar estados de produção :: Rascunho/finalizado ou estados aprovados, sem impor workflow excessivo. | crates/piece/src/editorial_state.rs | Estado é metadado canônico e não depende do índice de busca.
[Implementação] Preparar área de copiloto :: Criar região de propostas desativada sem modelo, sem bloquear editor. | apps/desktop/src/piece/copilot-shell.ts | Ausência de IA mantém funcionalidade editorial completa.
[Teste] Testar documentos longos :: Abrir Peças com listas, tabelas e citações além da viewport. | tests/piece/long-documents/ | Salvamento/exportação inclui todo o documento e não só blocos montados.
[Teste] Validar edição simultânea de contextos :: Alternar foco Mesa/Peça e inspecionar revisões. | reports/piece/focus-and-save.md | Comando de uma área não modifica documento da outra.
''')

add(21,'Implementar fork-on-insert e cadeia de evidências','ProvenanceStore / Peça','Próprio',['R02','R05','R13','R37'],['S17','S20'],
'Preservar origem sem tornar a Peça dependente de conteúdo vivo de uma célula.','Inserção independente com referência de revisão e localização.','Arrastar célula, editar a fonte e comprovar que a Peça conserva o trecho inserido.',r'''
[Contrato] Definir evento de inserção :: Registrar célula/revisão, fragmento, evidência e bloco de destino. | schemas/cell-forked-into-piece.json | Referência não se resume a ID+data e pode apontar ao texto efetivamente inserido.
[Implementação] Capturar snapshot da seleção :: Resolver conteúdo na revisão exata antes de clonar. | crates/provenance/src/snapshot.rs | Edição concorrente da célula não altera o fragmento entre preview e confirmação.
[Implementação] Clonar blocos com nova identidade :: Gerar IDs de destino e manter origem separada. | crates/piece/src/fork_insert.rs | Editar Peça não escreve de volta na célula e vice-versa.
[Implementação] Persistir texto citado e hash :: Conservar trecho e localizador correspondente no formato aprovado. | crates/provenance/src/quote.rs | Texto citado pode ser verificado mesmo depois de renomear arquivo original.
[Implementação] Encadear fonte primária :: Propagar referências de recorte até material/asset original quando existentes. | crates/provenance/src/chain.rs | Célula derivada não é apresentada como fonte primária independente.
[Implementação] Renderizar citações clicáveis :: Navegar ao trecho/revisão, com estado para fonte ausente. | apps/desktop/src/piece/citations.ts | Clique não abre path externo sem autorização nem substitui silenciosamente revisão antiga.
[Implementação] Tratar atualização da fonte :: Mostrar divergência e oferecer comparação, sem atualização automática da Peça. | apps/desktop/src/piece/source-diff.ts | Mudança posterior da fonte não modifica a citação histórica.
[Implementação] Reverter inserção :: Desfazer blocos e vínculos do mesmo evento preservando histórico. | crates/piece/src/undo_insert.rs | Undo não remove citações/assets ainda usados por outro trecho.
[Teste] Testar cadeias e duplicações :: Inserir mesma célula várias vezes e citar recortes aninhados. | tests/provenance/fork/ | Cada ocorrência possui destino próprio e origem inequívoca.
[Teste] Verificar referências após reconstrução :: Apagar DB e caches editoriais e resolver todas as citações. | reports/provenance/rebuild.json | Cadeia é reconstruível exclusivamente dos arquivos e objetos canônicos.
''')

add(22,'Entregar produção local, histórico e exportação','Peça / autoria completa','Próprio',['R02','R05','R07','R13','R29','R31'],['S20','S21'],
'Fechar o primeiro ciclo de trabalho completo sem depender de IA ou web.','v0.4.0: notas → Mesa → Peça, histórico e exportação Markdown.','Produzir ensaio local, recuperar uma revisão e exportar texto com fontes.',r'''
[Implementação] Criar navegação entre Peças :: Listar por workspace e manter contexto de consulta à Mesa. | apps/desktop/src/piece/library.ts | Uma Peça não some quando seu painel é fechado ou o board é arquivado.
[Implementação] Exibir histórico semântico :: Mostrar edições, inserções e autoria automática/humana. | apps/desktop/src/history/ | Cada entrada resolve revisão válida e não depende exclusivamente de audit_log.db.
[Implementação] Comparar revisões editoriais :: Mostrar mudanças de texto/metadados e permitir restauração como nova revisão. | apps/desktop/src/piece/revision-diff.ts | Restaurar não apaga a linha temporal nem referências necessárias.
[Implementação] Exportar Markdown portátil :: Resolver caminhos relativos e incluir referências/manifest conforme perfil. | crates/export/src/markdown.rs | Arquivo exportado abre em editor comum e preserva texto/citações suportados.
[Implementação] Exportar pacote de pesquisa :: Agrupar Peça, fontes autorizadas e mapa de proveniência opcional. | crates/export/src/research_bundle.rs | Export respeita escopo e não inclui todo o Vault por conveniência.
[Implementação] Criar snapshot de backup local :: Gerar cópia consistente com manifest e hashes sem rotina remota. | crates/backup/src/snapshot.rs | Snapshot valida após alterações posteriores no Vault ativo.
[Implementação] Criar restauração de ensaio :: Restaurar em diretório novo com confirmação e validação. | crates/backup/src/restore.rs | Nenhum arquivo existente é sobrescrito silenciosamente.
[Teste] Validar ciclo integral offline :: Capturar nota manual, organizar, inserir na Peça, revisar e exportar com rede bloqueada. | reports/v0.4/offline-journey.md | Todos os passos essenciais funcionam sem credenciais/modelos.
[Teste] Executar recuperação de autoria :: Interromper durante inserção e restauração, depois reconstruir caches. | tests/piece/recovery/ | Operação fica anterior ou recuperável, nunca com texto e citação divergentes sem detecção.
[Release] Publicar alpha funcional local :: Documentar limitações e preparar pacote com evidências. | release/v0.4.0/ | Mantenedor consegue usar o app para trabalho real sem habilitar funcionalidades ainda não seguras.
''')
