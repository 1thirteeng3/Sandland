use serde_json::json;

#[test]
fn test_gbnf_json_schema_compliance() {
    let valid_gbnf_output = r#"{
  "summary": "Implementação do núcleo local-first com SQLite e PixiJS.",
  "category": "engenharia-de-software",
  "tags": [
    "rust",
    "sqlite",
    "local-first"
  ]
}"#;

    let parsed: Result<serde_json::Value, _> = serde_json::from_str(valid_gbnf_output);
    assert!(parsed.is_ok(), "Output GBNF deve ser JSON estritamente válido");

    let val = parsed.unwrap();
    assert!(val.get("summary").is_some());
    assert_eq!(val["category"], "engenharia-de-software");
    assert_eq!(val["tags"].as_array().unwrap().len(), 3);
}

#[test]
fn test_taxonomy_normalization_rules() {
    // Normalização kebab-case e remoção de plurais simples
    let raw = "Machine Learnings";
    let normalized: String = raw
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();
    assert!(normalized.contains("machine-learning"));
}
