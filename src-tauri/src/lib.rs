pub mod domain;
pub mod infra;
#[cfg(feature = "app")]
pub mod ipc;

#[cfg(feature = "app")]
use ipc::vault::AppState;
#[cfg(feature = "app")]
use tauri::Manager;

#[cfg(feature = "app")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState::new())
        .setup(|app| {
            let user_home = app
                .path()
                .home_dir()
                .or_else(|_| app.path().document_dir())
                .map_err(|e| format!("Falha ao resolver diretório do usuário: {e}"))?;
            let vault_path = user_home.join("SandlandVault");
            let system_path = vault_path.join(".system");

            // Criação explícita de toda a árvore de diretórios antes do banco de dados
            std::fs::create_dir_all(&system_path)
                .map_err(|e| format!("Falha ao criar diretórios do cofre: {e}"))?;
            let _ = crate::infra::fs::vault::initialize_vault_structure(&vault_path);

            let db_path = system_path.join("index.db");
            let mut conn = crate::infra::db::init_database(&db_path).map_err(|e| {
                eprintln!("[ERRO CRITICO] Falha ao iniciar SQLite: {:?}", e);
                format!("{e:?}")
            })?;

            let guard = crate::infra::fs::vault::VaultGuard::new(vault_path.clone()).map_err(|e| {
                eprintln!("[ERRO CRITICO] Falha ao criar VaultGuard: {:?}", e);
                format!("{e:?}")
            })?;

            // Reidratação a frio a partir do disco (Cold Start)
            match crate::infra::fs::vault::hydrate_vault_from_disk(&guard, &mut conn) {
                Ok(count) => println!("[SANDLAND] Reidratação a frio: {count} documentos sincronizados com index.db."),
                Err(e) => eprintln!("[AVISO] Falha ao reidratar cofre a frio: {e:?}"),
            }

            // Inicializa o workspace padrão 'default-workspace'
            let default_ws = vault_path.join("workspaces/default-workspace");
            let _ = std::fs::create_dir_all(&default_ws);
            let topo_path = default_ws.join("topology.json");
            if !topo_path.exists() {
                let topology = crate::domain::workspace::topology::BoardTopology::new("default-workspace".to_string());
                let _ = crate::infra::fs::canvas_io::CanvasStorage::save_topology(&guard, "default-workspace", &topology);
            }

            let state = app.state::<AppState>();
            *state.vault_guard.lock().unwrap() = Some(guard);
            *state.db.lock().unwrap() = Some(std::sync::Arc::new(std::sync::Mutex::new(conn)));

            println!("[SANDLAND] Cofre inicializado com sucesso em: {}", vault_path.display());
            println!("[SANDLAND] Banco SQLite ativo em: {}", db_path.display());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Vault IPC
            ipc::vault::create_vault,
            ipc::vault::init_vault,
            ipc::vault::open_vault,
            ipc::vault::commit_document,
            ipc::vault::store_asset,
            ipc::vault::read_asset,
            ipc::vault::get_vault_status,
            // Ingest IPC
            ipc::ingest::ingest_file,
            ipc::ingest::ingest_file_content,
            ipc::ingest::ingest_url,
            ipc::ingest::list_ingested_items,
            // Taxonomy IPC
            ipc::taxonomy::trigger_classification,
            ipc::taxonomy::update_item_tags,
            // Workspace & Canvas IPC
            ipc::workspace::create_workspace,
            ipc::workspace::load_board_topology,
            ipc::workspace::save_board_topology,
            ipc::workspace::save_board_topology_fast,
            ipc::workspace::create_cell,
            ipc::workspace::list_cells,
        ])
        .run(tauri::generate_context!())
        .expect("Erro ao executar aplicação Sandland Tauri");
}
