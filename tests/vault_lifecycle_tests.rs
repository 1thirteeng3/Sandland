use sandland_lib::domain::core::errors::VaultError;
use sandland_lib::infra::fs::lifecycle::{
    init_vault, open_vault, validate_vault, CANONICAL_DIRECTORIES, CURRENT_SCHEMA_VERSION,
    MANIFEST_FILENAME,
};
use std::fs;
use std::time::Instant;
use tempfile::tempdir;

#[test]
fn test_create_vault_canonical_structure_and_manifest() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("novo_cofre");

    let start = Instant::now();
    let manifest = init_vault(&root, "Cofre Primário").expect("Falha ao inicializar cofre");
    let init_duration = start.elapsed();

    // Critério SC-001: Validação e inicialização em disco rápido < 150ms
    assert!(
        init_duration.as_millis() < 150,
        "Tempo de inicialização ({:?}) excedeu a meta de 150ms",
        init_duration
    );

    assert_eq!(manifest.vault_name, "Cofre Primário");
    assert_eq!(manifest.schema_version, CURRENT_SCHEMA_VERSION);
    assert_eq!(manifest.vault_id.get_version_num(), 4);

    // Valida existência de todos os diretórios canônicos
    for canon_dir in CANONICAL_DIRECTORIES {
        let expected_path = root.join(canon_dir);
        assert!(
            expected_path.exists() && expected_path.is_dir(),
            "Diretório canônico ausente: {:?}",
            expected_path
        );
    }

    // Valida conteúdo do vault.json
    let manifest_file = root.join(MANIFEST_FILENAME);
    assert!(manifest_file.exists());
    let bytes = fs::read(&manifest_file).unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(parsed["vault_name"], "Cofre Primário");
    assert_eq!(parsed["schema_version"], "2.0.0");
}

#[test]
fn test_open_arbitrary_folder_safe_refusal_without_mutation() {
    let dir = tempdir().unwrap();
    let user_folder = dir.path().join("documentos_usuario");
    fs::create_dir_all(&user_folder).unwrap();

    let user_file = user_folder.join("meu_artigo.txt");
    fs::write(&user_file, b"Texto confidencial do autor").unwrap();

    // Tenta abrir a pasta arbitrária como se fosse um cofre Sandland
    let result = open_vault(&user_folder);
    assert!(result.is_err(), "Deveria ter recusado abertura de pasta arbitrária");

    match result.unwrap_err() {
        VaultError::ManifestNotFound(p) => {
            assert_eq!(p, user_folder.join(MANIFEST_FILENAME));
        }
        other => panic!("Esperava erro ManifestNotFound, obteve: {:?}", other),
    }

    // Invariante inegociável: nenhum arquivo pode ser criado ou alterado na pasta
    assert!(
        !user_folder.join(MANIFEST_FILENAME).exists(),
        "vault.json NÃO deve ser criado ao abrir pasta arbitrária"
    );
    assert_eq!(
        fs::read(&user_file).unwrap(),
        b"Texto confidencial do autor",
        "Conteúdo de arquivo existente foi alterado indevidamente"
    );
}

#[test]
fn test_open_existing_vault_performance_and_timestamp_update() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("cofre_existente");

    let initial_manifest = init_vault(&root, "Cofre Existente").unwrap();
    let initial_opened_at = initial_manifest.last_opened_at;

    // Aguarda um pequeno delta para diferenciar timestamp
    std::thread::sleep(std::time::Duration::from_millis(50));

    let start = Instant::now();
    let reloaded = open_vault(&root).expect("Falha ao reabrir cofre existente");
    let open_duration = start.elapsed();

    // SC-001: Abertura e validação < 150ms
    assert!(
        open_duration.as_millis() < 150,
        "Tempo de abertura ({:?}) excedeu 150ms",
        open_duration
    );

    assert_eq!(reloaded.vault_id, initial_manifest.vault_id);
    assert_eq!(reloaded.vault_name, "Cofre Existente");
    assert!(
        reloaded.last_opened_at >= initial_opened_at,
        "last_opened_at deveria ter sido atualizado"
    );
}

#[test]
fn test_incompatible_version_rejection() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("cofre_versao_antiga");
    let mut manifest = init_vault(&root, "Cofre Legado").unwrap();

    // Simula esquema de versão futura ou incompatível
    manifest.schema_version = "99.0.0".to_string();
    let manifest_path = root.join(MANIFEST_FILENAME);
    fs::write(&manifest_path, serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();

    let result = validate_vault(&root);
    assert!(result.is_err());
    match result.unwrap_err() {
        VaultError::IncompatibleVersion { found, required } => {
            assert_eq!(found, "99.0.0");
            assert_eq!(required, CURRENT_SCHEMA_VERSION);
        }
        other => panic!("Esperava IncompatibleVersion, obteve: {:?}", other),
    }
}
