use chrono::Utc;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use crate::domain::core::errors::JournalError;
use crate::domain::model::JournalRecord;
use crate::infra::fs::sync_file;

pub const JOURNAL_DIR: &str = ".history/events";

/// Retorna o caminho do arquivo de journal para uma determinada data UTC
pub fn journal_file_path(root_path: &Path, date: chrono::NaiveDate) -> PathBuf {
    let filename = format!("{}.jsonl", date.format("%Y-%m-%d"));
    root_path.join(JOURNAL_DIR).join(filename)
}

/// Registra uma entrada durável de forma sequencial no journal diário com garantia física de gravação
pub fn append_journal_record(root_path: &Path, record: &JournalRecord) -> Result<(), JournalError> {
    let events_dir = root_path.join(JOURNAL_DIR);
    if !events_dir.exists() {
        fs::create_dir_all(&events_dir).map_err(|e| {
            JournalError::Io(format!("Falha ao criar diretório de journal: {e}"))
        })?;
    }

    let today = Utc::now().date_naive();
    let file_path = journal_file_path(root_path, today);

    let serialized = serde_json::to_string(record).map_err(|e| {
        JournalError::Serialization(format!("Falha ao serializar JournalRecord: {e}"))
    })?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
        .map_err(|e| {
            JournalError::Io(format!("Falha ao abrir arquivo de journal para append: {e}"))
        })?;

    writeln!(file, "{}", serialized).map_err(|e| {
        JournalError::Io(format!("Falha ao gravar linha no journal: {e}"))
    })?;

    // Invariante constitucional: fsync / FlushFileBuffers para garantir permanência física
    sync_file(&file).map_err(|e| {
        JournalError::Io(format!("Falha ao sincronizar buffer do journal no disco: {e}"))
    })?;

    Ok(())
}

/// Lê todos os registros de journal de um determinado arquivo diário
pub fn read_journal_file(path: &Path) -> Result<Vec<JournalRecord>, JournalError> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(path).map_err(|e| {
        JournalError::Io(format!("Falha ao abrir arquivo de journal: {e}"))
    })?;

    let reader = BufReader::new(file);
    let mut records = Vec::new();

    for (line_idx, line_res) in reader.lines().enumerate() {
        let line = line_res.map_err(|e| {
            JournalError::Io(format!("Erro ao ler linha {} do journal: {e}", line_idx + 1))
        })?;

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let record: JournalRecord = serde_json::from_str(trimmed).map_err(|e| {
            JournalError::CorruptedEntry(format!(
                "Linha {} corrompida em {:?}: {}",
                line_idx + 1,
                path,
                e
            ))
        })?;

        records.push(record);
    }

    Ok(records)
}

/// Lê todas as entradas de journal existentes no cofre, ordenadas cronologicamente
pub fn read_all_journals(root_path: &Path) -> Result<Vec<JournalRecord>, JournalError> {
    let events_dir = root_path.join(JOURNAL_DIR);
    if !events_dir.exists() {
        return Ok(Vec::new());
    }

    let mut journal_files = Vec::new();
    let entries = fs::read_dir(&events_dir).map_err(|e| {
        JournalError::Io(format!("Falha ao listar diretório de eventos: {e}"))
    })?;

    for entry_res in entries {
        let entry = entry_res.map_err(|e| JournalError::Io(e.to_string()))?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
            journal_files.push(path);
        }
    }

    // Ordena arquivos por nome de data (YYYY-MM-DD.jsonl)
    journal_files.sort();

    let mut all_records = Vec::new();
    for file_path in journal_files {
        let file_records = read_journal_file(&file_path)?;
        all_records.extend(file_records);
    }

    Ok(all_records)
}

/// Obtém a revisão mais recente registrada no journal para um arquivo relativo ao cofre
pub fn get_latest_file_revision(root_path: &Path, rel_path: &Path) -> Result<Option<u64>, JournalError> {
    let rel_str = rel_path.to_string_lossy().replace('\\', "/");
    let records = read_all_journals(root_path)?;
    for record in records.iter().rev() {
        if record.target == rel_str {
            return Ok(Some(record.new_rev));
        }
    }
    Ok(None)
}
