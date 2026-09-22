# Technical Research: Fundação do Vault e Protocolo de Persistência

**Feature**: `002-vault-foundation` | **Date**: 2026-09-22

Este documento consolida as decisões técnicas, justificativas e análise de alternativas para a implementação do Sprint 01 (Fundação do Vault, Persistência Atômica e Confinamento Anti-TOCTOU) em estrita conformidade com a Constituição Sandland v2.0.0 e a RFC de Integração e Desconstrução.

---

## 1. Estrutura Canônica do Vault e Ciclo de Vida (`lifecycle.rs`)

### Decisão
O Vault será inicializado com um manifesto canônico obrigatório `vault.json` localizado na raiz, acompanhado de uma estrutura de diretórios padronizada:
- `vault.json`: Documento raiz contendo `vault_id` (UUID v4), `vault_name`, `schema_version: "2.0.0"`, `created_at` (ISO-8601 UTC) e `compatibility_flags`.
- Diretórios canônicos: `assets/`, `ingest/web/`, `ingest/media/`, `ingest/notes/`, `taxonomy/`, `intentions/`, `workspaces/`.
- Histórico canônico: `.history/events/` (logs append-only `.jsonl`) e `.history/objects/` (snapshots imutáveis por hash).
- Caches derivados: `.system/cache/` (ignorado em backups/syncs).

**Regra de Abertura Segura**: Ao tentar abrir uma pasta arbitrária, o sistema DEVE verificar a existência e a conformidade do `vault.json`. Se o arquivo não existir ou o JSON estiver inválido/incompatível, a operação é rejeitada com `VaultError::InvalidRootDirectory` sem modificar nem injetar qualquer arquivo na pasta do usuário.

### Rationale
- O manifesto `vault.json` estabelece a identidade do cofre independentemente do caminho físico no disco (permitindo que o usuário mova a pasta sem perder associações de sistema).
- A separação formal entre arquivos canônicos e caches derivados `.system/cache/` assegura que o usuário possa excluir a pasta `.system` a qualquer momento para liberar espaço sem sofrer perda autoral.
- A recusa precoce de pastas inválidas impede a contaminação acidental de diretórios do usuário (ex.: pasta de Downloads ou Desktop).

### Alternativas Consideradas
- **Pasta oculta tipo `.obsidian` ou `.sandland` como raiz**: Rejeitada porque a Constituição exige formatos abertos e visibilidade explícita de dados para o usuário final, com o manifesto visível como um cidadão de primeira classe do Vault.
- **Banco SQLite único como cofre**: Rejeitado sumariamente por violar o Princípio Constitucional I (File-as-Truth & Soberania do Dado).

---

## 2. Protocolo de Persistência Atômica em Duas Fases (`file-commit` / `writer.rs` e `journal.rs`)

### Decisão
Implementar a máquina de estados formal `file-commit` em quatro etapas sequenciais:
1. **Fase 1 (Journaling Durável)**: Antes de tocar em qualquer arquivo canônico, a operação é serializada em um arquivo de log diário em `.history/events/YYYY-MM-DD.jsonl`. O arquivo de journal recebe `FlushFileBuffers` (Windows) ou `sync_all` / `fdatasync` (Unix) para garantir persistência em mídia não volátil.
2. **Fase 2 (Escrita Temporária Isolada)**: O payload do arquivo de destino (ex.: `board.canvas.json` ou `cell.md`) é gravado em um arquivo temporário no mesmo diretório ou volume do destino (`.tmp_<uuid>`).
3. **Fase 3 (Substituição Atômica)**: O arquivo temporário é substituído no caminho de destino via primitiva atômica de SO (`std::fs::rename` ou `MoveFileExW` com flag `MOVEFILE_REPLACE_EXISTING` no Windows).
4. **Fase 4 (Confirmação e Evento Assíncrono)**: O escritor único emite evento em memória para os indexadores derivados (SQLite, FTS, caches MPK) para atualização assíncrona desacoplada.

**Garantia de 500 ms**: Operações de escrita na interface gráfica acumulam deltas em memória e despacham commits duráveis via canal MPSC assíncrono gerenciado por uma thread dedicada de I/O a cada 350-500 ms de digitação ativa.

### Rationale
- Elimina a possibilidade de arquivos corrompidos ou pela metade em caso de queda de energia ou travamento abrupto da máquina.
- A sincronização física (`fsync`/`FlushFileBuffers`) antes do `rename` garante que os blocos de dados estejam gravados antes que o ponteiro de diretório seja atualizado.
- Manter o journal em formato de texto aberto `.jsonl` permite que o histórico seja lido, auditado ou recuperado por qualquer ferramenta padrão sem exigir bibliotecas proprietárias.

### Alternativas Consideradas
- **Sobrescrita direta de arquivos (`truncate + write`)**: Rejeitada porque uma queda de energia no milissegundo intermediário corrompe irremediavelmente o arquivo existente no disco.
- **SQLite como único journal de transações**: Rejeitada porque o RFC Seção 6 demonstrou que classificar o banco de auditoria SQLite como arquivo efêmero enquanto dependia dele para recuperação criava uma contradição de autoridade. O journal canônico deve ser aberto em `.history/`.

---

## 3. Content-Addressable Storage (CAS) e Deduplicação (`cas.rs`)

### Decisão
Todos os binários, PDFs, imagens e mídias capturados pelo Ingest ou anexados às células serão armazenados sob o diretório `assets/`:
- **Nomenclatura**: `assets/<sha256_hex>.<ext_normalizada>`, onde `<sha256_hex>` é o hash SHA-256 de 64 caracteres hexadecimais do conteúdo binário bruto.
- **Deduplicação Proativa**: Ao receber um arquivo para armazenamento, o sistema calcula seu SHA-256 em streaming. Se o arquivo já existir em `assets/`, a escrita em disco é ignorada e a referência existente é retornada imediatamente.
- **Imutabilidade**: Arquivos gravados no CAS recebem atributo de somente-leitura e nunca são modificados no local.
- **Referenciamento**: Células e notas referenciam o ativo via esquema canônico Sandland: `sandland-asset://<sha256_hex>.<ext>` ou caminho relativo normalizado `../assets/<sha256_hex>.<ext>`.

### Rationale
- Elimina duplicação de dados ao capturar repetidamente o mesmo documento ou mídia.
- Garante integridade referencial imutável: o hash no nome do arquivo atesta matematicamente que o conteúdo não foi adulterado.
- Permite validação instantânea de corrupção de disco recalculando o hash do arquivo.

### Alternativas Consideradas
- **Nomes originais de arquivos com timestamps (ex.: `assets/foto_2026.png`)**: Rejeitada por gerar conflitos de nomes, consumir espaço com duplicatas e perder rastreabilidade em caso de substituição.
- **Sharding de diretórios com subpastas (ex.: `assets/ab/cd/abcd...`)**: Avaliada; optou-se por raiz plana `assets/` no Sprint 01 para máxima simplicidade e facilidade de inspeção pelo usuário, com sharding programado para o marco de escala v1.2 (ADR-017).

---

## 4. Proteção Anti-TOCTOU e Confinamento de Diretório (`broker.rs`)

### Decisão
Implementar o `VaultGuard` (mediação de I/O) para interceptar e validar toda operação no sistema de arquivos, eliminando vulnerabilidades de Time-of-Check to Time-of-Use (TOCTOU):
1. **Rejeição Precoce**:
   - Rejeitar caminhos absolutos arbitrários, letras de volume diferentes ou esquemas URI não autorizados.
   - Rejeitar componentes de transversão de diretório (`..`) e caracteres nulos antes de qualquer operação no SO.
2. **Resolução Vinculada a Descritores (Anti-TOCTOU Real)**:
   - **No Linux**: Abrir a raiz do cofre como descritor de diretório (`dirfd`) e utilizar primitivas de abertura relativa (`openat` com flag `O_NOFOLLOW` ou `openat2` com `RESOLVE_BENEATH`), impedindo que symlinks conduzam a leitura/escrita para fora do cofre.
   - **No Windows**: Abrir handle do diretório raiz com `FILE_FLAG_BACKUP_SEMANTICS`. Ao resolver o caminho relativo, inspecionar cada componente garantindo que nenhum reparse point (`FILE_ATTRIBUTE_REPARSE_POINT` / Junction / Symlink) aponte para um alvo cujo caminho canônico final escape da raiz do cofre.
   - Se qualquer componente violar o confinamento, disparar `SecurityError::SandboxEscapeAttempt`.

### Rationale
- A sequência tradicional `canonicalize -> starts_with -> File::open` sofre de uma janela de corrida (race window) onde um atacante ou processo externo substitui um diretório intermediário por um link simbólico entre a validação e a abertura física.
- Vincular a abertura ao descritor de diretório ou validar handles abertos com restrições garante que a operação atômica no kernel do SO respeite o perímetro seguro.

### Alternativas Consideradas
- **Apenas `canonicalize` no Rust standard library**: Rejeitada sumariamente após análise detalhada no RFC Seção 12, que demonstrou a existência de TOCTOU na implementação preliminar.
- **Chroot / Jail completo em nível de sistema**: Rejeitada por exigir privilégios de administrador/root que um aplicativo desktop comum de usuário não deve requerer.

---

## 5. Rotina de Recuperação Idempotente no Boot (`recovery.rs`)

### Decisão
Durante a inicialização do cofre (`VaultStore::open`), o sistema executará uma rotina de recuperação em 3 passos:
1. **Varredura de Arquivos Temporários Orfãos**: Localizar arquivos com padrão `.tmp_*` em todos os diretórios do cofre. Se o arquivo temporário tiver mais de 10 segundos sem alteração, ele é purgado com segurança.
2. **Reconciliação do Journal**: Ler as últimas entradas do journal em `.history/events/`. Para cada operação registrada com status `Pending` ou `Committed`:
   - Verificar se o arquivo de destino correspondente existe e possui hash/tamanho condizente com o payload confirmado.
   - Se a operação foi confirmada no journal mas a substituição do arquivo de destino foi interrompida antes do rename, aplicar a publicação a partir do snapshot correspondente em `.history/objects/`.
3. **Idempotência Garantida**: A rotina pode ser executada N vezes consecutivas em qualquer estado do sistema sem introduzir efeitos colaterais nem corrupção de dados.

### Rationale
- Cumpre o requisito funcional FR-006 e a meta de perda máxima $\le 500\text{ ms}$ (SC-002 e SC-003).
- Proporciona transparência total ao usuário: após uma falha de energia, o aplicativo reabre sem telas de pânico ou diálogos complexos de reparo manual.
