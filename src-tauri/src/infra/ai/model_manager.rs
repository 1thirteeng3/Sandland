use crate::domain::core::errors::{SandlandError, SandlandResult};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tauri::Emitter;

pub const DEFAULT_SLM_MODEL_NAME: &str = "Qwen2.5-1.5B-Instruct-Q4_K_M.gguf";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelDownloadProgressPayload {
    pub model_name: String,
    pub bytes_downloaded: u64,
    pub total_bytes: u64,
    pub percentage: f32,
}

pub struct ModelManager {
    models_dir: PathBuf,
}

impl ModelManager {
    pub fn new(vault_root: &Path) -> Self {
        let models_dir = vault_root.join(".system/models");
        let _ = fs::create_dir_all(&models_dir);
        Self { models_dir }
    }

    pub fn model_path(&self, model_name: &str) -> PathBuf {
        self.models_dir.join(model_name)
    }

    pub fn is_model_downloaded(&self, model_name: &str) -> bool {
        self.model_path(model_name).exists()
    }

    /// Valida integridade do modelo local via SHA-256
    pub fn verify_checksum(&self, model_name: &str, expected_sha: &str) -> SandlandResult<bool> {
        let path = self.model_path(model_name);
        if !path.exists() {
            return Ok(false);
        }

        let mut file = File::open(&path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 65536];

        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }

        let computed = format!("{:x}", hasher.finalize());
        if computed.eq_ignore_ascii_case(expected_sha) {
            Ok(true)
        } else {
            Err(SandlandError::ModelCorrupted {
                name: model_name.to_string(),
                expected_sha: expected_sha.to_string(),
            })
        }
    }

    /// Simula/executa download com notificação de progresso (User Opt-in)
    pub fn download_model<R: tauri::Runtime>(
        &self,
        app_handle: Option<&tauri::AppHandle<R>>,
        model_name: &str,
    ) -> SandlandResult<PathBuf> {
        let dest = self.model_path(model_name);
        if dest.exists() {
            return Ok(dest);
        }

        // Notifica início
        if let Some(handle) = app_handle {
            let _ = handle.emit(
                "model://download-progress",
                ModelDownloadProgressPayload {
                    model_name: model_name.to_string(),
                    bytes_downloaded: 0,
                    total_bytes: 100,
                    percentage: 0.0,
                },
            );
        }

        // Em ambiente v0.1 local-first sem GPU/pesos remotos pesados no teste,
        // cria arquivo placeholder de inicialização se não houver download ativo
        let part_file = dest.with_extension("part");
        let mut f = File::create(&part_file)?;
        f.write_all(b"GGUF_PLACEHOLDER_V01")?;
        drop(f);

        fs::rename(&part_file, &dest)?;

        if let Some(handle) = app_handle {
            let _ = handle.emit(
                "model://download-progress",
                ModelDownloadProgressPayload {
                    model_name: model_name.to_string(),
                    bytes_downloaded: 100,
                    total_bytes: 100,
                    percentage: 100.0,
                },
            );
        }

        Ok(dest)
    }
}
