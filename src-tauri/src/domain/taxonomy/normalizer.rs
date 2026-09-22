pub fn normalize_to_kebab(raw: &str) -> String {
    let mut result = String::new();
    let mut prev_dash = false;

    for c in raw.trim().chars() {
        let lower = c.to_ascii_lowercase();
        if lower.is_ascii_alphanumeric() {
            result.push(lower);
            prev_dash = false;
        } else if (c == ' ' || c == '_' || c == '-' || c == '/') && !prev_dash {
            result.push('-');
            prev_dash = true;
        }
    }

    let trimmed = result.trim_matches('-');
    singularize(trimmed)
}

pub fn singularize(term: &str) -> String {
    if term.len() > 4 && term.ends_with('s') && !term.ends_with("ss") && !term.ends_with("is") {
        term[..term.len() - 1].to_string()
    } else {
        term.to_string()
    }
}

pub fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let (m, n) = (a_chars.len(), b_chars.len());

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }

    dp[m][n]
}

/// Normaliza um termo e reutiliza termos existentes se distância de Levenshtein <= 2
pub fn normalize_term(raw: &str, known_terms: &[String]) -> String {
    let candidate = normalize_to_kebab(raw);
    if candidate.is_empty() {
        return candidate;
    }

    for known in known_terms {
        if levenshtein(&candidate, known) <= 2 && candidate.len() >= 4 && known.len() >= 4 {
            return known.clone();
        }
    }

    candidate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kebab_and_singular() {
        assert_eq!(normalize_to_kebab("Machine Learnings"), "machine-learning");
        assert_eq!(normalize_to_kebab("Áreas de Análise"), "reas-de-anlise");
    }

    #[test]
    fn test_levenshtein_reuse() {
        let known = vec!["architecture".to_string(), "local-first".to_string()];
        assert_eq!(normalize_term("architectur", &known), "architecture");
    }
}
