# Contract: Confinamento de I/O e Mediação Anti-TOCTOU (`VaultGuard`)

**Feature**: `002-vault-foundation` | **Date**: 2026-09-22

Este contrato define a interface do mediador central de sistema de arquivos (`VaultGuard`), as regras de validação de caminhos e o mecanismo de proteção contra ataques de corrida (TOCTOU) e fuga de cofre.

---

## 1. Interface Rust: `VaultGuard`

```rust
use std::fs::File;
use std::path::{Path, PathBuf};
use crate::domain::core::errors::{SecurityError, VaultResult};

pub struct VaultGuard {
    vault_root: PathBuf,
    // No Windows: Handle do diretório raiz
    // No Linux: dirfd (File Descriptor) do diretório raiz
}

impl VaultGuard {
    /// Inicializa a jaula de confinamento sobre a raiz do cofre verificada.
    pub fn new(vault_root: &Path) -> Result<Self, SecurityError>;

    /// Resolve um caminho relativo validando que ele não escapa do perímetro do cofre.
    /// Rejeita imediatamente: caminhos absolutos, prefixos de drive, '..' e caracteres de controle.
    pub fn resolve_relative_path(&self, rel_path: &Path) -> Result<PathBuf, SecurityError>;

    /// Abre um arquivo de forma segura contra TOCTOU vinculada ao descritor da raiz.
    pub fn open_read(&self, rel_path: &Path) -> Result<File, SecurityError>;

    /// Cria/abre um arquivo para escrita atômica garantindo que symlinks não sejam seguidos para fora.
    pub fn open_write_atomic(&self, rel_path: &Path) -> Result<File, SecurityError>;
}
```

---

## 2. Invariantes de Segurança e Rejeição

1. **Tentativa de Transversão (`..`)**:
   - Qualquer segmento relativo contendo `..` é rejeitado sumariamente na validação léxica inicial:
     `Err(SecurityError::SandboxEscapeAttempt("Path traversal attempt detected"))`.
2. **Caminhos Absolutos**:
   - Qualquer argumento iniciando com `/`, `\` ou letra de unidade (ex.: `C:\`) é rejeitado imediatamente.
3. **Resolução de Links Simbólicos / Reparse Points**:
   - O mediador DEVE garantir que a resolução não atravesse junctions ou symlinks que apontem para fora da árvore do cofre. No Linux, `openat2` utiliza `RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS`. No Windows, cada nó pai é inspecionado antes da abertura física.
4. **Log de Auditoria de Segurança**:
   - Toda rejeição de acesso emite evento de segurança de nível crítico com timestamp, rota solicitada e identificador da chamada.
