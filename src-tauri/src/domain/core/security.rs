use crate::domain::core::errors::{SandlandError, SandlandResult, SecurityError};
use std::net::{IpAddr, Ipv4Addr, ToSocketAddrs};

/// Valida se uma URL é segura para extração web externa contra ataques SSRF (Princípio III da Constituição)
pub fn validate_url_for_ssrf(url_str: &str) -> SandlandResult<()> {
    let clean_url = url_str.trim();

    if !clean_url.starts_with("http://") && !clean_url.starts_with("https://") {
        return Err(SandlandError::InvalidInput(
            "Apenas URLs HTTP ou HTTPS são permitidas".to_string(),
        ));
    }

    // Extrai o host
    let without_scheme = clean_url
        .trim_start_matches("http://")
        .trim_start_matches("https://");
    let host_and_port = without_scheme.split('/').next().unwrap_or("");
    let host = host_and_port.split(':').next().unwrap_or("");

    if host.is_empty() {
        return Err(SandlandError::InvalidInput("Host da URL está vazio".to_string()));
    }

    let host_lower = host.to_lowercase();

    // Rejeição direta de nomes de loopback e metadados de nuvem comuns
    if host_lower == "localhost"
        || host_lower.ends_with(".localhost")
        || host_lower == "169.254.169.254"
        || host_lower == "metadata.google.internal"
    {
        return Err(SandlandError::Security(SecurityError::SsrfBlocked(format!(
            "Acesso a host interno ou loopback ('{}') bloqueado por segurança anti-SSRF",
            host
        ))));
    }

    // Tenta interpretar como IP direto
    if let Ok(ip) = host.parse::<IpAddr>() {
        return check_ip_ssrf(ip);
    }

    // Resolução DNS preventiva com checagem de IP retornado
    let addr_str = format!("{}:80", host);
    if let Ok(addrs) = addr_str.to_socket_addrs() {
        for socket_addr in addrs {
            check_ip_ssrf(socket_addr.ip())?;
        }
    }

    Ok(())
}

fn check_ip_ssrf(ip: IpAddr) -> SandlandResult<()> {
    match ip {
        IpAddr::V4(v4) => {
            if v4.is_loopback() || v4 == Ipv4Addr::new(0, 0, 0, 0) {
                return Err(SandlandError::Security(SecurityError::SsrfBlocked(
                    "Acesso a loopback (127.0.0.0/8) bloqueado".to_string(),
                )));
            }
            if v4.is_private() {
                return Err(SandlandError::Security(SecurityError::SsrfBlocked(format!(
                    "Acesso a IP privado ('{}') bloqueado por conformidade com RFC 1918",
                    v4
                ))));
            }
            if v4.is_link_local() {
                return Err(SandlandError::Security(SecurityError::SsrfBlocked(format!(
                    "Acesso a IP link-local ('{}') bloqueado",
                    v4
                ))));
            }
        }
        IpAddr::V6(v6) => {
            if v6.is_loopback() {
                return Err(SandlandError::Security(SecurityError::SsrfBlocked(
                    "Acesso a loopback IPv6 bloqueado".to_string(),
                )));
            }
            // Verifica IPv4-mapped loopback/private
            if let Some(v4) = v6.to_ipv4_mapped() {
                return check_ip_ssrf(IpAddr::V4(v4));
            }
        }
    }

    Ok(())
}
