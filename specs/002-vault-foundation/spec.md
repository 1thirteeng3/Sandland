# Feature Specification: Fundação do Vault e Protocolo de Persistência (v0.1)

**Feature Branch**: `002-vault-foundation`

**Created**: 2026-09-22

**Status**: Draft

**Input**: User description: "Especificar o Sprint 01: Fundação do Vault e Protocolo de Persistência (v0.1 - crates/vault, journal canônico e I/O anti-TOCTOU)"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Inicialização, Criação e Abertura Segura de Vaults (Priority: P1)

Como um pesquisador ou autor analítico, quero criar um novo cofre de conhecimento em qualquer pasta local do meu computador (ou abrir um cofre existente) com validação transparente de integridade, para que meus documentos e ativos sejam armazenados sob minha total soberania e eu possa ter certeza de que uma pasta inválida ou incompatível jamais será corrompida.

**Why this priority**: A autoridade soberana e a gestão do ciclo de vida do cofre local formam o alicerce fundamental de todo o ecossistema Sandland. Sem a capacidade de criar, inspecionar e abrir cofres com validação de esquema, nenhuma operação de escrita, leitura ou exploração pode existir.

**Independent Test**: Pode ser testado criando um novo cofre em uma pasta vazia e verificando a geração de sua estrutura canônica e manifesto com identificador único; em seguida, abrindo uma pasta arbitrária sem manifesto para atestar a recusa segura sem alteração de arquivos; e, finalmente, reabrindo o cofre válido para atestar o reconhecimento imediato de sua identidade.

**Acceptance Scenarios**:

1. **Given** uma pasta vazia no disco do usuário, **When** o usuário comanda a criação de um novo cofre, **Then** o sistema gera a estrutura de diretórios canônica, grava o manifesto raiz com identificador global único e versão do formato, e disponibiliza o cofre para operações de autoria.
2. **Given** uma pasta existente contendo arquivos do usuário que não constituem um cofre Sandland, **When** o usuário comanda a abertura dessa pasta como cofre, **Then** o sistema detecta a ausência do manifesto válido, recusa a operação com mensagem explicativa e não altera nem adiciona nenhum arquivo na pasta.
3. **Given** um cofre existente íntegro, **When** o usuário o abre na aplicação, **Then** o sistema valida a versão do esquema, autentica os parâmetros do cofre e carrega suas configurações operacionais instantaneamente.
4. **Given** uma tentativa de mover ou renomear o diretório do cofre no sistema operacional, **When** o usuário reabre o cofre a partir de seu novo caminho, **Then** o sistema reconhece a identidade imutável do cofre por meio de seu identificador persistente sem perda de dados.

---

### User Story 2 - Persistência Atômica e Imunidade a Travamentos / Quedas de Energia (Priority: P1)

Como um escritor e analista focado em produção contínua, quero que todas as minhas notas, células e modificações sejam salvas de forma durável e atômica com garantia de recuperação em caso de travamento do aplicativo ou corte repentino de energia, para que eu nunca perca mais do que uma fração de segundo do meu trabalho e o cofre nunca fique em estado corrompido ou inconsistente.

**Why this priority**: A perda silenciosa de dados ou corrupção de arquivos é a falha mais grave possível em uma ferramenta de pensamento e autoria local-first. Garantir atomicidade em duas fases e limite máximo de perda de 500 ms é a salvaguarda de confiabilidade inegociável do produto.

**Independent Test**: Pode ser testado realizando escritas e atualizações de notas enquanto se injetam interrupções forçadas do processo (kill brusco) e verificando que, na reinicialização subsequente, o cofre é restaurado de forma 100% íntegra para o último estado estável confirmado, sem resíduos temporários ou arquivos truncados.

**Acceptance Scenarios**:

1. **Given** um documento ou nota em edição contínua, **When** novas alterações são digitadas e confirmadas, **Then** os dados são registrados de forma durável com protocolo de duas fases, assegurando uma perda máxima de digitação de no máximo 500 milissegundos em caso de falha repentina de energia.
2. **Given** um desligamento abrupto durante a fase de escrita de um documento, **When** o aplicativo é reiniciado, **Then** a rotina de recuperação audita o histórico de operações, descarta arquivos temporários parciais e restaura o documento para o estado canônico exato da última confirmação durável.
3. **Given** duas operações simultâneas tentando modificar o mesmo documento ou topologia sob a mesma revisão base, **When** a segunda tentativa tenta confirmar suas alterações, **Then** o sistema detecta o conflito de revisão e impede a sobrescrita cega, sinalizando a necessidade de reconciliação de conteúdo.

---

### User Story 3 - Armazenamento de Anexos por Conteúdo (CAS) e Deduplicação Imutável (Priority: P2)

Como um pesquisador que incorpora imagens, PDFs e mídias em suas notas, quero que meus anexos sejam armazenados de forma imutável e deduplicada pelo seu hash de conteúdo, para que referências a mídias nunca se quebrem e arquivos repetidos não consumam espaço desnecessário no meu disco.

**Why this priority**: Documentos analíticos dependem fortemente de evidências visuais e anexos de pesquisa. O armazenamento endereçado por conteúdo (CAS) assegura imutabilidade dos dados brutos, integridade referencial permanente e economia de armazenamento por deduplicação nativa.

**Independent Test**: Pode ser testado adicionando o mesmo arquivo binário em notas diferentes ou em momentos distintos e verificando que apenas uma cópia física reside no armazenamento de ativos, com todas as referências apontando de forma consistente para o identificador correspondente.

**Acceptance Scenarios**:

1. **Given** uma nota em edição, **When** o usuário anexa um arquivo binário, **Then** o arquivo é armazenado no repositório de ativos com nome derivado de seu hash SHA-256 e o link canônico correspondente é gerado na nota.
2. **Given** um ativo com hash idêntico a um anexo já presente no cofre é adicionado novamente em outro ponto do sistema, **When** a gravação é solicitada, **Then** o sistema reutiliza o ativo existente sem duplicar os bytes físicos no disco e confirma a operação com sucesso.
3. **Given** uma nota que referencia um anexo no cofre, **When** o usuário renomeia a nota ou a transfere entre workspaces, **Then** a integridade referencial com o anexo é mantida inalterada sem geração de links quebrados.

---

### User Story 4 - Confinamento de Acesso e Proteção Anti-TOCTOU (Priority: P2)

Como um usuário consciente de segurança que armazena notas confidenciais, quero ter a garantia de que qualquer rotina em segundo plano, ferramenta de processamento ou agente automatizado fique estritamente restrita aos limites autorizados do cofre, impedindo que atalhos maliciosos (symlinks/junctions), caminhos relativos manipulados ou corridas no sistema de arquivos acessem arquivos fora da área permitida.

**Why this priority**: A segurança e o isolamento de dados são cláusulas pétreas da Constituição do Sandland. Evitar vulnerabilidades de tempo de verificação para tempo de uso (TOCTOU) e tentativas de fuga do cofre garante a proteção da privacidade do usuário diante de agentes e arquivos complexos.

**Independent Test**: Pode ser testado disparando solicitações de leitura e escrita contendo symlinks apontando para fora do cofre, caminhos com saltos relativos (`../`) e tentativas concorrentes de renomeação de diretórios durante a abertura; o sistema deve rejeitar categoricamente todas as tentativas e manter o confinamento absoluto.

**Acceptance Scenarios**:

1. **Given** uma solicitação de acesso a arquivo emitida por uma ferramenta ou agente, **When** o caminho solicitado tenta resolver referências fora dos limites do cofre (inclusive via symlinks ou caminhos com `..`), **Then** a operação é imediatamente bloqueada com erro explícito de segurança de confinamento.
2. **Given** um ambiente concorrente onde diretórios podem ser renomeados durante uma operação, **When** o sistema executa a resolução e abertura de arquivos, **Then** a operação utiliza descritores restritos ao diretório raiz autorizado, garantindo que o arquivo efetivamente aberto corresponda ao objeto legitimamente verificado.
3. **Given** um agente com permissão concedida para operar em um workspace específico, **When** tenta ler ou escrever em dados de outro workspace não autorizado, **Then** o sistema nega o acesso sumariamente com base na política de permissões de contexto.

---

### Edge Cases

- O que acontece se o disco do usuário ficar completamente sem espaço livre durante uma escrita?
  O sistema DEVE abortar a transação sem corromper o arquivo de destino original, manter o arquivo temporário isolado ou removê-lo se possível, e notificar o usuário com um alerta claro de falta de espaço sem travar a aplicação.
- Como o sistema se comporta diante de falhas de hardware ou setores defeituosos no arquivo de manifesto (`vault.json`)?
  O sistema DEVE validar a integridade estrutural do manifesto na abertura; caso encontre corrupção no arquivo principal, DEVE tentar carregar a cópia redundante/snapshot de recuperação antes de classificar o cofre como danificado e oferecer opções seguras de diagnóstico.
- Como o sistema lida com arquivos com permissões de somente-leitura impostas pelo sistema operacional externo?
  O sistema DEVE detectar a restrição de permissão antes de tentar a escrita, emitir uma mensagem legível ao usuário informando o caminho do arquivo bloqueado e orientar sobre a liberação de permissões, sem mascarar o erro.
- O que acontece se o usuário copiar manualmente arquivos para dentro da pasta canônica com nomes duplicados ou formatações fora do padrão?
  O sistema de reconciliação DEVE ler os arquivos externos sem sobrescrever silenciosamente, detectar divergências de identificadores e gerar registros de revisão preservando a integridade dos dados existentes.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O sistema DEVE fornecer mecanismos para criar e validar um novo cofre local através da inicialização de sua estrutura canônica padronizada (`assets/`, `ingest/`, `workspaces/`, `.history/`, `.system/cache/`) e geração do manifesto `vault.json` contendo identificador único global (UUID v4) e versão do formato.
- **FR-002**: O sistema DEVE recusar sumariamente a abertura de qualquer diretório como cofre Sandland caso o manifesto raiz esteja ausente, corrompido ou possua versão incompatível sem suporte de migração, sem realizar qualquer mutação no sistema de arquivos.
- **FR-003**: O sistema DEVE garantir a segregação estrita dos dados em quatro classes: Canônica (Markdown, JSON de topologia, intenções, taxonomia, CAS), Histórico Canônico (journal de eventos e objetos de revisão), Derivada (índices, SQLite, caches MPK) e Efêmera (scratchpad e temporários).
- **FR-004**: O sistema DEVE implementar persistência atômica em duas fases para todas as alterações canônicas: registro prévio da operação no log de eventos durável (`.history/events/*.jsonl`), seguido de substituição atômica de arquivos via primitiva nativa de renomeação do sistema operacional com garantia de sincronização física em disco (`fsync` / `FlushFileBuffers`).
- **FR-005**: O sistema DEVE limitar a janela de perda de dados após queda abrupta de energia ou encerramento forçado a no máximo 500 milissegundos de digitação ativa.
- **FR-006**: O sistema DEVE implementar rotina idempotente de recuperação no boot que audita a coerência entre o journal durável e os arquivos canônicos, completando operações pendentes confirmadas e purgando arquivos temporários orfãos.
- **FR-007**: O sistema DEVE armazenar todos os arquivos binários e mídias no Content-Addressable Storage (`assets/<hash>.<ext>`) endereçados estritamente pela sua impressão digital criptográfica SHA-256, realizando deduplicação transparente e automática.
- **FR-008**: O sistema DEVE gerenciar identificadores imutáveis para células, notas e peças independentemente de suas localizações físicas, garantindo que operações de renomeação ou movimentação de diretórios não quebrem vínculos internos nem alterem identidades de domínio.
- **FR-009**: O sistema DEVE aplicar mediação centralizada de I/O (`VaultGuard` / `ToolBroker`) que resolve arquivos a partir de descritores de diretório com confinamento estrito (`openat2` com resolução restrita no Linux e descritores imunes a reparse points/junctions no Windows), eliminando vulnerabilidades de tempo de verificação para tempo de uso (TOCTOU).
- **FR-010**: O sistema DEVE rejeitar qualquer operação de I/O que envolva resolução de symlinks para fora do cofre ou caminhos com transversão (`..`), emitindo evento de auditoria de segurança com o identificador `SecurityError::SandboxEscapeAttempt`.
- **FR-011**: O sistema DEVE assegurar que nenhuma thread de renderização de interface gráfica execute I/O de disco bloqueante ou síncrono, operando todas as operações de persistência e journal através de fila serializada assíncrona.
- **FR-012**: O sistema DEVE permitir a reconstrução a frio de todos os índices derivados (banco SQLite `index.db` e caches binários `board.canvas.mpk`) a partir da leitura direta dos arquivos Markdown canônicos e do `board.canvas.json`, sem perda autoral.

### Key Entities *(include if feature involves data)*

- **Vault**: Entidade raiz que representa o cofre de conhecimento do usuário. Possui identificador único imutável, caminho canônico no sistema de arquivos, versão de esquema, configurações locais e metadados de criação e última modificação.
- **Manifesto (`vault.json`)**: Documento declarativo na raiz do cofre que atesta sua identidade, versão de formato de dados, perfil de compatibilidade e configurações de segurança e retenção.
- **Asset**: Arquivo binário imutável gerenciado no CAS. Definido exclusivamente pelo seu hash criptográfico SHA-256, tamanho em bytes, extensão de mídia original e contagem de referências ativas por notas ou células.
- **Journal Event**: Registro imutável de mutação autoral gravado de forma sequencial e durável (`.jsonl`). Contém identificador sequencial, timestamp de confirmação, tipo de operação (criação, edição, exclusão, citação), identificador do item afetado, revisão base e payload da alteração.
- **Object Snapshot**: Cópia imutável versionada de um estado de documento ou topologia gravada em `.history/objects/<hash>`, possibilitando reversão e auditoria histórica completa.
- **Workspace**: Espaço isolado de trabalho analítico dentro do cofre. Agrupa uma topologia de Mesa (`board.canvas.json`), células locais, peças editoriais e rascunhos temporários.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: O tempo de inicialização e validação de abertura de um cofre existente em disco rápido (NVMe) DEVE ser inferior a 150 milissegundos.
- **SC-002**: A tolerância a falhas abruptas DEVE assegurar que nenhuma interrupção do sistema provoque perda superior a 500 milissegundos de digitação ativa confirmada.
- **SC-003**: 100% das interrupções de escrita simuladas em testes automatizados de falha devem resultar em recuperação limpa e consistente sem nenhum arquivo corrompido ou truncado.
- **SC-004**: 100% das tentativas de fuga do cofre (symlinks para `/etc` ou `C:\Windows`, caminhos com `../..` ou junctions forjadas) devem ser interceptadas e rejeitadas pelo sistema de segurança.
- **SC-005**: Adicionar múltiplos arquivos binários com o mesmo conteúdo deve resultar em exatamente 1 arquivo físico armazenado no repositório de ativos (100% de taxa de deduplicação).
- **SC-006**: A exclusão física completa de todos os índices derivados (`.system/cache/`) DEVE permitir a regeneração total do estado operacional do cofre em menos de 5 segundos para acervos com até 1.000 notas.

## Assumptions

- O usuário possui permissão de leitura e escrita no sistema de arquivos no diretório escolhido para hospedar o cofre.
- O sistema operacional subjacente oferece suporte a operações de substituição atômica de arquivos e sincronização com disco físico (`FlushFileBuffers` no Windows, `fsync` no Linux e macOS).
- Operações de sincronização em nuvem de terceiros (como Dropbox, OneDrive ou Google Drive) podem eventualmente sincronizar pastas do cofre; o design em arquivos abertos legíveis e journal sequencial visa minimizar conflitos de merge nessas ferramentas.
- O Sandland opera em modo single-user por processo local; a prevenção de concorrência destina-se a múltiplos componentes internos (threads, workers locais e agentes) operando sobre o mesmo cofre.
