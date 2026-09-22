use sandland_lib::domain::core::errors::SecurityError;
use sandland_lib::infra::fs::lifecycle::init_vault;
use sandland_lib::infra::security::broker::VaultGuard;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_block_relative_path_traversal() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("cofre_seguro");
    init_vault(&root, "Cofre Seguro").unwrap();

    let guard = VaultGuard::new(&root).expect("Falha ao inicializar VaultGuard");

    // Vetores de transversão clássica '../'
    let malicious_paths = [
        Path::new("../fora.txt"),
        Path::new("workspaces/../../arquivo_secreto.key"),
        Path::new("assets/../../../etc/shadow"),
        Path::new(".."),
        Path::new("notes/../../.."),
    ];

    for path in malicious_paths {
        let res = guard.resolve_relative_path(path);
        assert!(
            res.is_err(),
            "Caminho com '..' deveria ter sido rejeitado: {:?}",
            path
        );
        match res.unwrap_err() {
            SecurityError::SandboxEscapeAttempt(_) => {}
            other => panic!("Esperava SandboxEscapeAttempt para {:?}, obteve {:?}", path, other),
        }
    }
}

#[test]
fn test_block_absolute_paths() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("cofre_seguro_abs");
    init_vault(&root, "Cofre Seguro Abs").unwrap();

    let guard = VaultGuard::new(&root).expect("Falha ao inicializar VaultGuard");

    #[cfg(windows)]
    let absolute_paths = [
        Path::new(r"C:\Windows\System32\cmd.exe"),
        Path::new(r"\\servidor\share\arquivo.txt"),
        Path::new(r"D:\arquivo.txt"),
    ];

    #[cfg(not(windows))]
    let absolute_paths = [
        Path::new("/etc/passwd"),
        Path::new("/var/log/syslog"),
        Path::new("/root/.ssh/id_rsa"),
    ];

    for path in absolute_paths {
        let res = guard.resolve_relative_path(path);
        assert!(
            res.is_err(),
            "Caminho absoluto deveria ter sido rejeitado: {:?}",
            path
        );
        match res.unwrap_err() {
            SecurityError::SandboxEscapeAttempt(_) => {}
            other => panic!("Esperava SandboxEscapeAttempt, obteve {:?}", other),
        }
    }
}

#[test]
fn test_block_null_bytes_and_dos_devices() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("cofre_seguro_devices");
    init_vault(&root, "Cofre Seguro Devices").unwrap();

    let guard = VaultGuard::new(&root).expect("Falha ao inicializar VaultGuard");

    let invalid_paths = [
        Path::new("pasta\0maliciosa.txt"),
        Path::new("CON"),
        Path::new("aux.txt"),
        Path::new("NUL"),
        Path::new("com1.json"),
    ];

    for path in invalid_paths {
        let res = guard.resolve_relative_path(path);
        assert!(
            res.is_err(),
            "Caminho inválido/dispositivo deveria ter sido rejeitado: {:?}",
            path
        );
        match res.unwrap_err() {
            SecurityError::InvalidPath(_) => {}
            other => panic!("Esperava InvalidPath, obteve {:?}", other),
        }
    }
}

#[test]
fn test_valid_canonical_path_resolution() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("cofre_seguro_valido");
    init_vault(&root, "Cofre Valido").unwrap();

    let guard = VaultGuard::new(&root).expect("Falha ao inicializar VaultGuard");

    let valid_paths = [
        Path::new("workspaces/ws1/board.canvas.json"),
        Path::new("notes/minha_nota.md"),
        Path::new("assets/1234567890abcdef.png"),
    ];

    for path in valid_paths {
        let resolved = guard.resolve_relative_path(path).expect("Caminho válido deveria ser aceito");
        assert!(
            resolved.starts_with(guard.vault_root()),
            "Caminho resolvido {:?} deve estar dentro da raiz {:?}",
            resolved,
            guard.vault_root()
        );
    }
}
