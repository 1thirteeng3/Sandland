use sandland_lib::infra::db::indexer::IngestFilterDTO;
use sandland_lib::infra::db::ingest_repo::IngestRepository;
use sandland_lib::infra::db::init_database;
use sandland_lib::infra::fs::ingest_storage::IngestStorage;
use sandland_lib::infra::fs::lifecycle::init_vault;
use sandland_lib::infra::fs::vault::{hydrate_vault_from_disk, VaultGuard};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_physical_note_ingest_and_sqlite_lifecycle() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("cofre_ingestao");
    init_vault(&root, "Cofre Ingestão").unwrap();

    let guard = VaultGuard::new(&root).expect("Falha ao inicializar VaultGuard");
    let mut conn = init_database(&root.join("index.db")).expect("Falha ao inicializar banco de dados");

    // 1. Salva nota física diretamente no cofre via IngestStorage
    let note_title = "Minha Primeira Nota de Pesquisa";
    let raw_content = "# Minha Primeira Nota de Pesquisa\n\nEste é o corpo da nota física com conceitos avançados de IA.";
    let (item, canonical_md) = IngestStorage::save_note_content(&guard, note_title, raw_content)
        .expect("Falha ao gravar nota física");

    // Valida persistência física
    let full_path = guard.resolve_relative_path(std::path::Path::new(&item.canonical_uri)).expect("Caminho canônico inválido");
    assert!(full_path.exists(), "O arquivo físico canônico deve existir no disco");
    let saved_disk_content = fs::read_to_string(&full_path).unwrap();
    assert!(saved_disk_content.contains("Minha Primeira Nota de Pesquisa"));
    assert!(saved_disk_content.contains("---")); // Frontmatter YAML

    // 2. Indexa no SQLite index.db
    IngestRepository::upsert(&mut conn, &item, &canonical_md)
        .expect("Falha ao indexar item no SQLite");

    // 3. Consulta por ID
    let fetched = IngestRepository::get_by_id(&conn, &item.id)
        .expect("Falha ao buscar item")
        .expect("Item deveria existir");
    assert_eq!(fetched.title, note_title);
    assert_eq!(fetched.canonical_uri, item.canonical_uri);
    assert!(fetched.word_count > 0);

    // 4. Busca FTS5 no SQLite
    let fts_filter = IngestFilterDTO {
        query: Some("conceitos avançados".to_string()),
        category: None,
        tag: None,
        needs_manual_review_only: None,
        limit: Some(10),
        offset: Some(0),
    };
    let fts_results = IngestRepository::list(&conn, &fts_filter).expect("Falha na busca FTS5");
    assert_eq!(fts_results.len(), 1);
    assert_eq!(fts_results[0].id, item.id);

    // 5. Teste de Hidratação do Cofre no Cold Start (hydrate_vault_from_disk)
    let hydrated_count = hydrate_vault_from_disk(&guard, &mut conn).expect("Falha na hidratação");
    // Como a nota já está indexada com o mesmo hash, não deve reindexar redundantemente
    assert_eq!(hydrated_count, 0);

    // Adiciona uma nova nota externa diretamente no disco para testar cold start
    let external_note_path = root.join("ingest").join("notes").join("nota_externa.md");
    fs::write(&external_note_path, "# Nota Criada Externamente\n\nTexto externo de teste.").unwrap();

    let new_hydrated = hydrate_vault_from_disk(&guard, &mut conn).expect("Falha ao hidratar nova nota");
    assert_eq!(new_hydrated, 1, "Deve hidratar a nota recém-adicionada externamente");
}
