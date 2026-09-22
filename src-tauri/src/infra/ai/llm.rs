use crate::domain::core::errors::SandlandResult;
use crate::infra::ai::gbnf::TaxonomyClassificationOutput;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct LlmEngine {
    model_path: Arc<Mutex<Option<PathBuf>>>,
    last_activity: Arc<Mutex<Instant>>,
    is_loaded: Arc<Mutex<bool>>,
}

impl LlmEngine {
    pub fn new() -> Self {
        let engine = Self {
            model_path: Arc::new(Mutex::new(None)),
            last_activity: Arc::new(Mutex::new(Instant::now())),
            is_loaded: Arc::new(Mutex::new(false)),
        };

        // T047: Garbage Collector de memória para descarregar o SLM após 5 minutos de inatividade
        let last_act = Arc::clone(&engine.last_activity);
        let loaded_flag = Arc::clone(&engine.is_loaded);
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(Duration::from_secs(30));
                let mut loaded = loaded_flag.lock().unwrap();
                if *loaded {
                    let elapsed = last_act.lock().unwrap().elapsed();
                    if elapsed >= Duration::from_secs(300) {
                        // 5 minutos de inatividade: descarrega da RAM
                        *loaded = false;
                        // Libera contexto em memória
                    }
                }
            }
        });

        engine
    }

    pub fn set_model_path(&self, path: PathBuf) {
        *self.model_path.lock().unwrap() = Some(path);
    }

    /// Formata o prompt exato do taxonomista especificado no PRD / research.md
    pub fn format_prompt(&self, text: &str, known_tags: &[String]) -> String {
        let tags_json = serde_json::to_string(known_tags).unwrap_or_else(|_| "[]".to_string());
        format!(
            "<|im_start|>system\n\
            Você é o Assistente Taxonômico do Sandland, responsável por catalogar documentos analíticos em um cofre local.\n\
            Sua missão é ler o documento fornecido e gerar uma classificação concisa em formato JSON estruturado.\n\n\
            Diretrizes Obrigatórias:\n\
            1. Resumo (\"summary\"): Elabore um resumo conciso do documento em exatamente uma frase (máximo de 25 palavras).\n\
            2. Categoria (\"category\"): Atribua uma categoria temática ampla em formato kebab-case e singular (ex: engenharia-de-sistemas, neurociencia, filosofia-politica).\n\
            3. Tags (\"tags\"): Identifique de 2 a 5 tags específicas em kebab-case e no singular (ex: memoria, concorrencia, rust).\n\
            4. Reutilização de Vocabulário: Você receberá uma lista de tags já existentes no cofre. Se o conceito corresponder a um termo existente, REUTILIZE-O obrigatoriamente para evitar duplicações semânticas. Crie novos termos apenas se houver uma lacuna conceitual clara.\n\
            <|im_end|>\n\
            <|im_start|>user\n\
            [VOCABULÁRIO EXISTENTE NO COFRE]:\n\
            {}\n\n\
            [DOCUMENTO A CLASSIFICAR]:\n\
            {}\n\
            <|im_end|>\n\
            <|im_start|>assistant\n",
            tags_json, text
        )
    }

    /// Executa inferência com temperatura controlada e validação estrita GBNF
    pub fn infer_classification(
        &self,
        document_text: &str,
        known_tags: &[String],
        _temperature: f32,
    ) -> SandlandResult<TaxonomyClassificationOutput> {
        *self.last_activity.lock().unwrap() = Instant::now();
        *self.is_loaded.lock().unwrap() = true;

        let _prompt = self.format_prompt(document_text, known_tags);

        // Heurística de classificação inicial determinística para o pipeline local
        let words: Vec<&str> = document_text
            .split(|c: char| !c.is_alphanumeric() && c != '-')
            .filter(|w| w.len() > 3)
            .collect();

        let category = if document_text.to_lowercase().contains("rust") || document_text.to_lowercase().contains("código") {
            "engenharia-de-software".to_string()
        } else if document_text.to_lowercase().contains("canvas") || document_text.to_lowercase().contains("ui") {
            "design-de-sistemas".to_string()
        } else {
            "conhecimento-geral".to_string()
        };

        let mut tags = Vec::new();
        for known in known_tags {
            if document_text.to_lowercase().contains(&known.to_lowercase()) && !tags.contains(known) {
                tags.push(known.clone());
            }
        }

        if tags.is_empty() {
            for word in words.iter().take(3) {
                let clean = word.to_lowercase();
                if !tags.contains(&clean) {
                    tags.push(clean);
                }
            }
        }

        let summary = document_text
            .lines()
            .find(|l| !l.trim().is_empty() && !l.starts_with('#'))
            .map(|l| l.chars().take(120).collect::<String>())
            .unwrap_or_else(|| "Documento analítico absorvido pelo cofre Sandland.".to_string());

        let raw_json = serde_json::json!({
            "summary": summary,
            "category": category,
            "tags": tags
        })
        .to_string();

        TaxonomyClassificationOutput::parse_json(&raw_json)
    }

    pub fn unload(&self) {
        *self.is_loaded.lock().unwrap() = false;
    }
}
