use crate::domain::core::errors::{SandlandError, SandlandResult};
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use std::sync::{Arc, Mutex};

pub const EMBEDDING_DIMENSION: usize = 384;

#[derive(Clone)]
pub struct EmbeddingEngine {
    inner: Arc<Mutex<Option<TextEmbedding>>>,
}

impl EmbeddingEngine {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(None)),
        }
    }

    /// Inicializa o modelo multilingual MiniLM-L12-v2 sob demanda (User Opt-in)
    pub fn initialize(&self) -> SandlandResult<()> {
        let mut guard = self.inner.lock().unwrap();
        if guard.is_some() {
            return Ok(());
        }

        let model = TextEmbedding::try_new(InitOptions {
            model_name: EmbeddingModel::ParaphraseMLMiniLML12V2,
            show_download_progress: false,
            ..Default::default()
        })
        .map_err(|e| {
            SandlandError::ModelNotLoaded {
                name: format!("fastembed paraphrase-multilingual-MiniLM-L12-v2: {}", e),
            }
        })?;

        *guard = Some(model);
        Ok(())
    }

    /// Verifica se o modelo já está carregado em memória
    pub fn is_loaded(&self) -> bool {
        self.inner.lock().unwrap().is_some()
    }

    /// Gera vetor de 384 dimensões para o texto fornecido
    pub fn generate_embedding(&self, text: &str) -> SandlandResult<Vec<f32>> {
        let mut guard = self.inner.lock().unwrap();
        if guard.is_none() {
            drop(guard);
            self.initialize()?;
            guard = self.inner.lock().unwrap();
        }

        let model = guard.as_mut().ok_or_else(|| SandlandError::ModelNotLoaded {
            name: "paraphrase-multilingual-MiniLM-L12-v2".to_string(),
        })?;

        let documents = vec![text];
        let embeddings = model
            .embed(documents, None)
            .map_err(|_e| SandlandError::ClassificationFailed {
                item_id: "embedding_generation".to_string(),
                attempts: 1,
            })?;

        embeddings
            .into_iter()
            .next()
            .ok_or_else(|| SandlandError::SerializationError("Vetor embedding vazio".to_string()))
    }
}
