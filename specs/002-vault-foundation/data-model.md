# Data Model: Fundação do Vault e Protocolo de Persistência

**Feature**: `002-vault-foundation` | **Date**: 2026-09-22

Este documento formaliza as entidades, atributos, relacionamentos, esquemas JSON e máquinas de estado de persistência do Sprint 01 do Sandland.

---

## 1. Diagrama de Relacionamentos (ERD)

```text
  ┌─────────────────────────────────────────────────────────────┐
  │                         VaultManifest                       │
  │  vault_id: UUID                                             │
  │  vault_name: String                                         │
  │  schema_version: String ("2.0.0")                           │
  │  created_at: DateTime<Utc>                                  │
  │  last_opened_at: DateTime<Utc>                              │
  └──────────────────────────────┬──────────────────────────────┘
                                 │ 1:N
        ┌────────────────────────┼────────────────────────┐
        ▼                        ▼                        ▼
┌──────────────────┐    ┌──────────────────┐    ┌──────────────────┐
│    Workspace     │    │      Asset       │    │  JournalRecord   │
│  workspace_id    │    │  sha256_hash     │    │  event_id        │
│  name            │    │  byte_size       │    │  timestamp       │
│  topology_ref    │    │  extension       │    │  operation_kind  │
│  created_at      │    │  created_at      │    │  target_path     │
└────────┬─────────┘    └──────────────────┘    │  revision        │
         │ 1:N                                  │  payload_hash    │
         ▼                                      └──────────────────┘
┌──────────────────┐                                      │ 1:1
│     CellFile     │                                      ▼
│  cell_id: UUID   │                            ┌──────────────────┐
│  title           │                            │  ObjectSnapshot  │
│  frontmatter     │                            │  object_hash     │
│  markdown_body   │                            │  content_bytes   │
└──────────────────┘                            │  stored_at       │
                                                └──────────────────┘
```

---

## 2. Especificação das Entidades

### 2.1 `VaultManifest` (`vault.json`)
Representa o cabeçalho canônico e o atestado de integridade do cofre.

- **`vault_id`** (`Uuid` / `String`): Identificador único global e imutável (UUID v4).
- **`vault_name`** (`String`): Nome amigável escolhido pelo usuário para exibição na interface.
- **`schema_version`** (`String`): Versão do layout canônico do Sandland (fixado em `"2.0.0"`).
- **`created_at`** (`DateTime<Utc>`): Timestamp de criação do cofre em ISO-8601 UTC.
- **`last_opened_at`** (`DateTime<Utc>`): Timestamp da última sessão aberta pelo aplicativo.
- **`compatibility`** (`CompatibilityFlags`):
  - `min_supported_app_version`: Versão mínima do Sandland requerida para ler o cofre.
  - `features_enabled`: Lista de extensões ativas (`["cas_sha256", "journal_v2", "board_json"]`).

**Esquema JSON (`vault.json`)**:
```json
{
  "vault_id": "4a7b9c1d-8f2e-4b6a-9c3d-1e5f7a9b2c4d",
  "vault_name": "Pesquisa Pessoal",
  "schema_version": "2.0.0",
  "created_at": "2026-09-22T10:00:00Z",
  "last_opened_at": "2026-09-22T12:00:00Z",
  "compatibility": {
    "min_supported_app_version": "0.1.0",
    "features_enabled": ["cas_sha256", "journal_v2", "board_json"]
  }
}
```

---

### 2.2 `Asset` (Content-Addressable Storage)
Representa qualquer arquivo binário ou mídia armazenado de forma imutável e deduplicada.

- **`sha256_hash`** (`String`): Hash SHA-256 de 64 caracteres hexadecimais em minúsculas (chave primária natural).
- **`byte_size`** (`u64`): Tamanho exato do arquivo binário em bytes.
- **`extension`** (`String`): Extensão normalizada do arquivo (ex.: `"png"`, `"pdf"`, `"mp4"`).
- **`mime_type`** (`String`): Tipo MIME inferido por sniff de cabeçalho mágico.
- **`created_at`** (`DateTime<Utc>`): Momento da primeira gravação no CAS.
- **Caminho Físico Canônico**: `assets/<sha256_hash>.<extension>`

---

### 2.3 `JournalRecord` (`.history/events/YYYY-MM-DD.jsonl`)
Representa cada mutação autoral sequencial confirmada no cofre antes de sua publicação no disco.

- **`event_id`** (`Uuid` / `String`): Identificador único do evento (UUID v7 monotônico com timestamp embutido).
- **`timestamp`** (`DateTime<Utc>`): Timestamp UTC em microssegundos.
- **`op_kind`** (`OperationKind`): Tipo de operação:
  - `CreateFile`: Criação de novo arquivo canônico.
  - `UpdateFile`: Atualização de conteúdo com nova revisão.
  - `DeleteFile`: Remoção lógica/física de arquivo.
  - `MoveFile`: Renomeação ou transferência de pasta.
- **`target_path`** (`String`): Caminho relativo ao cofre (ex.: `"workspaces/ws-01/board.canvas.json"`).
- **`expected_revision`** (`Option<u64>`): Versão prévia esperada para validação de concorrência otimista.
- **`new_revision`** (`u64`): Nova versão sequencial gerada para o item.
- **`payload_sha256`** (`String`): Hash SHA-256 do conteúdo a ser publicado.
- **`snapshot_ref`** (`Option<String>`): Apontador para o objeto correspondente em `.history/objects/<hash>`.
- **`state`** (`JournalState`): `Committed` ou `Applied`.

**Exemplo de Linha JSONL**:
```json
{"event_id":"01921a8c-4f70-7123-8abc-9876543210ab","timestamp":"2026-09-22T12:05:00.123456Z","op_kind":"UpdateFile","target_path":"workspaces/ws-01/board.canvas.json","expected_revision":4,"new_revision":5,"payload_sha256":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855","snapshot_ref":"e3b0c44298fc1c14...","state":"Committed"}
```

---

### 2.4 `ObjectSnapshot` (`.history/objects/<hash>`)
Armazena a cópia imutável exata de cada revisão de documento ou topologia para possibilitar rollback, auditoria e recuperação após travamento.

- **`object_hash`** (`String`): Hash SHA-256 dos bytes do snapshot (nome do arquivo).
- **`content_bytes`** (`Vec<u8>`): Conteúdo exato comprimido com zstd ou armazenado bruto.
- **`stored_at`** (`DateTime<Utc>`): Timestamp da geração do snapshot.

---

## 3. Máquina de Estados do Ciclo de Persistência (`file-commit`)

```text
    ┌───────────────┐
    │     Start     │
    └───────┬───────┘
            │ 1. Validar expected_revision
            ▼
    ┌───────────────┐
    │ PreparedTmp   │──► [Gravar payload em .tmp_<uuid> no mesmo volume]
    └───────┬───────┘
            │ 2. Serializar evento e fsync no journal
            ▼
    ┌───────────────┐
    │   Committed   │──► [Garantido: perda máxima <= 500 ms cumprida]
    └───────┬───────┘
            │ 3. Atomic rename (.tmp_<uuid> -> target)
            ▼
    ┌───────────────┐
    │    Applied    │──► [Arquivo canônico publicado no disco]
    └───────┬───────┘
            │ 4. Emitir DomainEvent assíncrono para indexadores
            ▼
    ┌───────────────┐
    │   Completed   │
    └───────────────┘
```

**Comportamento em Falha**:
- Queda antes de `Committed`: A operação é descartada; o arquivo anterior no disco permanece 100% intacto; o arquivo temporário é purgado no boot.
- Queda entre `Committed` e `Applied`: O boot detecta que o evento foi confirmado no journal mas o arquivo canônico ainda está desatualizado; a rotina de recuperação reaplica o snapshot imediatamente de forma idempotente.
