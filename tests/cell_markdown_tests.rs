use sandland_lib::domain::workspace::cell::WorkspaceCell;
use sandland_lib::infra::fs::canvas_io::CanvasStorage;
use sandland_lib::infra::fs::vault::{initialize_vault_structure, VaultGuard};
use tempfile::tempdir;

#[test]
fn test_cell_markdown_frontmatter_roundtrip() {
    let mut cell = WorkspaceCell::new("cell-101".to_string(), "default-workspace".to_string());
    cell.node_id = Some("node-101".to_string());
    cell.item_id = Some("uuid-original".to_string());
    cell.source_path = Some("ingest/notes/sample.md".to_string());
    cell.source_hash = Some("hash123".to_string());
    cell.title = "Local-First Architecture".to_string();
    cell.content = "Content of the modular cell with markdown details.".to_string();
    cell.tags = vec!["architecture".to_string(), "local-first".to_string()];
    cell.revision = 3;

    let md = cell.to_markdown();
    assert!(md.starts_with("---\n"));
    assert!(md.contains("id: \"cell-101\""));
    assert!(md.contains("workspace_id: \"default-workspace\""));
    assert!(md.contains("node_id: \"node-101\""));
    assert!(md.contains("revision: 3"));
    assert!(md.contains("# Local-First Architecture"));
    assert!(md.contains("Content of the modular cell with markdown details."));

    let parsed = WorkspaceCell::from_markdown(&md).expect("Falha ao parsear markdown gerado");
    assert_eq!(parsed.id, "cell-101");
    assert_eq!(parsed.workspace_id, "default-workspace");
    assert_eq!(parsed.node_id.as_deref(), Some("node-101"));
    assert_eq!(parsed.item_id.as_deref(), Some("uuid-original"));
    assert_eq!(parsed.source_path.as_deref(), Some("ingest/notes/sample.md"));
    assert_eq!(parsed.source_hash.as_deref(), Some("hash123"));
    assert_eq!(parsed.title, "Local-First Architecture");
    assert_eq!(parsed.content, "Content of the modular cell with markdown details.");
    assert_eq!(parsed.revision, 3);
    assert_eq!(parsed.tags, vec!["architecture", "local-first"]);
}

#[test]
fn test_cell_storage_save_and_read() {
    let temp = tempdir().expect("Falha ao criar tempdir");
    initialize_vault_structure(temp.path()).expect("Falha ao inicializar cofre");
    let guard = VaultGuard::new(temp.path().to_path_buf()).expect("Falha ao criar VaultGuard");

    let frontmatter = serde_json::json!({
        "title": "Célula Persistida",
        "nodeId": "node-42",
        "tags": ["pesquisa", "protótipo"]
    });

    let (path, revision) = CanvasStorage::save_workspace_cell(
        &guard,
        "default-workspace",
        "cell-42",
        "Texto da célula salvo no disco.",
        Some(frontmatter),
    ).expect("Falha ao salvar célula");

    assert_eq!(path, "workspaces/default-workspace/cells/cell-42.md");
    assert_eq!(revision, 2);

    let read_content = CanvasStorage::read_workspace_cell(&guard, "default-workspace", "cell-42")
        .expect("Falha ao ler célula salva");
    assert!(read_content.contains("cell-42"));
    assert!(read_content.contains("node-42"));
    assert!(read_content.contains("Texto da célula salvo no disco."));
}
