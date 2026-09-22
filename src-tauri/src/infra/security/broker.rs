use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use crate::domain::core::errors::SecurityError;
use crate::infra::fs::write_file_atomic;
#[cfg(not(windows))]
use crate::infra::security::platform_unix::verify_unix_path_safety;
use crate::infra::security::platform_windows::verify_windows_path_safety;

pub struct VaultGuard {
    vault_root: PathBuf,
}

impl VaultGuard {
    /// Inicializa a jaula de confinamento sobre a raiz do cofre verificada
    pub fn new<P: Into<PathBuf>>(vault_root: P) -> Result<Self, SecurityError> {
        let root_buf = vault_root.into();
        let canonical_root = root_buf.canonicalize().map_err(|e| {
            SecurityError::InvalidPath(format!("Falha ao resolver raiz do cofre: {e}"))
        })?;
        Ok(Self {
            vault_root: canonical_root,
        })
    }

    pub fn vault_root(&self) -> &Path {
        &self.vault_root
    }

    pub fn base_path(&self) -> &Path {
        &self.vault_root
    }

    /// Valida o caminho léxico, bloqueando '..', caminhos absolutos, letras de drive e caracteres perigosos
    pub fn validate_lexical_path(rel_path: &Path) -> Result<(), SecurityError> {
        if rel_path.is_absolute() {
            return Err(SecurityError::SandboxEscapeAttempt(
                "Caminhos absolutos são proibidos".to_string(),
            ));
        }

        let path_str = rel_path.to_string_lossy();
        if path_str.contains('\0') {
            return Err(SecurityError::InvalidPath(
                "Byte nulo detectado no caminho".to_string(),
            ));
        }

        for component in rel_path.components() {
            match component {
                Component::Prefix(_) => {
                    return Err(SecurityError::SandboxEscapeAttempt(
                        "Prefixos de unidade de disco são proibidos".to_string(),
                    ));
                }
                Component::RootDir => {
                    return Err(SecurityError::SandboxEscapeAttempt(
                        "Raiz de sistema proibida em caminhos relativos".to_string(),
                    ));
                }
                Component::ParentDir => {
                    return Err(SecurityError::SandboxEscapeAttempt(
                        "Tentativa de transversão '..' detectada".to_string(),
                    ));
                }
                Component::Normal(segment) => {
                    let seg_str = segment.to_string_lossy().to_uppercase();
                    let reserved = [
                        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5",
                        "COM6", "COM7", "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4",
                        "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
                    ];
                    if reserved.iter().any(|&r| seg_str == r || seg_str.starts_with(&format!("{}.", r))) {
                        return Err(SecurityError::InvalidPath(format!(
                            "Nome de dispositivo reservado do sistema operacional: {seg_str}"
                        )));
                    }
                }
                Component::CurDir => {}
            }
        }

        Ok(())
    }

    /// Resolve um caminho relativo validando que ele não escapa do perímetro do cofre
    pub fn resolve_relative_path(&self, rel_path: &Path) -> Result<PathBuf, SecurityError> {
        Self::validate_lexical_path(rel_path)?;

        #[cfg(windows)]
        verify_windows_path_safety(&self.vault_root, rel_path)?;

        #[cfg(not(windows))]
        verify_unix_path_safety(&self.vault_root, rel_path)?;

        let full_target = self.vault_root.join(rel_path);

        if full_target.exists() {
            if let Ok(canonical) = full_target.canonicalize() {
                if !canonical.starts_with(&self.vault_root) {
                    return Err(SecurityError::SandboxEscapeAttempt(format!(
                        "Caminho canônico {:?} escapa do cofre {:?}",
                        canonical, self.vault_root
                    )));
                }
            }
        }

        Ok(full_target)
    }

    /// Abre um arquivo de forma segura contra TOCTOU vinculada ao confinamento
    pub fn open_read(&self, rel_path: &Path) -> Result<File, SecurityError> {
        let safe_target = self.resolve_relative_path(rel_path)?;
        File::open(&safe_target).map_err(|e| {
            SecurityError::InvalidPath(format!("Falha ao abrir arquivo para leitura: {e}"))
        })
    }

    /// Abre um arquivo de forma segura contra TOCTOU retornando o handle e o caminho canônico
    pub fn secure_open_read(&self, rel_path: &Path) -> Result<(File, PathBuf), SecurityError> {
        let safe_target = self.resolve_relative_path(rel_path)?;
        let file = File::open(&safe_target).map_err(|e| {
            SecurityError::InvalidPath(format!("Falha ao abrir arquivo para leitura: {e}"))
        })?;
        Ok((file, safe_target))
    }

    /// Cria/abre um arquivo para escrita atômica garantindo confinamento
    pub fn open_write_atomic(&self, rel_path: &Path) -> Result<File, SecurityError> {
        let safe_target = self.resolve_relative_path(rel_path)?;
        if let Some(parent) = safe_target.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&safe_target)
            .map_err(|e| {
                SecurityError::InvalidPath(format!("Falha ao abrir arquivo para escrita: {e}"))
            })
    }

    /// Lê o conteúdo textual completo de um arquivo validado
    pub fn secure_read_to_string(&self, rel_path: &Path) -> Result<String, SecurityError> {
        let mut file = self.open_read(rel_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| {
            SecurityError::InvalidPath(format!("Falha ao ler conteúdo do arquivo: {e}"))
        })?;
        Ok(content)
    }

    /// Grava conteúdo de forma segura e atômica dentro do cofre
    pub fn secure_write(&self, rel_path: &Path, content: &[u8]) -> Result<PathBuf, SecurityError> {
        let target = self.resolve_relative_path(rel_path)?;
        if let Some(parent) = target.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        write_file_atomic(&target, content).map_err(|e| {
            SecurityError::InvalidPath(format!("Falha ao gravar arquivo atomicamente: {e}"))
        })?;
        Ok(target)
    }

    /// Grava uma nota Markdown com Frontmatter YAML padronizado
    pub fn write_markdown_note(
        &self,
        relative_path: &Path,
        id: &str,
        title: &str,
        item_type: &str,
        category: Option<&str>,
        tags: &[String],
        summary: Option<&str>,
        body: &str,
    ) -> Result<PathBuf, SecurityError> {
        let content = crate::infra::fs::vault::format_markdown_with_frontmatter(
            id, title, item_type, category, tags, summary, body,
        );
        self.secure_write(relative_path, content.as_bytes())
    }
}
