use serde_json::Value;

#[derive(Debug, Clone, Default)]
pub struct ParsedMarkdown {
    pub title: Option<String>,
    pub tags: Vec<String>,
    pub summary: Option<String>,
    pub word_count: usize,
    pub clean_body: String,
    pub frontmatter: Option<serde_json::Map<String, Value>>,
}

/// Analisa documento Markdown extraindo frontmatter YAML, título, tags, contagem de palavras e resumo
pub fn parse_markdown(raw: &str) -> ParsedMarkdown {
    let mut frontmatter_map = None;
    let mut body = raw.trim();

    if raw.starts_with("---\n") || raw.starts_with("---\r\n") {
        let delimiter_len = if raw.starts_with("---\r\n") { 5 } else { 4 };
        if let Some(end_idx) = raw[delimiter_len..].find("\n---\n").or_else(|| raw[delimiter_len..].find("\r\n---\r\n")) {
            let fm_str = &raw[delimiter_len..delimiter_len + end_idx];
            let rest = &raw[delimiter_len + end_idx..];
            let after_fm = rest.find("---\n").map(|i| i + 4).or_else(|| rest.find("---\r\n").map(|i| i + 5)).unwrap_or(0);
            body = rest[after_fm..].trim();

            let mut map = serde_json::Map::new();
            let mut current_key: Option<String> = None;
            let mut current_list: Vec<Value> = Vec::new();

            for line in fm_str.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with('-') && current_key.is_some() {
                    let item = trimmed.trim_start_matches('-').trim().trim_matches('"').trim_matches('\'');
                    current_list.push(Value::String(item.to_string()));
                } else if let Some((k, v)) = line.split_once(':') {
                    if let Some(key) = current_key.take() {
                        if !current_list.is_empty() {
                            map.insert(key, Value::Array(std::mem::take(&mut current_list)));
                        }
                    }
                    let key = k.trim().to_string();
                    let val_str = v.trim();
                    if val_str.is_empty() {
                        current_key = Some(key);
                    } else if val_str.starts_with('[') && val_str.ends_with(']') {
                        let inner = &val_str[1..val_str.len() - 1];
                        let items: Vec<Value> = inner
                            .split(',')
                            .map(|s| Value::String(s.trim().trim_matches('"').trim_matches('\'').to_string()))
                            .filter(|s| !s.as_str().map_or(true, |v| v.is_empty()))
                            .collect();
                        map.insert(key, Value::Array(items));
                    } else {
                        let clean_val = val_str.trim_matches('"').trim_matches('\'');
                        map.insert(key, Value::String(clean_val.to_string()));
                    }
                }
            }

            if let Some(key) = current_key.take() {
                if !current_list.is_empty() {
                    map.insert(key, Value::Array(current_list));
                }
            }

            frontmatter_map = Some(map);
        }
    }

    let mut title = None;
    let mut tags = Vec::new();
    let mut summary = None;

    if let Some(ref fm) = frontmatter_map {
        if let Some(Value::String(t)) = fm.get("title") {
            if !t.trim().is_empty() {
                title = Some(t.clone());
            }
        }
        if let Some(Value::Array(arr)) = fm.get("tags") {
            for item in arr {
                if let Value::String(s) = item {
                    if !s.trim().is_empty() {
                        tags.push(s.clone());
                    }
                }
            }
        }
        if let Some(Value::String(s)) = fm.get("summary") {
            if !s.trim().is_empty() {
                summary = Some(s.clone());
            }
        }
    }

    if title.is_none() {
        for line in body.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# ") {
                title = Some(trimmed.trim_start_matches("# ").trim().to_string());
                break;
            }
        }
    }

    let word_count = body
        .split_whitespace()
        .filter(|w| !w.starts_with('#') && !w.starts_with('-') && !w.starts_with('*'))
        .count();

    if summary.is_none() {
        let first_para = body
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty() && !l.starts_with('#') && !l.starts_with("```") && !l.starts_with('>'))
            .collect::<Vec<_>>()
            .join(" ");

        if !first_para.is_empty() {
            let truncated = if first_para.chars().count() > 240 {
                let s: String = first_para.chars().take(240).collect();
                format!("{}...", s.trim_end())
            } else {
                first_para
            };
            summary = Some(truncated);
        }
    }

    ParsedMarkdown {
        title,
        tags,
        summary,
        word_count,
        clean_body: body.to_string(),
        frontmatter: frontmatter_map,
    }
}
