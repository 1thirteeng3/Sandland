use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CitationDrift {
    Synchronized,
    Diverged,
    Orphaned,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CitationRecord {
    pub id: String,
    pub source_cell_id: String,
    pub source_revision: u64,
    pub source_title: String,
    pub source_asset_hash: Option<String>,
    pub quote: String,
    pub quote_hash: String,
    pub inserted_at: i64,
}

impl CitationRecord {
    pub fn new(
        id: String,
        source_cell_id: String,
        source_revision: u64,
        source_title: String,
        source_asset_hash: Option<String>,
        quote: String,
    ) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(quote.as_bytes());
        let quote_hash = format!("{:x}", hasher.finalize());
        let inserted_at = chrono::Utc::now().timestamp();

        Self {
            id,
            source_cell_id,
            source_revision,
            source_title,
            source_asset_hash,
            quote,
            quote_hash,
            inserted_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorialPiece {
    pub id: String,
    pub workspace_id: String,
    pub title: String,
    pub slug: String,
    pub body: String,
    #[serde(default)]
    pub citations: Vec<CitationRecord>,
    #[serde(default)]
    pub word_count: u32,
    pub created_at: i64,
    pub updated_at: i64,
}

impl EditorialPiece {
    pub fn new(id: String, workspace_id: String, title: String) -> Self {
        let now = chrono::Utc::now().timestamp();
        let slug = slugify(&title);
        Self {
            id,
            workspace_id,
            title,
            slug,
            body: String::new(),
            citations: Vec::new(),
            word_count: 0,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn recalculate_metrics(&mut self) {
        self.word_count = self
            .body
            .split_whitespace()
            .filter(|w| !w.is_empty())
            .count() as u32;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    /// Serializa para Markdown com YAML frontmatter canônico
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("---\n");
        out.push_str(&format!("id: \"{}\"\n", self.id));
        out.push_str(&format!("workspace_id: \"{}\"\n", self.workspace_id));
        out.push_str(&format!("title: \"{}\"\n", self.title.replace('"', "\\\"")));
        out.push_str(&format!("slug: \"{}\"\n", self.slug));
        out.push_str(&format!("word_count: {}\n", self.word_count));
        out.push_str(&format!("created_at: {}\n", self.created_at));
        out.push_str(&format!("updated_at: {}\n", self.updated_at));

        if !self.citations.is_empty() {
            out.push_str("citations:\n");
            for cit in &self.citations {
                out.push_str(&format!("  - id: \"{}\"\n", cit.id));
                out.push_str(&format!("    source_cell_id: \"{}\"\n", cit.source_cell_id));
                out.push_str(&format!("    source_revision: {}\n", cit.source_revision));
                out.push_str(&format!(
                    "    source_title: \"{}\"\n",
                    cit.source_title.replace('"', "\\\"")
                ));
                if let Some(ref h) = cit.source_asset_hash {
                    out.push_str(&format!("    source_asset_hash: \"{}\"\n", h));
                } else {
                    out.push_str("    source_asset_hash: null\n");
                }
                out.push_str(&format!(
                    "    quote: \"{}\"\n",
                    cit.quote.replace('\n', "\\n").replace('"', "\\\"")
                ));
                out.push_str(&format!("    quote_hash: \"{}\"\n", cit.quote_hash));
                out.push_str(&format!("    inserted_at: {}\n", cit.inserted_at));
            }
        } else {
            out.push_str("citations: []\n");
        }

        out.push_str("---\n\n");
        out.push_str(&self.body);
        out
    }

    /// Desserializa de Markdown com YAML Frontmatter
    pub fn from_markdown(raw: &str) -> Result<Self, String> {
        let mut id = String::new();
        let mut workspace_id = String::new();
        let mut title = String::new();
        let mut slug = String::new();
        let mut word_count = 0u32;
        let mut created_at = chrono::Utc::now().timestamp();
        let mut updated_at = created_at;
        let mut citations = Vec::new();
        let mut body = raw.to_string();

        if raw.starts_with("---\n") || raw.starts_with("---\r\n") {
            let offset = if raw.starts_with("---\r\n") { 5 } else { 4 };
            if let Some(end_idx) = raw[offset..].find("\n---\n").or_else(|| raw[offset..].find("\r\n---\r\n")) {
                let fm_str = &raw[offset..offset + end_idx];
                let after_fm = &raw[offset + end_idx..];
                let body_offset = if after_fm.starts_with("\r\n---\r\n") { 7 } else { 5 };
                body = after_fm[body_offset..].to_string();

                let mut in_citations = false;
                let mut current_cit: Option<PartialCitation> = None;

                for line in fm_str.lines() {
                    let trimmed = line.trim();
                    if trimmed == "citations:" {
                        in_citations = true;
                        continue;
                    }

                    if in_citations {
                        if trimmed.starts_with("- id:") {
                            if let Some(cit) = current_cit.take() {
                                if let Some(valid) = cit.into_citation() {
                                    citations.push(valid);
                                }
                            }
                            let val = trimmed[5..].trim().trim_matches('"').to_string();
                            current_cit = Some(PartialCitation {
                                id: val,
                                ..Default::default()
                            });
                            continue;
                        } else if trimmed.starts_with("source_cell_id:") {
                            if let Some(ref mut c) = current_cit {
                                c.source_cell_id = trimmed["source_cell_id:".len()..].trim().trim_matches('"').to_string();
                            }
                            continue;
                        } else if trimmed.starts_with("source_revision:") {
                            if let Some(ref mut c) = current_cit {
                                c.source_revision = trimmed["source_revision:".len()..].trim().parse().unwrap_or(1);
                            }
                            continue;
                        } else if trimmed.starts_with("source_title:") {
                            if let Some(ref mut c) = current_cit {
                                c.source_title = trimmed["source_title:".len()..].trim().trim_matches('"').to_string();
                            }
                            continue;
                        } else if trimmed.starts_with("source_asset_hash:") {
                            if let Some(ref mut c) = current_cit {
                                let val = trimmed["source_asset_hash:".len()..].trim().trim_matches('"');
                                c.source_asset_hash = if val == "null" || val.is_empty() {
                                    None
                                } else {
                                    Some(val.to_string())
                                };
                            }
                            continue;
                        } else if trimmed.starts_with("quote:") {
                            if let Some(ref mut c) = current_cit {
                                let val = trimmed["quote:".len()..].trim().trim_matches('"');
                                c.quote = val.replace("\\n", "\n").replace("\\\"", "\"");
                            }
                            continue;
                        } else if trimmed.starts_with("quote_hash:") {
                            if let Some(ref mut c) = current_cit {
                                c.quote_hash = trimmed["quote_hash:".len()..].trim().trim_matches('"').to_string();
                            }
                            continue;
                        } else if trimmed.starts_with("inserted_at:") {
                            if let Some(ref mut c) = current_cit {
                                c.inserted_at = trimmed["inserted_at:".len()..].trim().parse().unwrap_or(0);
                            }
                            continue;
                        } else if !trimmed.is_empty() && !line.starts_with(' ') && !line.starts_with('\t') {
                            in_citations = false;
                            if let Some(cit) = current_cit.take() {
                                if let Some(valid) = cit.into_citation() {
                                    citations.push(valid);
                                }
                            }
                        }
                    }

                    if !in_citations {
                        if let Some((k, v)) = line.split_once(':') {
                            let key = k.trim();
                            let val = v.trim().trim_matches('"');
                            match key {
                                "id" => id = val.to_string(),
                                "workspace_id" => workspace_id = val.to_string(),
                                "title" => title = val.replace("\\\"", "\"").to_string(),
                                "slug" => slug = val.to_string(),
                                "word_count" => word_count = val.parse().unwrap_or(0),
                                "created_at" => created_at = val.parse().unwrap_or(created_at),
                                "updated_at" => updated_at = val.parse().unwrap_or(updated_at),
                                _ => {}
                            }
                        }
                    }
                }

                if let Some(cit) = current_cit.take() {
                    if let Some(valid) = cit.into_citation() {
                        citations.push(valid);
                    }
                }
            }
        }

        if slug.is_empty() && !title.is_empty() {
            slug = slugify(&title);
        }

        let calculated_words = body
            .split_whitespace()
            .filter(|w| !w.is_empty())
            .count() as u32;

        Ok(Self {
            id,
            workspace_id,
            title,
            slug,
            body,
            citations,
            word_count: if word_count > 0 { word_count } else { calculated_words },
            created_at,
            updated_at,
        })
    }
}

#[derive(Default)]
struct PartialCitation {
    id: String,
    source_cell_id: String,
    source_revision: u64,
    source_title: String,
    source_asset_hash: Option<String>,
    quote: String,
    quote_hash: String,
    inserted_at: i64,
}

impl PartialCitation {
    fn into_citation(self) -> Option<CitationRecord> {
        if self.id.is_empty() || self.source_cell_id.is_empty() {
            return None;
        }
        Some(CitationRecord {
            id: self.id,
            source_cell_id: self.source_cell_id,
            source_revision: self.source_revision,
            source_title: self.source_title,
            source_asset_hash: self.source_asset_hash,
            quote: self.quote,
            quote_hash: self.quote_hash,
            inserted_at: self.inserted_at,
        })
    }
}

pub fn slugify(text: &str) -> String {
    let mut slug = String::new();
    let mut last_dash = true;

    for c in text.to_lowercase().chars() {
        if c.is_alphanumeric() {
            slug.push(c);
            last_dash = false;
        } else if (c == ' ' || c == '-' || c == '_') && !last_dash {
            slug.push('-');
            last_dash = true;
        }
    }

    if slug.ends_with('-') {
        slug.pop();
    }

    if slug.is_empty() {
        format!("piece-{}", uuid::Uuid::now_v7().to_string()[..8].to_string())
    } else {
        slug
    }
}
