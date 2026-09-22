# Contract: VaultStore & Tauri IPC Interface

**Feature**: `002-vault-foundation` | **Date**: 2026-09-22

Este contrato define a trait do `VaultStore` em Rust e a interface de comandos IPC do Tauri v2 correspondente exposta ao frontend.

---

## 1. Rust Trait: `VaultStore`

```rust
use std::path::{Path, PathBuf};
use async_trait::async_trait;
use crate::domain::model::{VaultManifest, AssetRef, JournalRecord};
use crate::domain::core::errors::VaultResult;

#[async_trait]
pub trait VaultStore: Send + Sync {
    /// Inicializa um novo cofre na pasta indicada (deve estar vazia ou conter apenas arquivos permitidos).
    async fn init_vault(&self, root_path: &Path, name: &str) -> VaultResult<VaultManifest>;

    /// Valida e abre um cofre existente lendo o manifesto `vault.json`.
    async fn open_vault(&self, root_path: &Path) -> VaultResult<VaultManifest>;

    /// Fecha o cofre ativo, liberando handles de arquivo e garantindo flush de journals pendentes.
    async fn close_vault(&self) -> VaultResult<()>;

    /// Obtém os metadados do manifesto do cofre atualmente ativo.
    async fn get_manifest(&self) -> VaultResult<VaultManifest>;

    /// Armazena um arquivo binário no CAS por hash SHA-256 com deduplicação.
    async fn store_asset(&self, bytes: &[u8], extension: &str) -> VaultResult<AssetRef>;

    /// Lê os bytes de um ativo do CAS pelo seu hash SHA-256 verificado.
    async fn read_asset(&self, sha256_hash: &str) -> VaultResult<Vec<u8>>;

    /// Executa uma escrita atômica em duas fases via `file-commit`.
    async fn commit_file(
        &self,
        rel_path: &Path,
        payload: &[u8],
        expected_revision: Option<u64>,
    ) -> VaultResult<u64>;

    /// Lê o conteúdo de um arquivo canônico validado pelo VaultGuard.
    async fn read_file(&self, rel_path: &Path) -> VaultResult<Vec<u8>>;

    /// Executa a rotina idempotente de recuperação no boot.
    async fn recover_vault(&self) -> VaultResult<RecoveryReport>;
}
```

---

## 2. Tauri IPC Commands

Todos os comandos IPC utilizam parâmetros e retornos fortemente tipados via `serde`.

### `init_vault`
- **Assinatura**: `init_vault(path: String, name: String) -> Result<VaultManifestDto, String>`
- **Descrição**: Cria a estrutura de pastas e grava `vault.json`.
- **Erros**: Retorna erro se a pasta não for vazia ou se já existir um cofre inválido.

### `open_vault`
- **Assinatura**: `open_vault(path: String) -> Result<VaultManifestDto, String>`
- **Descrição**: Valida a presença de `vault.json`, confere versão e aciona `recover_vault`.
- **Erros**: Retorna erro claro se o manifesto não existir (sem alterar arquivos no disco).

### `commit_document`
- **Assinatura**: `commit_document(rel_path: String, content: String, expected_revision: Option<u64>) -> Result<u64, String>`
- **Descrição**: Serializa evento no journal diário e aplica substituição atômica no arquivo de destino.
- **Erros**: Retorna erro se `expected_revision` divergir da revisão existente no disco (conflito).

### `store_asset`
- **Assinatura**: `store_asset(bytes_base64: String, extension: String) -> Result<AssetRefDto, String>`
- **Descrição**: Salva arquivo binário no CAS e retorna hash e caminho normalizado.

### `get_vault_status`
- **Assinatura**: `get_vault_status() -> Result<VaultStatusDto, String>`
- **Descrição**: Retorna se há um cofre ativo, total de arquivos canônicos e métricas de integridade.
