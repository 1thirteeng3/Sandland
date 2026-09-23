use crate::domain::core::errors::SandlandResult;
use crate::domain::ingest::item::{IngestStatus, IngestedItem, SourceType};
use crate::domain::ingest::parser::parse_markdown;
use crate::infra::fs::vault::format_markdown_with_frontmatter;
use crate::infra::security::VaultGuard;
use sha2::{Digest, Sha256};
use std::path::Path;
use uuid::Uuid;

pub struct IngestStorage;

impl IngestStorage {
    /// Salva fisicamente uma nota de texto no diretório canônico `ingest/notes/`
    pub fn save_note_content(
        vault_guard: &VaultGuard,
        raw_title: &str,
        raw_content: &str,
    ) -> SandlandResult<(IngestedItem, String)> {
        let parsed = parse_markdown(raw_content);
        let id = Uuid::now_v7().to_string();

        let title = parsed
            .title
            .unwrap_or_else(|| {
                if !raw_title.trim().is_empty() {
                    raw_title.to_string()
                } else {
                    "Nota Sem Título".to_string()
                }
            });

        let sanitized_stem: String = title
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '-' || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();

        let file_name = format!("{}-{}.md", sanitized_stem, &id[..8]);
        let rel_path = format!("ingest/notes/{}", file_name);

        let markdown = format_markdown_with_frontmatter(
            &id,
            &title,
            "note",
            None,
            &parsed.tags,
            parsed.summary.as_deref(),
            &parsed.clean_body,
        );

        let mut hasher = Sha256::new();
        hasher.update(markdown.as_bytes());
        let content_hash = format!("{:x}", hasher.finalize());

        // Grava no disco através do VaultGuard com proteção anti-TOCTOU
        vault_guard.secure_write(Path::new(&rel_path), markdown.as_bytes())?;

        let now = chrono::Utc::now().timestamp();
        let item = IngestedItem {
            id,
            source_type: SourceType::File,
            source_path: rel_path.clone(),
            canonical_uri: rel_path,
            title,
            summary: parsed.summary,
            content_hash,
            word_count: parsed.word_count,
            tags: parsed.tags,
            status: IngestStatus::Ingested,
            ingested_at: now,
            updated_at: now,
        };

        Ok((item, markdown))
    }

    /// Salva fisicamente um snapshot textual de URL no diretório `ingest/web/`
    pub fn save_web_snapshot(
        vault_guard: &VaultGuard,
        url: &str,
        title: &str,
        markdown_body: &str,
    ) -> SandlandResult<(IngestedItem, String)> {
        let id = Uuid::now_v7().to_string();
        let file_name = format!("web-{}.md", &id[..8]);
        let rel_path = format!("ingest/web/{}", file_name);

        let tags = vec!["web-snapshot".to_string()];
        let markdown = format_markdown_with_frontmatter(
            &id,
            title,
            "web_snapshot",
            None,
            &tags,
            Some(&format!("Captura de URL: {}", url)),
            markdown_body,
        );

        let mut hasher = Sha256::new();
        hasher.update(markdown.as_bytes());
        let content_hash = format!("{:x}", hasher.finalize());

        vault_guard.secure_write(Path::new(&rel_path), markdown.as_bytes())?;

        let now = chrono::Utc::now().timestamp();
        let word_count = markdown_body.split_whitespace().count();

        let item = IngestedItem {
            id,
            source_type: SourceType::Url,
            source_path: url.to_string(),
            canonical_uri: rel_path,
            title: title.to_string(),
            summary: Some(format!("Snapshot da página {}", url)),
            content_hash,
            word_count,
            tags,
            status: IngestStatus::Ingested,
            ingested_at: now,
            updated_at: now,
        };

        Ok((item, markdown))
    }
}
