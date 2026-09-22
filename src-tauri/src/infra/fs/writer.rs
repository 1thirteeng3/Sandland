use chrono::Utc;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use crate::domain::core::errors::{JournalError, VaultError, VaultResult};
use crate::domain::core::events::{DomainEvent, EventBus};
use crate::domain::identity::generate_v7_id;
use crate::domain::model::{JournalOperation, JournalRecord};
use crate::infra::fs::journal::{append_journal_record, get_latest_file_revision};
use crate::infra::fs::{atomic_rename, write_file_atomic};

pub const OBJECTS_DIR: &str = ".history/objects";

/// Calcula o hash SHA-256 de um slice de bytes e retorna em hexadecimal
pub fn compute_sha256(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

/// Salva um snapshot imutável em `.history/objects/<sha256>`
pub fn save_object_snapshot(root_path: &Path, content: &[u8], hash: &str) -> VaultResult<()> {
    let objects_dir = root_path.join(OBJECTS_DIR);
    if !objects_dir.exists() {
        fs::create_dir_all(&objects_dir).map_err(|e| {
            VaultError::Io(format!("Falha ao criar diretório de objetos históricos: {e}"))
        })?;
    }

    let snap_path = objects_dir.join(hash);
    if !snap_path.exists() {
        write_file_atomic(&snap_path, content)?;
    }
    Ok(())
}

/// Executa a máquina de estados formal `file-commit` em duas fases
pub fn commit_document_file(
    root_path: &Path,
    rel_path: &Path,
    payload: &[u8],
    expected_revision: Option<u64>,
    event_bus: Option<&EventBus>,
) -> VaultResult<u64> {
    let target_abs_path = root_path.join(rel_path);
    let parent_dir = target_abs_path.parent().ok_or_else(|| {
        VaultError::Io("Caminho de destino não possui diretório pai".to_string())
    })?;

    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir).map_err(|e| {
            VaultError::Io(format!("Falha ao criar diretório pai: {e}"))
        })?;
    }

    let file_already_exists = target_abs_path.exists();
    let current_journal_rev = get_latest_file_revision(root_path, rel_path)
        .map_err(VaultError::Journal)?;

    // Validação de conflito de revisão (OCC - Optimistic Concurrency Control)
    let (prev_revision, next_revision_number) = match (expected_revision, current_journal_rev) {
        (Some(exp), Some(actual)) => {
            if exp != actual {
                return Err(VaultError::Journal(JournalError::RevisionConflict {
                    expected: Some(exp),
                    actual,
                }));
            }
            (Some(actual), exp + 1)
        }
        (Some(exp), None) => {
            if exp != 0 && file_already_exists {
                return Err(VaultError::Journal(JournalError::RevisionConflict {
                    expected: Some(exp),
                    actual: 0,
                }));
            }
            (None, exp + 1)
        }
        (None, Some(actual)) => {
            (Some(actual), actual + 1)
        }
        (None, None) => {
            (None, 1)
        }
    };

    let payload_hash = compute_sha256(payload);

    // Salva snapshot no histórico de objetos
    save_object_snapshot(root_path, payload, &payload_hash)?;

    // Fase 1: Gravação em arquivo temporário no mesmo diretório
    let tmp_name = format!(".tmp_{}", uuid::Uuid::new_v4());
    let tmp_path = parent_dir.join(&tmp_name);

    fs::write(&tmp_path, payload).map_err(|e| {
        VaultError::Io(format!("Falha ao gravar arquivo temporário de commit: {e}"))
    })?;

    // Fase 2: Registro sequencial e flush no journal diário
    let record = JournalRecord {
        v: 2,
        id: generate_v7_id(),
        ts: Utc::now().timestamp_micros(),
        op: if file_already_exists {
            JournalOperation::Update
        } else {
            JournalOperation::Create
        },
        target: rel_path.to_string_lossy().replace('\\', "/"),
        prev_rev: prev_revision,
        new_rev: next_revision_number,
        hash: payload_hash.clone(),
        snap: Some(payload_hash.clone()),
    };

    if let Err(e) = append_journal_record(root_path, &record) {
        let _ = fs::remove_file(&tmp_path);
        return Err(VaultError::Journal(e));
    }

    // Fase 3: Substituição atômica via rename no sistema operacional
    if let Err(e) = atomic_rename(&tmp_path, &target_abs_path) {
        let _ = fs::remove_file(&tmp_path);
        return Err(VaultError::Io(format!(
            "Falha ao aplicar substituição atômica para {:?}: {}",
            target_abs_path, e
        )));
    }

    // Fase 4: Despacha evento de domínio assíncrono para os indexadores
    if let Some(bus) = event_bus {
        let _ = bus.publish(DomainEvent::DocumentCommitted {
            rel_path: rel_path.to_string_lossy().replace('\\', "/"),
            revision: next_revision_number,
            hash: payload_hash,
            timestamp: Utc::now().timestamp_millis(),
        });
    }

    Ok(next_revision_number)
}
