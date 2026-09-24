use sandland_lib::domain::workspace::piece::{CitationDrift, CitationRecord, EditorialPiece};
use sandland_lib::infra::fs::canvas_io::CanvasStorage;
use sandland_lib::infra::fs::piece_io::PieceStorage;
use sandland_lib::infra::fs::vault::{initialize_vault_structure, VaultGuard};
use tempfile::tempdir;

#[test]
fn test_piece_citations_drift_detection() {
    let temp = tempdir().expect("Falha ao criar tempdir");
    initialize_vault_structure(temp.path()).expect("Falha ao inicializar cofre");
    let guard = VaultGuard::new(temp.path().to_path_buf()).expect("Falha ao criar VaultGuard");

    // 1. Cria Célula 1 (ficará sincronizada)
    let (_, rev1) = CanvasStorage::save_workspace_cell(
        &guard,
        "default-workspace",
        "cell-sync",
        "Conteúdo fixo que não mudará.",
        Some(serde_json::json!({ "title": "Nota Fixa" })),
    )
    .unwrap();

    // 2. Cria Célula 2 (será alterada depois para gerar Drift)
    let (_, rev2_initial) = CanvasStorage::save_workspace_cell(
        &guard,
        "default-workspace",
        "cell-diverge",
        "Conteúdo inicial da evidência.",
        Some(serde_json::json!({ "title": "Evidência Inicial" })),
    )
    .unwrap();

    // 3. Monta Peça com 3 citações: Sincronizada, Divergente e Órfã (sem célula no disco)
    let mut piece = EditorialPiece::new(
        "piece-audit".to_string(),
        "default-workspace".to_string(),
        "Auditoria de Citações".to_string(),
    );

    let cit_sync = CitationRecord::new(
        "cit-1".to_string(),
        "cell-sync".to_string(),
        rev1,
        "Nota Fixa".to_string(),
        None,
        "Conteúdo fixo que não mudará.".to_string(),
    );

    let cit_diverge = CitationRecord::new(
        "cit-2".to_string(),
        "cell-diverge".to_string(),
        rev2_initial,
        "Evidência Inicial".to_string(),
        None,
        "Conteúdo inicial da evidência.".to_string(),
    );

    let cit_orphan = CitationRecord::new(
        "cit-3".to_string(),
        "cell-inexistente".to_string(),
        1,
        "Célula Excluída".to_string(),
        None,
        "Citação histórica de nó que foi deletado.".to_string(),
    );

    piece.citations.extend(vec![cit_sync, cit_diverge, cit_orphan]);
    PieceStorage::save_piece(&guard, "default-workspace", piece).unwrap();

    // 4. Altera a Célula 2 na Mesa para incrementar sua revisão
    let (_, rev2_updated) = CanvasStorage::save_workspace_cell(
        &guard,
        "default-workspace",
        "cell-diverge",
        "Conteúdo modificado da evidência!",
        Some(serde_json::json!({ "title": "Evidência Atualizada" })),
    )
    .unwrap();
    assert!(rev2_updated > rev2_initial);

    // 5. Executa auditoria determinística de divergência
    let drift_report =
        PieceStorage::check_drift(&guard, "default-workspace", "piece-audit").unwrap();

    assert_eq!(drift_report.len(), 3);

    let item_sync = drift_report.iter().find(|i| i.citation_id == "cit-1").unwrap();
    assert_eq!(item_sync.status, CitationDrift::Synchronized);
    assert_eq!(item_sync.cited_revision, rev1);
    assert_eq!(item_sync.current_revision, Some(rev1));

    let item_diverge = drift_report.iter().find(|i| i.citation_id == "cit-2").unwrap();
    assert_eq!(item_diverge.status, CitationDrift::Diverged);
    assert_eq!(item_diverge.cited_revision, rev2_initial);
    assert_eq!(item_diverge.current_revision, Some(rev2_updated));
    assert_eq!(item_diverge.current_title, Some("Evidência Atualizada".to_string()));

    let item_orphan = drift_report.iter().find(|i| i.citation_id == "cit-3").unwrap();
    assert_eq!(item_orphan.status, CitationDrift::Orphaned);
    assert_eq!(item_orphan.current_revision, None);
}
