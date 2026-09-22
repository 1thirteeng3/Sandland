use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use crate::domain::core::errors::VaultResult;
use crate::domain::model::RecoveryReport;
use crate::infra::fs::journal::read_all_journals;
use crate::infra::fs::writer::{compute_sha256, OBJECTS_DIR};
use crate::infra::fs::write_file_atomic;

/// Executa a rotina idempotente de recuperação no boot
pub fn run_boot_recovery(root_path: &Path) -> VaultResult<RecoveryReport> {
    let mut report = RecoveryReport::default();

    // 1. Purga de arquivos temporários órfãos (.tmp_*)
    for entry in WalkDir::new(root_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if file_name.starts_with(".tmp_") {
                    if fs::remove_file(path).is_ok() {
                        report.purged_temporary_files += 1;
                    }
                }
            }
        }
    }

    // 2. Reconciliação do journal com arquivos canônicos
    match read_all_journals(root_path) {
        Ok(records) => {
            for record in records {
                let target_file = root_path.join(&record.target);
                let needs_reconciliation = if target_file.exists() {
                    match fs::read(&target_file) {
                        Ok(bytes) => compute_sha256(&bytes) != record.hash,
                        Err(_) => true,
                    }
                } else {
                    true
                };

                if needs_reconciliation {
                    if let Some(ref snap_hash) = record.snap {
                        let snap_file = root_path.join(OBJECTS_DIR).join(snap_hash);
                        if snap_file.exists() {
                            if let Ok(snap_bytes) = fs::read(&snap_file) {
                                if let Some(parent) = target_file.parent() {
                                    let _ = fs::create_dir_all(parent);
                                }
                                if write_file_atomic(&target_file, &snap_bytes).is_ok() {
                                    report.replayed_operations += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(_) => {
            report.corrupted_records += 1;
        }
    }

    report.success = true;
    Ok(report)
}
