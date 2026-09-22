use sandland_lib::domain::core::errors::VaultError;

#[test]
fn test_sanity() {
    let err = VaultError::Serialization("teste".to_string());
    assert_eq!(err.to_string(), "Erro de serialização ou formatação de dados: teste");
}
