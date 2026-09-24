use sandland_lib::domain::workspace::piece::{CitationRecord, EditorialPiece};
use sandland_lib::infra::fs::canvas_io::CanvasStorage;
use sandland_lib::infra::fs::piece_io::PieceStorage;
use sandland_lib::infra::fs::vault::{initialize_vault_structure, VaultGuard};
use tempfile::tempdir;

#[test]
fn test_fork_on_insert_citation_and_retroactive_immunity() {
    let temp = tempdir().expect("Falha ao criar tempdir");
    initialize_vault_structure(temp.path()).expect("Falha ao inicializar cofre");
    let guard = VaultGuard::new(temp.path().to_path_buf()).expect("Falha ao criar VaultGuard");

    // 1. Cria uma célula original no canvas
    let cell_id = "cell-concept-1";
    let original_text = "Definição do conceito original na Mesa.";
    let (_, rev1) = CanvasStorage::save_workspace_cell(
        &guard,
        "default-workspace",
        cell_id,
        original_text,
        Some(serde_json::json!({ "title": "Conceito 1" })),
    )
    .expect("Falha ao salvar célula");

    // 2. Transfere para a Peça via Fork-on-Insert
    let mut piece = EditorialPiece::new(
        "piece-thesis".to_string(),
        "default-workspace".to_string(),
        "Tese de Mestrado".to_string(),
    );

    let citation = CitationRecord::new(
        "cit-1".to_string(),
        cell_id.to_string(),
        rev1,
        "Conceito 1".to_string(),
        None,
        original_text.to_string(),
    );

    piece.citations.push(citation);
    piece.body = format!("Como visto em: <cite id=\"cit-1\">{}</cite>", original_text);
    PieceStorage::save_piece(&guard, "default-workspace", piece.clone())
        .expect("Falha ao salvar peça");

    // 3. Edita a célula original na Mesa posteriormente
    let updated_cell_text = "Nova formulação radical que contradiz o conceito original.";
    let (_, rev2) = CanvasStorage::save_workspace_cell(
        &guard,
        "default-workspace",
        cell_id,
        updated_cell_text,
        Some(serde_json::json!({ "title": "Conceito 1 Alterado" })),
    )
    .expect("Falha ao atualizar célula");
    assert!(rev2 > rev1);

    // 4. Verifica Princípio II (Imunidade Retroativa): A Peça NÃO se altera automaticamente
    let loaded_piece = PieceStorage::read_piece(&guard, "default-workspace", "piece-thesis")
        .expect("Falha ao recarregar peça");
    assert!(
        loaded_piece.body.contains(original_text),
        "A Peça deve reter o texto citado original intacto sem mutações retroativas"
    );
    assert!(!loaded_piece.body.contains(updated_cell_text));
    assert_eq!(loaded_piece.citations[0].source_revision, rev1);
    assert_eq!(loaded_piece.citations[0].quote, original_text);
}
