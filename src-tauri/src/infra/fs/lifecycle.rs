use chrono::Utc;
use std::fs;
use std::path::Path;
use crate::domain::core::errors::{VaultError, VaultResult};
use crate::domain::model::VaultManifest;
use crate::infra::fs::write_file_atomic;

pub const CURRENT_SCHEMA_VERSION: &str = "2.0.0";
pub const MANIFEST_FILENAME: &str = "vault.json";

/// Lista canônica de diretórios estruturados do Sandland Vault
pub const CANONICAL_DIRECTORIES: &[&str] = &[
    "assets",
    "workspaces",
    "ingest/web",
    "ingest/media",
    "ingest/notes",
    "taxonomy",
    "intentions",
    ".history/events",
    ".history/objects",
    ".system/cache",
];

/// Inicializa um novo cofre no caminho indicado
pub fn init_vault(root_path: &Path, vault_name: &str) -> VaultResult<VaultManifest> {
    if !root_path.exists() {
        fs::create_dir_all(root_path).map_err(|e| {
            VaultError::Io(format!("Falha ao criar diretório raiz do cofre: {}", e))
        })?;
    } else if !root_path.is_dir() {
        return Err(VaultError::InvalidRootDirectory(root_path.to_path_buf()));
    }

    let manifest_path = root_path.join(MANIFEST_FILENAME);
    if manifest_path.exists() {
        return Err(VaultError::Busy(format!(
            "O cofre já está inicializado neste diretório: {:?}",
            manifest_path
        )));
    }

    // Cria as pastas canônicas da estrutura Sandland
    for dir in CANONICAL_DIRECTORIES {
        let dir_path = root_path.join(dir);
        fs::create_dir_all(&dir_path).map_err(|e| {
            VaultError::Io(format!(
                "Falha ao criar pasta canônica '{dir}': {e}"
            ))
        })?;
    }

    let manifest = VaultManifest::new(vault_name.to_string());
    let serialized = serde_json::to_vec_pretty(&manifest)?;

    write_file_atomic(&manifest_path, &serialized).map_err(|e| {
        VaultError::Io(format!("Falha ao gravar manifesto do cofre: {}", e))
    })?;

    Ok(manifest)
}

/// Valida rigorosamente se um diretório é um cofre Sandland íntegro sem mutações no disco
pub fn validate_vault(root_path: &Path) -> VaultResult<VaultManifest> {
    if !root_path.exists() || !root_path.is_dir() {
        return Err(VaultError::InvalidRootDirectory(root_path.to_path_buf()));
    }

    let manifest_path = root_path.join(MANIFEST_FILENAME);
    if !manifest_path.exists() {
        return Err(VaultError::ManifestNotFound(manifest_path));
    }

    let content = fs::read(&manifest_path).map_err(|e| {
        VaultError::Io(format!("Falha ao ler {MANIFEST_FILENAME}: {e}"))
    })?;

    let manifest: VaultManifest = serde_json::from_slice(&content).map_err(|e| {
        VaultError::ManifestCorrupted(format!(
            "Falha ao decodificar JSON do manifesto: {e}"
        ))
    })?;

    if manifest.schema_version != CURRENT_SCHEMA_VERSION {
        return Err(VaultError::IncompatibleVersion {
            found: manifest.schema_version,
            required: CURRENT_SCHEMA_VERSION.to_string(),
        });
    }

    Ok(manifest)
}

/// Abre um cofre existente validado e atualiza o timestamp da última sessão
pub fn open_vault(root_path: &Path) -> VaultResult<VaultManifest> {
    let mut manifest = validate_vault(root_path)?;
    manifest.last_opened_at = Utc::now();

    let manifest_path = root_path.join(MANIFEST_FILENAME);
    let serialized = serde_json::to_vec_pretty(&manifest)?;
    write_file_atomic(&manifest_path, &serialized)?;

    Ok(manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_init_and_validate_vault() {
        let dir = tempdir().unwrap();
        let root = dir.path().join("sandland_vault");

        let manifest = init_vault(&root, "Meu Cofre de Pesquisa").unwrap();
        assert_eq!(manifest.vault_name, "Meu Cofre de Pesquisa");
        assert_eq!(manifest.schema_version, CURRENT_SCHEMA_VERSION);

        // Verifica existência de diretórios canônicos
        for d in CANONICAL_DIRECTORIES {
            assert!(root.join(d).exists(), "Diretório {} deve existir", d);
        }

        // Validação sem mutação
        let validated = validate_vault(&root).unwrap();
        assert_eq!(validated.vault_id, manifest.vault_id);

        // Abertura com atualização de timestamp
        let opened = open_vault(&root).unwrap();
        assert_eq!(opened.vault_id, manifest.vault_id);
    }

    #[test]
    fn test_open_arbitrary_folder_fails_safely() {
        let dir = tempdir().unwrap();
        let arbitrary_root = dir.path().join("pasta_qualquer");
        fs::create_dir_all(&arbitrary_root).unwrap();
        fs::write(arbitrary_root.join("foto.png"), b"dados").unwrap();

        let res = validate_vault(&arbitrary_root);
        assert!(res.is_err());
        match res.unwrap_err() {
            VaultError::ManifestNotFound(_) => {}
            other => panic!("Esperava ManifestNotFound, recebeu: {:?}", other),
        }

        // Assegura que nenhum arquivo foi criado ou alterado
        assert!(!arbitrary_root.join(MANIFEST_FILENAME).exists());
        assert_eq!(fs::read(arbitrary_root.join("foto.png")).unwrap(), b"dados");
    }
}
