use std::path::Path;
use crate::domain::core::errors::SecurityError;

/// Validação e resolução de caminhos em plataformas Unix (Linux e macOS)
pub fn verify_unix_path_safety(vault_root: &Path, rel_path: &Path) -> Result<(), SecurityError> {
    let mut current = vault_root.to_path_buf();

    for component in rel_path.components() {
        match component {
            std::path::Component::Normal(part) => {
                current.push(part);
                if current.exists() {
                    if let Ok(symlink_meta) = std::fs::symlink_metadata(&current) {
                        if symlink_meta.file_type().is_symlink() {
                            if let Ok(target) = std::fs::read_link(&current) {
                                let resolved = if target.is_absolute() {
                                    target
                                } else {
                                    current.parent().unwrap_or(vault_root).join(target)
                                };
                                if let Ok(canonical_resolved) = resolved.canonicalize() {
                                    if let Ok(canonical_root) = vault_root.canonicalize() {
                                        if !canonical_resolved.starts_with(&canonical_root) {
                                            return Err(SecurityError::SandboxEscapeAttempt(
                                                format!("Symlink Unix em {:?} escapa dos limites: {:?}", current, canonical_resolved)
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            std::path::Component::ParentDir => {
                return Err(SecurityError::SandboxEscapeAttempt(
                    "Path traversal '..' não permitido".to_string(),
                ));
            }
            _ => {}
        }
    }
    Ok(())
}
