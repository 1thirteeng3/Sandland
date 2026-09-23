use sandland_lib::infra::fs::cas::{read_cas_asset, store_cas_asset, ASSETS_DIR};
use sandland_lib::infra::fs::lifecycle::init_vault;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_asset_ingest_pipeline_and_deduplication() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("cofre_assets");
    init_vault(&root, "Cofre Assets").unwrap();

    let sample_image_data = b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDRSAMPLE_IMAGE_BYTES_123456";
    let ext = "png";

    // 1. Armazena o ativo pela primeira vez
    let asset1 = store_cas_asset(&root, sample_image_data, ext)
        .expect("Falha ao ingerir imagem no CAS");
    assert_eq!(asset1.byte_size, sample_image_data.len() as u64);
    assert_eq!(asset1.extension, "png");
    assert!(asset1.canonical_uri.starts_with("sandland-asset://"));

    // 2. Ingestão repetida do mesmo ativo
    let asset2 = store_cas_asset(&root, sample_image_data, ext)
        .expect("Falha ao re-ingerir imagem no CAS");
    assert_eq!(asset1.sha256_hash, asset2.sha256_hash);

    // Valida que apenas 1 arquivo físico existe no diretório assets/
    let assets_dir = root.join(ASSETS_DIR);
    let files: Vec<_> = fs::read_dir(&assets_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .collect();
    assert_eq!(files.len(), 1, "Deduplicação CAS deve manter exatamente 1 arquivo em disco");

    // 3. Lê o ativo de volta e confirma integridade dos bytes
    let read_bytes = read_cas_asset(&root, &asset1.sha256_hash).expect("Falha ao ler ativo");
    assert_eq!(read_bytes, sample_image_data);
}
