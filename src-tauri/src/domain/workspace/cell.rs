use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceCell {
    pub id: String,
    pub workspace_id: String,
    pub node_id: Option<String>,
    pub item_id: Option<String>,
    pub source_path: Option<String>,
    pub source_hash: Option<String>,
    #[serde(default = "default_revision")]
    pub revision: u64,
    pub created_at: i64,
    pub updated_at: i64,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub position: Option<Position>,
    #[serde(default)]
    pub tags: Vec<String>,
}

fn default_revision() -> u64 {
    1
}

pub type Cell = WorkspaceCell;

impl WorkspaceCell {
    pub fn new(id: String, workspace_id: String) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id,
            workspace_id,
            node_id: None,
            item_id: None,
            source_path: None,
            source_hash: None,
            revision: 1,
            created_at: now,
            updated_at: now,
            title: String::new(),
            content: String::new(),
            position: None,
            tags: Vec::new(),
        }
    }

    /// Serializa a célula para Markdown com YAML frontmatter canônico
    pub fn to_markdown(&self) -> String {
        let mut fm = String::new();
        fm.push_str("---\n");
        fm.push_str(&format!("id: \"{}\"\n", self.id));
        fm.push_str(&format!("workspace_id: \"{}\"\n", self.workspace_id));
        if let Some(ref nid) = self.node_id {
            fm.push_str(&format!("node_id: \"{}\"\n", nid));
        }
        if let Some(ref iid) = self.item_id {
            fm.push_str(&format!("item_id: \"{}\"\n", iid));
        }
        if let Some(ref sp) = self.source_path {
            fm.push_str(&format!("source_path: \"{}\"\n", sp));
        }
        if let Some(ref sh) = self.source_hash {
            fm.push_str(&format!("source_hash: \"{}\"\n", sh));
        }
        if !self.tags.is_empty() {
            fm.push_str("tags:\n");
            for tag in &self.tags {
                fm.push_str(&format!("  - \"{}\"\n", tag));
            }
        }
        fm.push_str(&format!("revision: {}\n", self.revision));
        fm.push_str(&format!("created_at: {}\n", self.created_at));
        fm.push_str(&format!("updated_at: {}\n", self.updated_at));
        fm.push_str("---\n\n");

        if !self.title.is_empty() && !self.content.starts_with("# ") {
            fm.push_str(&format!("# {}\n\n", self.title));
        }
        fm.push_str(&self.content);
        fm
    }

    /// Desserializa um arquivo markdown com YAML frontmatter para WorkspaceCell
    pub fn from_markdown(raw: &str) -> Result<Self, String> {
        let mut id = String::new();
        let mut workspace_id = String::new();
        let mut node_id = None;
        let mut item_id = None;
        let mut source_path = None;
        let mut source_hash = None;
        let mut revision = 1u64;
        let mut created_at = chrono::Utc::now().timestamp();
        let mut updated_at = created_at;
        let mut tags = Vec::new();
        let mut body = raw.to_string();

        if raw.starts_with("---\n") || raw.starts_with("---\r\n") {
            let offset = if raw.starts_with("---\r\n") { 5 } else { 4 };
            if let Some(end_idx) = raw[offset..].find("\n---\n").or_else(|| raw[offset..].find("\r\n---\r\n")) {
                let fm_str = &raw[offset..offset + end_idx];
                let after_fm = &raw[offset + end_idx..];
                let body_offset = if after_fm.starts_with("\r\n---\r\n") { 7 } else { 5 };
                body = after_fm[body_offset..].to_string();

                let mut in_tags = false;
                for line in fm_str.lines() {
                    let trimmed = line.trim();
                    if trimmed == "tags:" {
                        in_tags = true;
                        continue;
                    }
                    if in_tags {
                        if trimmed.starts_with("- ") {
                            let tag_val = trimmed[2..].trim().trim_matches('"').trim_matches('\'');
                            tags.push(tag_val.to_string());
                            continue;
                        } else if !trimmed.is_empty() && !trimmed.starts_with('#') {
                            in_tags = false;
                        }
                    }

                    if let Some((k, v)) = line.split_once(':') {
                        let key = k.trim();
                        let val = v.trim().trim_matches('"').trim_matches('\'');
                        match key {
                            "id" => id = val.to_string(),
                            "workspace_id" => workspace_id = val.to_string(),
                            "node_id" => node_id = Some(val.to_string()),
                            "item_id" => item_id = Some(val.to_string()),
                            "source_path" => source_path = Some(val.to_string()),
                            "source_hash" => source_hash = Some(val.to_string()),
                            "revision" => {
                                if let Ok(rev) = val.parse::<u64>() {
                                    revision = rev;
                                }
                            }
                            "created_at" => {
                                if let Ok(t) = val.parse::<i64>() {
                                    created_at = t;
                                }
                            }
                            "updated_at" => {
                                if let Ok(t) = val.parse::<i64>() {
                                    updated_at = t;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        let trimmed_body = body.trim();
        let (title, content) = if trimmed_body.starts_with("# ") {
            if let Some(first_nl) = trimmed_body.find('\n') {
                let title_line = trimmed_body[2..first_nl].trim().to_string();
                let rest = trimmed_body[first_nl..].trim().to_string();
                (title_line, rest)
            } else {
                (trimmed_body[2..].trim().to_string(), String::new())
            }
        } else {
            (String::new(), trimmed_body.to_string())
        };

        Ok(Self {
            id,
            workspace_id,
            node_id,
            item_id,
            source_path,
            source_hash,
            revision,
            created_at,
            updated_at,
            title,
            content,
            position: None,
            tags,
        })
    }
}
