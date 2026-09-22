use crate::domain::core::errors::SandlandResult;
use std::sync::{Arc, Mutex};

pub const EMBEDDING_DIMENSION: usize = 384;

#[derive(Clone)]
pub struct EmbeddingEngine {
    _inner: Arc<Mutex<Option<()>>>,
}

impl EmbeddingEngine {
    pub fn new() -> Self {
        Self {
            _inner: Arc::new(Mutex::new(None)),
        }
    }

    /// Inicializa o motor de embeddings
    pub fn initialize(&self) -> SandlandResult<()> {
        Ok(())
    }

    /// Verifica se o modelo já está carregado em memória
    pub fn is_loaded(&self) -> bool {
        false
    }

    /// Gera vetor de 384 dimensões para o texto fornecido (stub para v0.1)
    pub fn generate_embedding(&self, _text: &str) -> SandlandResult<Vec<f32>> {
        Ok(vec![0.0; EMBEDDING_DIMENSION])
    }
}

impl Default for EmbeddingEngine {
    fn default() -> Self {
        Self::new()
    }
}
