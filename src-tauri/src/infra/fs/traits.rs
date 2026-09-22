use async_trait::async_trait;
use std::path::Path;
use crate::domain::core::errors::VaultResult;
use crate::domain::model::{AssetRef, RecoveryReport, VaultManifest};

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
