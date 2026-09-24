use sandland_lib::domain::workspace::piece::{CitationRecord, EditorialPiece};
use sandland_lib::infra::fs::piece_io::PieceStorage;
use sandland_lib::infra::fs::vault::{initialize_vault_structure, VaultGuard};
use tempfile::tempdir;

#[test]
fn test_editorial_piece_frontmatter_roundtrip() {
    let mut piece = EditorialPiece::new(
        "piece-artigo-1".to_string(),
        "default-workspace".to_string(),
        "Arquitetura e Soberania Local".to_string(),
    );
    piece.body = "# Título da Peça\n\nEste é o corpo do artigo editorial com análise.".to_string();
    piece.recalculate_metrics();

    let cit = CitationRecord::new(
        "cit-1".to_string(),
        "cell-welcome".to_string(),
        1,
        "Bem-vindo".to_string(),
        None,
        "Texto citado do cartão".to_string(),
    );
    piece.citations.push(cit);

    let md = piece.to_markdown();
    assert!(md.starts_with("---\n"));
    assert!(md.contains("id: \"piece-artigo-1\""));
    assert!(md.contains("slug: \"arquitetura-e-soberania-local\""));
    assert!(md.contains("citations:"));
    assert!(md.contains("source_cell_id: \"cell-welcome\""));
    assert!(md.contains("source_revision: 1"));
    assert!(md.contains("quote: \"Texto citado do cartão\""));
    assert!(md.contains("# Título da Peça"));

    let parsed = EditorialPiece::from_markdown(&md).expect("Falha ao parsear markdown da Peça");
    assert_eq!(parsed.id, "piece-artigo-1");
    assert_eq!(parsed.workspace_id, "default-workspace");
    assert_eq!(parsed.title, "Arquitetura e Soberania Local");
    assert_eq!(parsed.slug, "arquitetura-e-soberania-local");
    assert_eq!(parsed.citations.len(), 1);
    assert_eq!(parsed.citations[0].id, "cit-1");
    assert_eq!(parsed.citations[0].source_cell_id, "cell-welcome");
    assert_eq!(parsed.citations[0].source_revision, 1);
    assert_eq!(parsed.citations[0].quote, "Texto citado do cartão");
    assert!(parsed.body.contains("Este é o corpo do artigo editorial"));
}

#[test]
fn test_piece_storage_lifecycle_and_export() {
    let temp = tempdir().expect("Falha ao criar tempdir");
    initialize_vault_structure(temp.path()).expect("Falha ao inicializar cofre");
    let guard = VaultGuard::new(temp.path().to_path_buf()).expect("Falha ao criar VaultGuard");

    let mut piece = EditorialPiece::new(
        "piece-report-1".to_string(),
        "default-workspace".to_string(),
        "Relatório Analítico de Síntese".to_string(),
    );
    piece.body = "Primeiro parágrafo do relatório com dados compilados.".to_string();

    let cit = CitationRecord::new(
        "cit-report-1".to_string(),
        "cell-data".to_string(),
        2,
        "Dados Analíticos".to_string(),
        None,
        "Números extraídos do experimento local.".to_string(),
    );
    piece.citations.push(cit);

    // 1. Salvar Peça no disco
    let (rel_path, words, _) =
        PieceStorage::save_piece(&guard, "default-workspace", piece.clone()).expect("Falha ao salvar peça");
    assert_eq!(rel_path, "workspaces/default-workspace/pieces/piece-report-1.md");
    assert!(words > 0);

    // 2. Listar peças do workspace
    let pieces = PieceStorage::list_pieces(&guard, "default-workspace").expect("Falha ao listar peças");
    assert_eq!(pieces.len(), 1);
    assert_eq!(pieces[0].id, "piece-report-1");
    assert_eq!(pieces[0].citation_count, 1);

    // 3. Ler peça do disco
    let loaded = PieceStorage::read_piece(&guard, "default-workspace", "piece-report-1")
        .expect("Falha ao ler peça");
    assert_eq!(loaded.title, "Relatório Analítico de Síntese");
    assert_eq!(loaded.citations.len(), 1);

    // 4. Compilar exportação com referências
    let export_text = PieceStorage::compile_export(&guard, "default-workspace", "piece-report-1", true)
        .expect("Falha ao compilar exportação");
    assert!(export_text.contains("## Referências & Proveniência"));
    assert!(export_text.contains("Dados Analíticos"));
    assert!(export_text.contains("Revisão 2"));
}
