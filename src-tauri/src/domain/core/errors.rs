use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", content = "details")]
pub enum SecurityError {
    #[error("Tentativa de fuga do Sandbox detectada: {0}")]
    SandboxEscapeAttempt(String),

    #[error("Tentativa de transversão de caminho detectada: {0}")]
    PathTraversal(String),

    #[error("Caminho inválido fornecido: {0}")]
    InvalidPath(String),

    #[error("Symlink ou junction aponta para fora dos limites do cofre: {0}")]
    SymlinkEscaped(PathBuf),
}

#[derive(Error, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", content = "details")]
pub enum JournalError {
    #[error("Erro de I/O no journal: {0}")]
    Io(String),

    #[error("Entrada corrompida no journal: {0}")]
    CorruptedEntry(String),

    #[error("Conflito de revisão: esperada {expected:?}, mas encontrada {actual}")]
    RevisionConflict {
        expected: Option<u64>,
        actual: u64,
    },

    #[error("Erro de serialização no journal: {0}")]
    Serialization(String),
}

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "code", content = "details")]
pub enum VaultError {
    #[error("Diretório raiz inválido ou inexistente: {0}")]
    InvalidRootDirectory(PathBuf),

    #[error("Manifesto do cofre (vault.json) não encontrado em: {0}")]
    ManifestNotFound(PathBuf),

    #[error("Manifesto do cofre corrompido: {0}")]
    ManifestCorrupted(String),

    #[error("Versão incompatível do esquema: encontrada '{found}', requerida '{required}'")]
    IncompatibleVersion {
        found: String,
        required: String,
    },

    #[error("Erro de segurança e confinamento: {0}")]
    Security(#[from] SecurityError),

    #[error("Erro no protocolo de journal: {0}")]
    Journal(#[from] JournalError),

    #[error("Ativo CAS não encontrado para o hash: {0}")]
    AssetNotFound(String),

    #[error("Integridade de ativo corrompida. Hash calculado '{calculated}', esperado '{expected}'")]
    AssetCorrupted {
        calculated: String,
        expected: String,
    },

    #[error("Erro de I/O no disco: {0}")]
    Io(String),

    #[error("Erro de serialização ou formatação de dados: {0}")]
    Serialization(String),

    #[error("Operação recusada: recurso ocupado ou bloqueado: {0}")]
    Busy(String),
}

impl From<std::io::Error> for VaultError {
    fn from(err: std::io::Error) -> Self {
        VaultError::Io(err.to_string())
    }
}

impl From<serde_json::Error> for VaultError {
    fn from(err: serde_json::Error) -> Self {
        VaultError::Serialization(err.to_string())
    }
}

pub type VaultResult<T> = Result<T, VaultError>;

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "code", content = "details")]
pub enum SandlandError {
    #[error("Cofre não encontrado no caminho: {0}")]
    VaultNotFound(PathBuf),

    #[error("Já existe um cofre inicializado no caminho: {0}")]
    VaultAlreadyExists(PathBuf),

    #[error("Tentativa de fuga do Sandbox detectada: {attempted} fora de {boundary}")]
    SandboxEscape { attempted: PathBuf, boundary: PathBuf },

    #[error("Erro no Vault: {0}")]
    Vault(#[from] VaultError),

    #[error("Erro de segurança: {0}")]
    Security(#[from] SecurityError),

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
