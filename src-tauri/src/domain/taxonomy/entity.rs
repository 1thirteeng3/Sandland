use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TermType {
    Category,
    Tag,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxonTerm {
    pub term: String,
    pub term_type: TermType,
    pub frequency: u32,
    pub created_at: i64,
}

impl TaxonTerm {
    pub fn new(term: String, term_type: TermType) -> Self {
        Self {
            term,
            term_type,
            frequency: 1,
            created_at: chrono::Utc::now().timestamp(),
        }
    }
}
