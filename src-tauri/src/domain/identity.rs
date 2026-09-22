use uuid::Uuid;

/// Gera um identificador UUID v7 monotônico com timestamp embutido
pub fn generate_v7_id() -> String {
    Uuid::now_v7().to_string()
}

/// Gera um identificador UUID v4 aleatório criptograficamente seguro
pub fn generate_v4_id() -> String {
    Uuid::new_v4().to_string()
}

/// Valida se uma string é um formato de UUID válido
pub fn validate_uuid(id: &str) -> bool {
    Uuid::parse_str(id).is_ok()
}

/// Extrai o timestamp em milissegundos a partir de um UUID v7
pub fn parse_v7_timestamp_millis(id: &str) -> Option<u64> {
    let parsed = Uuid::parse_str(id).ok()?;
    if parsed.get_version_num() != 7 {
        return None;
    }
    // UUID v7 armazena o timestamp Unix de 48 bits nos primeiros 6 bytes (big-endian)
    let bytes = parsed.as_bytes();
    let millis = ((bytes[0] as u64) << 40)
        | ((bytes[1] as u64) << 32)
        | ((bytes[2] as u64) << 24)
        | ((bytes[3] as u64) << 16)
        | ((bytes[4] as u64) << 8)
        | (bytes[5] as u64);
    Some(millis)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v7_generation_and_validation() {
        let id1 = generate_v7_id();
        let id2 = generate_v7_id();
        assert!(validate_uuid(&id1));
        assert!(validate_uuid(&id2));
        assert_ne!(id1, id2);
        let ts1 = parse_v7_timestamp_millis(&id1);
        assert!(ts1.is_some());
        assert!(ts1.unwrap() > 0);
    }

    #[test]
    fn test_v4_generation_and_validation() {
        let id = generate_v4_id();
        assert!(validate_uuid(&id));
    }
}
