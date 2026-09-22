# Feature Specification: Núcleo Local-First e Canvas Básico (v0.1)

**Feature Branch**: `001-local-first-canvas`

**Created**: 2026-09-21

**Status**: Draft

**Input**: User description: "Gere a especificação focada APENAS na "v0.1: O Núcleo Local-First e Canvas Básico" baseada no meu PRD. O escopo deve cobrir os requisitos funcionais, jornadas de usuário e critérios de aceite para a ingestão de documentos (Ingest), auto-tagging local via SLM e o Canvas infinito (PixiJS) básico. Ignore detalhes de implementação em Rust e SQLite, foque no comportamento esperado do usuário final."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Ingestão Transparente de Documentos (Priority: P1)

Como um pesquisador ou escritor analítico, quero adicionar notas rápidas, artigos de texto e arquivos de notas ao sistema de forma imediata e passiva (via arrastar-e-soltar ou salvamento direto na pasta de entrada do cofre local), sem ser interrompido por formulários de classificação burocrática, para que eu não perca o fluxo criativo e saiba que meus dados estão salvos com segurança no meu próprio computador.

**Why this priority**: A ingestão sem atrito é a porta de entrada de todo o ciclo de conhecimento. Sem a capacidade de receber e preservar arquivos de forma autônoma e segura, nenhuma outra etapa analítica pode ocorrer.

**Independent Test**: Pode ser testado adicionando arquivos de texto ou notas brutas na pasta de entrada ou soltando arquivos na janela do aplicativo; o usuário deve vê-los listados no repositório de itens recebidos sem perda de dados e com visualização imediata do conteúdo original.

**Acceptance Scenarios**:

1. **Given** o aplicativo está aberto e o usuário possui um arquivo de texto/nota em seu desktop, **When** o usuário arrasta e solta o arquivo na área de captura do sistema, **Then** o documento é absorvido imediatamente, mantendo seu texto integral, registrando a data de entrada e ficando disponível para leitura e exploração.
2. **Given** o usuário salva uma nova nota diretamente na pasta monitorada de entrada no disco, **When** o aplicativo detecta a presença do novo arquivo, **Then** o item é adicionado automaticamente à lista de materiais ingeridos sem exigir reinicialização ou sincronização manual pelo usuário.
3. **Given** um documento é adicionado, **When** o usuário consulta as propriedades do arquivo, **Then** o sistema confirma que o arquivo reside localmente no disco, preservando formatação Markdown e quaisquer anexos originais.

---

### User Story 2 - Auto-Tagging Inteligente e Classificação Local (Priority: P1)

Como um usuário que acumula dezenas de fontes de pesquisa, quero que o sistema analise o conteúdo dos documentos ingeridos utilizando inteligência artificial local e privada, sugerindo automaticamente categorias e etiquetas temáticas com reaproveitamento de termos já existentes, para que eu mantenha meu acervo ordenado sem gastar tempo com catalogação manual repetitiva.

**Why this priority**: O volume de material capturado torna-se inútil sem indexação semântica. O auto-tagging automatiza a organização preliminar, garantindo que o usuário encontre conexões conceituais sem esforço e com total privacidade.

**Independent Test**: Pode ser testado adicionando 5 notas sobre temas correlatos; o sistema deve atribuir automaticamente categorias e tags coerentes reutilizando termos comuns e exibindo os rótulos atribuídos nos cartões das notas.

**Acceptance Scenarios**:

1. **Given** uma nova nota conceitual sobre um tema específico é ingerida, **When** o assistente de inteligência local conclui a análise de conteúdo em segundo plano, **Then** o documento recebe uma categoria primária, resumo de uma frase e tags temáticas padronizadas.
2. **Given** o acervo já possui uma taxonomia de termos existentes (ex.: "concorrência", "design-systems"), **When** uma nova nota com assunto similar é processada, **Then** o sistema prioriza o reaproveitamento dos termos existentes em vez de criar variações redundantes ou sinônimos conflitantes.
3. **Given** o sistema enfrenta um documento ambíguo, muito ruidoso ou ocorre indisponibilidade momentânea da análise inteligente, **When** o processo de classificação atinge o tempo limite de segurança, **Then** o documento é salvo com sucesso e marcado visualmente com um indicador discreto de "Revisão Manual Necessária", permitindo ao usuário definir as etiquetas com um clique.

---

### User Story 3 - Mesa de Trabalho Espacial / Canvas Infinito (Priority: P2)

Como um analista visual, quero abrir uma tela infinita (Whiteboard) onde possa visualizar minhas notas como cartões atômicos, posicioná-los livremente em duas dimensões e criar conexões visuais direcionadas entre eles, para estruturar espacialmente minhas linhas de raciocínio e hipóteses de trabalho.

**Why this priority**: A cognição humana é espacial. Ter os cartões visíveis em um mapa livre permite identificar padrões, agrupar conceitos e preparar a estrutura de ensaios ou relatórios analíticos de maneira intuitiva.

**Independent Test**: Pode ser testado abrindo o canvas, criando ou arrastando nós da lista de entrada para a mesa, movimentando-os pela tela e desenhando conexões entre nós distintos; ao fechar e reabrir o aplicativo, todas as posições e links devem permanecer exatamente onde foram deixados.

**Acceptance Scenarios**:

1. **Given** o usuário está com o canvas aberto, **When** arrasta um documento da área de entrada para uma coordenada vazia da mesa, **Then** o documento é instanciado como um cartão visual exibindo título, resumo e etiquetas.
2. **Given** existem dois cartões no canvas, **When** o usuário conecta a borda do cartão A ao cartão B através de um gesto de arrasto, **Then** uma linha de conexão visual é criada e persistida entre ambos.
3. **Given** o usuário navega pelo canvas através de gestos de deslocamento panorâmico (*pan*) e rolagem de ampliação (*zoom*), **When** percorre distâncias grandes no espaço infinito, **Then** a navegação responde suavemente sem travamentos perceptíveis.
4. **Given** o usuário modifica as posições dos cartões e fecha o aplicativo, **When** reabre a mesma área de trabalho, **Then** todas as coordenadas, nós e conexões são restaurados de forma idêntica.

---

### User Story 4 - Navegação Fluida com Múltiplos Níveis de Detalhe (Priority: P3)

Como um usuário com um grande acervo de pesquisa visual, quero que a exibição dos cartões no canvas adapte automaticamente sua densidade visual de acordo com o nível de zoom, para que eu mantenha visibilidade do todo em visões panorâmicas e veja detalhes textuais quando focar em cartões específicos.

**Why this priority**: Sustenta a experiência de longo prazo. Conforme a quantidade de nós cresce para centenas no mesmo espaço, a sobrecarga de renderização e a poluição visual prejudicariam a usabilidade se todos os cartões mostrassem sempre o texto completo.

**Independent Test**: Pode ser testado populando o canvas com mais de 50 cartões e alternando suavemente entre o zoom máximo e o zoom panorâmico; o nível de texto e detalhes deve transicionar de forma transparente e responsiva.

**Acceptance Scenarios**:

1. **Given** o usuário está com zoom próximo (foco de leitura), **When** visualiza um cartão, **Then** o cartão renderiza título, formatação de texto completa e etiquetas.
2. **Given** o usuário reduz o zoom para uma visão intermediária, **When** a escala diminui, **Then** o corpo de texto longo é ocultado, mantendo visíveis apenas o título, a cor indicativa do tipo de nota e o contorno do cartão.
3. **Given** o usuário reduz o zoom ao nível panorâmico global (visão macro), **When** os cartões ocupam proporções reduzidas na tela, **Then** são exibidos como blocos sólidos simplificados de alta visibilidade, garantindo que o movimento da câmera permaneça instantâneo.

---

### Edge Cases

- O que acontece se o usuário arrastar um arquivo corrompido, vazio (0 bytes) ou em formato binário incompatível para a área de ingestão?
  - O sistema deve notificar o usuário com uma mensagem clara e amigável, impedindo a poluição do catálogo e mantendo o arquivo na pasta de assets sem quebrar o índice de busca.
- Como o sistema se comporta caso o computador do usuário esteja com carga de trabalho intensa ou memória escassa durante a classificação por IA?
  - O processamento de inteligência artificial deve executar com prioridade de segundo plano sem congelar a interface do usuário. Caso a inferência não responda em tempo hábil, o sistema deve acionar o mecanismo de contingência heurística e marcar o item para revisão sem interromper o trabalho ativo do usuário.
- O que acontece se o aplicativo for encerrado repentinamente (falha de energia ou fechamento forçado) enquanto o usuário movimenta cartões ou digita notas no canvas?
  - O sistema deve restaurar o estado da área de trabalho quase imediatamente a partir do último registro local de alta frequência, com tolerância de perda inferior a um segundo de interação.
- O que acontece se forem criados dois cartões em coordenadas exatamente sobrepostas no canvas?
  - O sistema deve aplicar um leve deslocamento visual automático (*smart offset*) ao soltar o segundo cartão ou destacar a sobreposição para facilitar a seleção individual.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O sistema DEVE permitir a ingestão passiva de documentos de texto e notas em formato Markdown através de arrastar-e-soltar e via detecção automática em pasta monitorada local.
- **FR-002**: O sistema DEVE armazenar todo o conteúdo textual do usuário em arquivos Markdown no disco local, preservando metadados estruturados de cabeçalho (data de criação, título, tags, categoria e status).
- **FR-003**: O sistema DEVE executar a classificação e o auto-tagging dos documentos de forma 100% local no computador do usuário, sem transmitir dados para servidores externos ou serviços em nuvem.
- **FR-004**: O sistema DEVE analisar e associar automaticamente a cada documento novo uma categoria temática, até cinco tags contextuais e um resumo sintético de uma frase.
- **FR-005**: O sistema DEVE verificar a base de tags já existentes no cofre antes de propor novas tags, priorizando a consolidação de termos já utilizados pelo usuário em vez de introduzir redundâncias taxonômicas.
- **FR-006**: O sistema DEVE fornecer um mecanismo de contingência graciosa para falhas de inferência de IA, atribuindo etiquetas preliminares baseadas em termos do próprio texto e sinalizando visualmente os itens que demandam revisão manual.
- **FR-007**: O usuário DEVE poder editar, adicionar ou remover manualmente qualquer tag ou categoria atribuída pelo sistema a qualquer momento através da interface.
- **FR-008**: O sistema DEVE oferecer uma área de trabalho visual infinita bidimensional (Canvas/Whiteboard) com suporte completo a movimentação panorâmica (*pan*) e ampliação contínua (*zoom*).
- **FR-009**: O usuário DEVE poder criar notas diretamente no canvas, bem como adicionar documentos ingeridos à mesa de trabalho na forma de cartões atômicos.
- **FR-010**: O usuário DEVE poder reposicionar livremente os cartões no espaço bidimensional e criar conexões visuais direcionadas entre eles.
- **FR-011**: O sistema DEVE persistir a posição exata, dimensões e conexões de todos os cartões da área de trabalho continuamente, restaurando a cena de forma idêntica ao reabrir o aplicativo.
- **FR-012**: A interface do canvas DEVE adaptar automaticamente a densidade de conteúdo dos cartões em pelo menos 3 níveis visuais (detalhado, intermediário e panorâmico simplificado) de acordo com o fator de zoom da visualização.
- **FR-013**: O sistema DEVE manter o consumo de recursos sob controle, liberando modelos de inteligência artificial da memória ativa após um período estipulado de inatividade de análise.

### Key Entities

- **Documento Ingerido (Item de Ingest)**: Representa qualquer nota, arquivo de texto ou material capturado pelo usuário. Possui identificador único, título, conteúdo textual, metadados de criação/modificação, resumo conceitual, categoria temática, lista de tags e estado de revisão.
- **Área de Trabalho (Workspace / Board Canvas)**: Representa um ambiente de exploração espacial independente. Contém o estado da câmera visual (coordenadas de centro e nível de zoom), coleção de nós posicionados e conjunto de conexões entre nós.
- **Cartão Atômico (Canvas Node / Célula)**: Instância visual de um documento ou nota livre posicionado no canvas. Possui coordenadas espaciais $(x, y)$, dimensões de exibição, referência ao documento de origem ou texto próprio e estado visual ativo.
- **Conexão Visual (Edge / Relação)**: Representa um vínculo semântico ou fluxo de raciocínio entre dois cartões. Possui nó de origem, nó de destino e propriedades visuais de rota.
- **Termo Taxonômico (Categoria / Tag)**: Elemento do vocabulário controlado do cofre local. Possui nome canônico padronizado e histórico de frequência de uso.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: O usuário consegue ingerir um novo documento e vê-lo disponível para leitura no sistema em menos de 1 segundo após soltar o arquivo na interface.
- **SC-002**: A classificação automática por inteligência local conclui a atribuição de categoria e tags de um documento típico (até 3000 palavras) em menos de 5 segundos no hardware do usuário.
- **SC-003**: Pelo menos 80% das tags propostas pelo assistente local em documentos subsequentes de uma mesma temática reaproveitam o vocabulário existente sem gerar termos duplicados ou variações ortográficas ruidosas.
- **SC-004**: A navegação pelo canvas (pan e zoom) mantém fluidez visual estável (sensação de 60 quadros por segundo sem engasgos) em áreas de trabalho contendo até 100 cartões e conexões ativas.
- **SC-005**: 100% dos dados textuais, anotações e topologias de canvas permanecem acessíveis e editáveis mesmo com o computador desconectado de qualquer rede de internet.
- **SC-006**: Após um encerramento inesperado do aplicativo, o estado reaberto do canvas e das notas recupera o trabalho com discrepância temporal inferior a 1 segundo de digitação ou movimentação.

## Assumptions

- O usuário executa o aplicativo em um computador desktop padrão (Windows, Linux ou macOS) com suporte a aceleração gráfica por hardware básica para renderização de telas interativas.
- Os modelos locais de inteligência artificial necessários para classificação são baixados e verificados de forma transparente pelo assistente na configuração inicial do aplicativo.
- O formato prioritário de documentos de texto para esta versão inicial compreende arquivos Markdown (`.md`) e texto sem formatação (`.txt`), sendo formatos avançados de terceiros (como PDFs escaneados complexos ou áudios longos) delegados para marcos subsequentes do produto.
- A sincronização remota via serviços externos ou nuvem está expressamente fora do escopo desta versão v0.1, garantindo foco na excelência da experiência local-first.
