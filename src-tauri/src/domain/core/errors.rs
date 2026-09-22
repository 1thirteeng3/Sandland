use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "code", content = "details")]
pub enum SandlandError {
    #[error("Cofre não encontrado no caminho: {0}")]
    VaultNotFound(PathBuf),

    #[error("Já existe um cofre inicializado no caminho: {0}")]
    VaultAlreadyExists(PathBuf),

    #[error("Tentativa de fuga do Sandbox detectada: {attempted} fora de {boundary}")]
    SandboxEscape { attempted: PathBuf, boundary: PathBuf },

    #[error("Falha ao ingerir o documento '{item_id}': {reason}")]
    IngestFailed { item_id: String, reason: String },

    #[error("Falha na classificação taxonômica após {attempts} tentativas")]
    ClassificationFailed { item_id: String, attempts: u8 },

    #[error("Modelo local '{name}' ausente ou não inicializado. Execute o download através da interface.")]
    ModelNotLoaded { name: String },

    #[error("Integridade corrompida no modelo '{name}'. Hash esperado: {expected_sha}")]
    ModelCorrupted { name: String, expected_sha: String },

    #[error("Erro de banco de dados SQLite: {0}")]
    DatabaseError(String),

    #[error("Erro de serialização ou parsing: {0}")]
    SerializationError(String),

    #[error("Erro de I/O no disco local: {0}")]
    IoError(String),
}

impl From<std::io::Error> for SandlandError {
    fn from(err: std::io::Error) -> Self {
        SandlandError::IoError(err.to_string())
    }
}

impl From<rusqlite::Error> for SandlandError {
    fn from(err: rusqlite::Error) -> Self {
        SandlandError::DatabaseError(err.to_string())
    }
}

impl From<serde_json::Error> for SandlandError {
    fn from(err: serde_json::Error) -> Self {
        SandlandError::SerializationError(err.to_string())
    }
}

pub type SandlandResult<T> = Result<T, SandlandError>;
