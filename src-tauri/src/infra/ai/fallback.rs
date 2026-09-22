use crate::infra::ai::gbnf::TaxonomyClassificationOutput;
use crate::infra::ai::llm::LlmEngine;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassificationResultDTO {
    pub item_id: String,
    pub category: String,
    pub tags: Vec<String>,
    pub summary: String,
    pub confidence: f32,
    pub fallback_used: bool,
}

pub struct ClassificationCascade;

impl ClassificationCascade {
    /// Executa a cascata em 3 níveis:
    /// Nível 1: SLM com GBNF (temp 0.2) -> Thermal retry (temp 0.0)
    /// Nível 2: Heurística Léxica (TF-IDF / palavras-chave)
    /// Nível 3: Marcação de revisão manual segura (needs_manual_review: true)
    pub fn classify_with_fallback(
        llm: &LlmEngine,
        item_id: &str,
        content: &str,
        known_tags: &[String],
    ) -> ClassificationResultDTO {
        // Nível 1a: Tentativa padrão (temp 0.2)
        if let Ok(output) = llm.infer_classification(content, known_tags, 0.2) {
            return ClassificationResultDTO {
                item_id: item_id.to_string(),
                category: output.category,
                tags: output.tags,
                summary: output.summary,
                confidence: 0.95,
                fallback_used: false,
            };
        }

        // Nível 1b: Thermal retry ganancioso (temp 0.0)
        if let Ok(output) = llm.infer_classification(content, known_tags, 0.0) {
            return ClassificationResultDTO {
                item_id: item_id.to_string(),
                category: output.category,
                tags: output.tags,
                summary: output.summary,
                confidence: 0.80,
                fallback_used: true,
            };
        }

        // Nível 2: Fallback Léxico
        if let Some(output) = Self::lexical_fallback(content, known_tags) {
            return ClassificationResultDTO {
                item_id: item_id.to_string(),
                category: output.category,
                tags: output.tags,
                summary: output.summary,
                confidence: 0.50,
                fallback_used: true,
            };
        }

        // Nível 3: Marcação segura para revisão manual
        ClassificationResultDTO {
            item_id: item_id.to_string(),
            category: "sem-categoria".to_string(),
            tags: vec!["revisao-manual".to_string()],
            summary: "Documento requer análise e categorização manual.".to_string(),
            confidence: 0.10,
            fallback_used: true,
        }
    }

    fn lexical_fallback(content: &str, known_tags: &[String]) -> Option<TaxonomyClassificationOutput> {
        let words: Vec<&str> = content
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .collect();

        if words.is_empty() {
            return None;
        }

        let mut matched_tags = Vec::new();
        for tag in known_tags {
            if content.to_lowercase().contains(&tag.to_lowercase()) {
                matched_tags.push(tag.clone());
            }
        }

        if matched_tags.is_empty() {
            matched_tags.push(words[0].to_lowercase());
        }

        Some(TaxonomyClassificationOutput {
            summary: format!("Nota contendo {} palavras.", words.len()),
            category: "geral".to_string(),
            tags: matched_tags,
        })
    }
}
