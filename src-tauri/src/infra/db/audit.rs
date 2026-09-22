use crate::domain::core::errors::{SandlandError, SandlandResult};
use rusqlite::Connection;
use serde_json::Value;

pub struct AuditLogger;

impl AuditLogger {
    /// Registra um evento semântico ou operacional de negócio no audit_log.db.
    /// Eventos de arrasto espacial utilizam 'NODE_DRAG_END' (nunca no mousemove de 60fps).
    pub fn log_event(
        conn: &Connection,
        event_type: &str,
        entity_id: &str,
        payload: &Value,
        reversible: bool,
    ) -> SandlandResult<i64> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let payload_str = payload.to_string();

        conn.execute(
            r#"
            INSERT INTO audit_events (timestamp, event_type, entity_id, payload_json, reversible)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            rusqlite::params![timestamp, event_type, entity_id, payload_str, reversible],
        )
        .map_err(|e| SandlandError::DatabaseError(format!("Falha ao gravar log de auditoria: {}", e)))?;

        Ok(conn.last_insert_rowid())
    }
}
