use sandland_lib::infra::fs::cas::{read_cas_asset, store_cas_asset, ASSETS_DIR};
use sandland_lib::infra::fs::lifecycle::init_vault;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_cas_storage_and_sha256_deduplication() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("cofre_cas");
    init_vault(&root, "Cofre CAS").unwrap();

    let image_bytes = b"EXEMPLO_DE_IMAGEM_PNG_BYTES_1234567890_BINARIO";
    let ext = "png";

    // 1. Armazena o ativo pela primeira vez
    let asset_ref1 = store_cas_asset(&root, image_bytes, ext).expect("Falha ao salvar ativo CAS");
    assert_eq!(asset_ref1.byte_size, image_bytes.len() as u64);
    assert_eq!(asset_ref1.extension, "png");
    assert!(asset_ref1.canonical_uri.starts_with("sandland-asset://"));

    // 2. Armazena os exatos mesmos bytes novamente (simulando inclusão em outra nota)
    let asset_ref2 = store_cas_asset(&root, image_bytes, "png").expect("Falha ao re-armazenar");
    assert_eq!(asset_ref1.sha256_hash, asset_ref2.sha256_hash);

    // Critério SC-005: 100% de taxa de deduplicação física no disco
    let assets_dir = root.join(ASSETS_DIR);
    let files: Vec<_> = fs::read_dir(&assets_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .collect();

    assert_eq!(
        files.len(),
        1,
        "Deve existir exatamente 1 arquivo físico no diretório assets/ após deduplicação"
    );

    // 3. Lê o ativo e valida integridade dos bytes
    let read_back = read_cas_asset(&root, &asset_ref1.sha256_hash).expect("Falha ao ler ativo CAS");
    assert_eq!(read_back, image_bytes);
}

#[test]
fn test_cas_read_non_existent_hash() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("cofre_cas_missing");
    init_vault(&root, "Cofre CAS Vazio").unwrap();

    let fake_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    let res = read_cas_asset(&root, fake_hash);
    assert!(res.is_err());
}
