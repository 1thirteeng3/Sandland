use sandland_lib::domain::core::security::validate_url_for_ssrf;

#[test]
fn test_ssrf_rejects_loopback_ipv4() {
    let res = validate_url_for_ssrf("http://127.0.0.1/admin");
    assert!(res.is_err(), "Deve rejeitar IPv4 de loopback");
    let err_msg = format!("{}", res.unwrap_err());
    assert!(err_msg.contains("SSRF") || err_msg.contains("bloqueado"));
}

#[test]
fn test_ssrf_rejects_localhost_hostname() {
    let res = validate_url_for_ssrf("http://localhost:8080/secret");
    assert!(res.is_err(), "Deve rejeitar hostname localhost");
}

#[test]
fn test_ssrf_rejects_private_class_c() {
    let res = validate_url_for_ssrf("http://192.168.1.100/router");
    assert!(res.is_err(), "Deve rejeitar IP privado RFC 1918 (192.168.x.x)");
}

#[test]
fn test_ssrf_rejects_private_class_a() {
    let res = validate_url_for_ssrf("http://10.0.0.1/internal");
    assert!(res.is_err(), "Deve rejeitar IP privado RFC 1918 (10.x.x.x)");
}

#[test]
fn test_ssrf_rejects_link_local() {
    let res = validate_url_for_ssrf("http://169.254.169.254/latest/meta-data");
    assert!(res.is_err(), "Deve rejeitar link-local de metadata cloud");
}

#[test]
fn test_ssrf_rejects_file_scheme() {
    let res = validate_url_for_ssrf("file:///etc/passwd");
    assert!(res.is_err(), "Deve rejeitar scheme file://");
}

#[test]
fn test_ssrf_allows_public_url() {
    let res = validate_url_for_ssrf("https://example.com/article");
    assert!(res.is_ok(), "Deve aceitar URL pública válida");
}
