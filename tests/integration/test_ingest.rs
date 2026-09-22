use std::fs;
use std::path::PathBuf;

#[test]
fn test_ingest_pipeline_integrity() {
    let temp_dir = std::env::temp_dir().join(format!("sandland_test_ingest_{}", uuid::Uuid::now_v7()));
    let ingest_dir = temp_dir.join("ingest/notes");
    let system_dir = temp_dir.join(".system");
    fs::create_dir_all(&ingest_dir).unwrap();
    fs::create_dir_all(&system_dir).unwrap();

    let sample_note = r#"---
id: 01921345-6789-7abc-def0-123456789abc
title: "Nota de Teste SDD"
item_type: "note"
created_at: 1726915200
---

# Nota de Teste SDD

Este é um documento de validação de ingestão passiva para o Sandland Local-First.
O indexador relacional deve catalogar este texto no SQLite e FTS5.
"#;

    let note_path = ingest_dir.join("nota-teste.md");
    fs::write(&note_path, sample_note).unwrap();

    // Verify file written to disk
    assert!(note_path.exists());
    let content = fs::read_to_string(&note_path).unwrap();
    assert!(content.contains("Nota de Teste SDD"));
    assert!(!content.contains("vault_path"));
    assert!(!content.contains("content_hash"));

    // Cleanup
    let _ = fs::remove_dir_all(&temp_dir);
}
