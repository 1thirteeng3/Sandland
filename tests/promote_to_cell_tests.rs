use sandland_lib::domain::workspace::topology::{BoardTopology, CanvasNode};
use sandland_lib::infra::fs::canvas_io::CanvasStorage;
use sandland_lib::infra::fs::ingest_storage::IngestStorage;
use sandland_lib::infra::fs::lifecycle::init_vault;
use sandland_lib::infra::fs::vault::VaultGuard;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_fork_on_insert_cell_promotion() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("cofre_promote");
    init_vault(&root, "Cofre Fork-on-Insert").unwrap();

    let guard = VaultGuard::new(&root).expect("Falha ao inicializar VaultGuard");

    // 1. Cria a nota física de origem no acervo
    let note_title = "Documento de Arquitetura Original";
    let original_content = "# Documento de Arquitetura\n\nTexto original e imutável que deve ser preservado.";
    let (item, _) = IngestStorage::save_note_content(&guard, note_title, original_content)
        .expect("Falha ao salvar nota original");

    let original_file_path = guard.resolve_relative_path(std::path::Path::new(&item.canonical_uri)).unwrap();
    assert!(original_file_path.exists());

    // 2. Promove a nota para uma célula no Canvas (Workspace 'ws-alpha')
    let workspace_id = "ws-alpha";
    let node_id = format!("node-{}", item.id);
    let cell_node = CanvasNode {
        id: node_id.clone(),
        title: Some(item.title.clone()),
        content: Some(original_content.to_string()),
        x: 100.0,
        y: 100.0,
        width: 300.0,
        height: 200.0,
        color_preset: Some("blue".to_string()),
        item_id: Some(item.id.clone()),
        local_cell_path: Some(item.canonical_uri.clone()),
        node_type: Some("note".to_string()),
    };

    let mut topology = CanvasStorage::load_topology(&guard, workspace_id)
        .unwrap_or_else(|_| BoardTopology::new(workspace_id.to_string()));
    topology.nodes.push(cell_node);
    CanvasStorage::save_topology(&guard, workspace_id, &topology)
        .expect("Falha ao salvar topologia");

    // 3. Modifica a célula na Mesa Espacial (Fork-on-Insert)
    let mut updated_topology = CanvasStorage::load_topology(&guard, workspace_id).unwrap();
    let node = updated_topology.nodes.iter_mut().find(|n| n.id == node_id).unwrap();
    node.content = Some("Texto da célula MODIFICADO no Canvas para fins de síntese e ideação.".to_string());
    CanvasStorage::save_topology(&guard, workspace_id, &updated_topology)
        .expect("Falha ao atualizar topologia");

    // 4. Valida que o arquivo original em ingest/notes/ permanece 100% INTACTO
    let note_on_disk = fs::read_to_string(&original_file_path).unwrap();
    assert!(
        note_on_disk.contains("Texto original e imutável que deve ser preservado."),
        "O arquivo físico original da nota NÃO deve ser alterado após modificações na célula da Mesa"
    );
    assert!(
        !note_on_disk.contains("MODIFICADO no Canvas"),
        "Conteúdo forkeado não pode vazar para a nota original no disco"
    );
}
