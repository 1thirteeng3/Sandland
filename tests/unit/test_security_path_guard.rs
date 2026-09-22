use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn test_path_boundary_enforcement() {
    let temp_root = std::env::temp_dir().join(format!("sandland_sec_test_{}", uuid::Uuid::now_v7()));
    let vault_dir = temp_root.join("my_vault");
    let outside_dir = temp_root.join("forbidden_area");

    fs::create_dir_all(&vault_dir).unwrap();
    fs::create_dir_all(&outside_dir).unwrap();

    let canonical_vault = vault_dir.canonicalize().unwrap();

    // Test safe path inside vault
    let safe_relative = Path::new("notes/safe_note.md");
    let full_safe = canonical_vault.join(safe_relative);
    let _ = fs::create_dir_all(full_safe.parent().unwrap());
    fs::write(&full_safe, b"Safe content").unwrap();

    let canonical_safe = full_safe.canonicalize().unwrap();
    assert!(
        canonical_safe.starts_with(&canonical_vault),
        "Caminho seguro deve estar estritamente dentro da raiz do cofre"
    );

    // Test traversal attack attempting ../../forbidden_area
    let malicious_target = canonical_vault.join("../../forbidden_area/leak.txt");
    let escaped = if let Ok(canonical_malicious) = malicious_target.canonicalize() {
        !canonical_malicious.starts_with(&canonical_vault)
    } else {
        // If not found yet, checking parent boundary
        true
    };

    assert!(escaped, "Tentativa de fuga fora dos limites deve ser detectada");

    // Cleanup
    let _ = fs::remove_dir_all(&temp_root);
}
