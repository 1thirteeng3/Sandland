use chrono::Utc;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use crate::domain::core::errors::{VaultError, VaultResult};
use crate::domain::model::AssetRef;
use crate::infra::fs::write_file_atomic;

pub const ASSETS_DIR: &str = "assets";

/// Normaliza uma extensão de arquivo removendo pontos iniciais e convertendo para minúsculas
pub fn normalize_extension(ext: &str) -> String {
    let trimmed = ext.trim().trim_start_matches('.').to_lowercase();
    if trimmed.is_empty() {
        "bin".to_string()
    } else {
        trimmed
    }
}

/// Calcula o hash SHA-256 em streaming para um buffer de bytes
pub fn hash_bytes_sha256(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

/// Armazena um ativo no Content-Addressable Storage (CAS) com deduplicação proativa
pub fn store_cas_asset(root_path: &Path, bytes: &[u8], extension: &str) -> VaultResult<AssetRef> {
    let assets_dir = root_path.join(ASSETS_DIR);
    if !assets_dir.exists() {
        fs::create_dir_all(&assets_dir).map_err(|e| {
            VaultError::Io(format!("Falha ao criar diretório de assets: {e}"))
        })?;
    }

    let ext = normalize_extension(extension);
    let hash = hash_bytes_sha256(bytes);
    let filename = format!("{}.{}", hash, ext);
    let target_path = assets_dir.join(&filename);

    // Deduplicação: se o arquivo já existe no CAS, confirma sem reescrever
    if !target_path.exists() {
        write_file_atomic(&target_path, bytes)?;

        // Marca arquivo como somente-leitura para garantir imutabilidade
        if let Ok(metadata) = fs::metadata(&target_path) {
            let mut permissions = metadata.permissions();
            #[allow(clippy::permissions_set_readonly_modify)]
            permissions.set_readonly(true);
            let _ = fs::set_permissions(&target_path, permissions);
        }
    }

    Ok(AssetRef {
        sha256_hash: hash.clone(),
        byte_size: bytes.len() as u64,
        extension: ext.clone(),
        created_at: Utc::now(),
        canonical_uri: format!("sandland-asset://{}.{}", hash, ext),
    })
}

/// Localiza o caminho em disco de um ativo pelo seu hash SHA-256
pub fn resolve_asset_path(root_path: &Path, sha256_hash: &str) -> VaultResult<PathBuf> {
    let assets_dir = root_path.join(ASSETS_DIR);
    if !assets_dir.exists() {
        return Err(VaultError::AssetNotFound(sha256_hash.to_string()));
    }

    let entries = fs::read_dir(&assets_dir).map_err(|e| {
        VaultError::Io(format!("Falha ao listar diretório de assets: {e}"))
    })?;

    for entry_res in entries {
        let entry = entry_res.map_err(|e| VaultError::Io(e.to_string()))?;
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with(sha256_hash) {
                    return Ok(path);
                }
            }
        }
    }

    Err(VaultError::AssetNotFound(sha256_hash.to_string()))
}

/// Lê o conteúdo de um ativo do CAS e valida sua integridade criptográfica
pub fn read_cas_asset(root_path: &Path, sha256_hash: &str) -> VaultResult<Vec<u8>> {
    let asset_path = resolve_asset_path(root_path, sha256_hash)?;
    let bytes = fs::read(&asset_path).map_err(|e| {
        VaultError::Io(format!("Falha ao ler ativo em {:?}: {}", asset_path, e))
    })?;

    let calculated_hash = hash_bytes_sha256(&bytes);
    if calculated_hash != sha256_hash {
        return Err(VaultError::AssetCorrupted {
            calculated: calculated_hash,
            expected: sha256_hash.to_string(),
        });
    }

    Ok(bytes)
}
