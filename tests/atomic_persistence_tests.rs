use sandland_lib::domain::identity::generate_v7_id;
use sandland_lib::domain::model::{JournalOperation, JournalRecord};
use sandland_lib::infra::fs::journal::{append_journal_record, read_all_journals};
use sandland_lib::infra::fs::lifecycle::init_vault;
use sandland_lib::infra::fs::recovery::run_boot_recovery;
use sandland_lib::infra::fs::writer::{
    commit_document_file, compute_sha256, save_object_snapshot,
};
use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_two_phase_atomic_commit_and_journal_record() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("cofre_persist");
    init_vault(&root, "Cofre Persistente").unwrap();

    let doc_rel_path = Path::new("workspaces/ws1/board.canvas.json");
    let payload_v1 = br#"{"nodes":[],"version":1}"#;

    // 1. Primeiro commit (criação)
    let rev1 = commit_document_file(&root, doc_rel_path, payload_v1, None, None).unwrap();
    assert!(rev1 > 0);

    let written_file = root.join(doc_rel_path);
    assert!(written_file.exists());
    assert_eq!(fs::read(&written_file).unwrap(), payload_v1);

    // Valida se o journal registrou a entrada
    let journals = read_all_journals(&root).unwrap();
    assert_eq!(journals.len(), 1);
    assert_eq!(journals[0].target, "workspaces/ws1/board.canvas.json");
    assert_eq!(journals[0].hash, compute_sha256(payload_v1));
    assert_eq!(journals[0].op, JournalOperation::Create);

    // 2. Segundo commit (atualização com revision tracking)
    let payload_v2 = br#"{"nodes":[{"id":"node1"}],"version":2}"#;
    let rev2 = commit_document_file(&root, doc_rel_path, payload_v2, Some(rev1), None).unwrap();
    assert_eq!(rev2, rev1 + 1);

    assert_eq!(fs::read(&written_file).unwrap(), payload_v2);

    let journals_updated = read_all_journals(&root).unwrap();
    assert_eq!(journals_updated.len(), 2);
    assert_eq!(journals_updated[1].op, JournalOperation::Update);
    assert_eq!(journals_updated[1].prev_rev, Some(rev1));
}

#[test]
fn test_revision_conflict_rejection() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("cofre_conflito");
    init_vault(&root, "Cofre Conflito").unwrap();

    let doc_rel_path = Path::new("notes/nota.md");
    let _rev1 = commit_document_file(&root, doc_rel_path, b"# Nota 1", None, None).unwrap();

    // Tentativa de gravar sob revisão desatualizada (ex.: rev esperada 99 em vez de rev1)
    let result = commit_document_file(&root, doc_rel_path, b"# Conflito", Some(999), None);
    assert!(result.is_err());
}

#[test]
fn test_crash_recovery_purges_orphan_temp_files_and_replays_committed_snapshot() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("cofre_crash");
    init_vault(&root, "Cofre Crash").unwrap();

    // Cria arquivos temporários simulando uma queda abrupta no meio do I/O
    let orphan_tmp = root.join("workspaces").join(".tmp_12345");
    fs::write(&orphan_tmp, b"dados parciais truncados").unwrap();

    // Simula uma transação que fez flush no journal e snapshot mas o processo foi 'killado' antes do rename
    let uncommitted_doc = Path::new("workspaces/ws1/board.canvas.json");
    let expected_payload = br#"{"nodes":[{"id":"restaurado"}],"version":1}"#;
    let snap_hash = compute_sha256(expected_payload);

    save_object_snapshot(&root, expected_payload, &snap_hash).unwrap();

    let record = JournalRecord {
        v: 2,
        id: generate_v7_id(),
        ts: chrono::Utc::now().timestamp_micros(),
        op: JournalOperation::Create,
        target: uncommitted_doc.to_string_lossy().replace('\\', "/"),
        prev_rev: None,
        new_rev: 1,
        hash: snap_hash.clone(),
        snap: Some(snap_hash),
    };
    append_journal_record(&root, &record).unwrap();

    // O arquivo destino ainda não existe antes do boot
    let target_file = root.join(uncommitted_doc);
    assert!(!target_file.exists());

    // Executa a recuperação de boot
    let report = run_boot_recovery(&root).unwrap();
    assert!(report.success);
    assert_eq!(report.purged_temporary_files, 1);
    assert_eq!(report.replayed_operations, 1);

    // Verifica que o arquivo temporário foi purgado
    assert!(!orphan_tmp.exists());

    // Verifica que o arquivo canônico foi recuperado com perfeição
    assert!(target_file.exists());
    assert_eq!(fs::read(&target_file).unwrap(), expected_payload);
}
