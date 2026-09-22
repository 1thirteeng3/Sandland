use crate::domain::core::errors::{SandlandError, SandlandResult};
use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, FileIdMap};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;
use tauri::Emitter;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileWatcherPayload {
    pub vault_path: String,
    pub kind: String, // "Created" | "Modified" | "Removed"
}

pub struct VaultWatcher {
    _debouncer: Debouncer<notify::RecommendedWatcher, FileIdMap>,
}

impl VaultWatcher {
    pub fn start<R: tauri::Runtime>(
        ingest_dir: &Path,
        vault_root: &Path,
        app_handle: Option<tauri::AppHandle<R>>,
    ) -> SandlandResult<Self> {
        let (tx, rx) = mpsc::channel();

        let mut debouncer = new_debouncer(
            Duration::from_millis(250),
            None,
            move |res: DebounceEventResult| {
                if let Ok(events) = res {
                    let _ = tx.send(events);
                }
            },
        )
        .map_err(|e| SandlandError::IoError(format!("Falha ao iniciar File Watcher: {}", e)))?;

        debouncer
            .watcher()
            .watch(ingest_dir, RecursiveMode::Recursive)
            .map_err(|e| SandlandError::IoError(format!("Falha ao monitorar diretório: {}", e)))?;

        let vault_root_buf = vault_root.to_path_buf();
        // Spawns background worker to process debounced events and emit to frontend
        std::thread::spawn(move || {
            while let Ok(events) = rx.recv() {
                for event in events {
                    for path in &event.paths {
                        // Process only .md files
                        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
                            continue;
                        }

                        let kind = match event.kind {
                            notify::EventKind::Create(_) => "Created",
                            notify::EventKind::Modify(_) => "Modified",
                            notify::EventKind::Remove(_) => "Removed",
                            _ => "Modified",
                        };

                        let rel_path = path
                            .strip_prefix(&vault_root_buf)
                            .unwrap_or(path)
                            .to_string_lossy()
                            .replace('\\', "/");

                        let payload = FileWatcherPayload {
                            vault_path: rel_path,
                            kind: kind.to_string(),
                        };

                        if let Some(ref handle) = app_handle {
                            let _ = handle.emit("vault://file-watcher-event", payload);
                        }
                    }
                }
            }
        });

        Ok(Self {
            _debouncer: debouncer,
        })
    }
}
