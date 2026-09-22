pub mod audit;
pub mod indexer;

use crate::domain::core::errors::{SandlandError, SandlandResult};
use refinery::embed_migrations;
use rusqlite::Connection;
use std::path::Path;
use std::sync::{Arc, Mutex};

embed_migrations!("./migrations");

pub type DbPool = Arc<Mutex<Connection>>;

extern "C" {
    fn sqlite3_vec_init(
        db: *mut rusqlite::ffi::sqlite3,
        pz_err_msg: *mut *mut std::os::raw::c_char,
        p_api: *const rusqlite::ffi::sqlite3_api_routines,
    ) -> std::os::raw::c_int;
}

static INIT_VEC_AUTO_EXT: std::sync::Once = std::sync::Once::new();

pub fn init_database(db_path: &Path) -> SandlandResult<Connection> {
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Registra como extensão automática global para qualquer conexão SQLite
    INIT_VEC_AUTO_EXT.call_once(|| unsafe {
        let _ = rusqlite::ffi::sqlite3_auto_extension(Some(std::mem::transmute(
            sqlite3_vec_init as *const (),
        )));
    });

    let mut conn = Connection::open(db_path)
        .map_err(|e| SandlandError::DatabaseError(format!("Falha ao abrir SQLite: {}", e)))?;

    // Registra explicitamente no handle da conexão aberta
    unsafe {
        let rc = sqlite3_vec_init(conn.handle(), std::ptr::null_mut(), std::ptr::null());
        if rc != rusqlite::ffi::SQLITE_OK {
            return Err(SandlandError::DatabaseError(format!(
                "Falha ao registrar extensão sqlite-vec no handle: rc={}",
                rc
            )));
        }
    }

    // Performance and integrity PRAGMAs
    conn.execute_batch(
        r#"
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA foreign_keys = ON;
        PRAGMA cache_size = -64000;
        "#,
    )
    .map_err(|e| SandlandError::DatabaseError(format!("Falha ao configurar PRAGMAs: {}", e)))?;

    // Run embedded migrations
    migrations::runner()
        .run(&mut conn)
        .map_err(|e| SandlandError::DatabaseError(format!("Falha ao executar migrações refinery: {}", e)))?;

    Ok(conn)
}
