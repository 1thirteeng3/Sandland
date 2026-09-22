use crate::domain::core::errors::{SandlandError, SandlandResult};
use serde::{Deserialize, Serialize};

pub const TAXONOMY_GBNF_GRAMMAR: &str = r#"
root   ::= object
object ::= "{\n" "  \"summary\": " string ",\n" "  \"category\": " string ",\n" "  \"tags\": " taglist "\n}"
taglist ::= "[\n" (string (",\n" string)*)? "\n  ]"
string  ::= "\"" ([^"\\] | "\\" (["\\/bfnrt] | "u" [0-9a-fA-F] [0-9a-fA-F] [0-9a-fA-F] [0-9a-fA-F]))* "\""
"#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomyClassificationOutput {
    pub summary: String,
    pub category: String,
    pub tags: Vec<String>,
}

impl TaxonomyClassificationOutput {
    /// Faz o parse seguro do JSON gerado sob a gramática GBNF
    pub fn parse_json(raw_json: &str) -> SandlandResult<Self> {
        let trimmed = raw_json.trim();
        let parsed: TaxonomyClassificationOutput = serde_json::from_str(trimmed).map_err(|e| {
            SandlandError::SerializationError(format!("JSON inválido para taxonomia GBNF: {}", e))
        })?;

        if parsed.category.trim().is_empty() {
            return Err(SandlandError::SerializationError(
                "Categoria retornada vazia pelo modelo".to_string(),
            ));
        }

        Ok(parsed)
    }
}
