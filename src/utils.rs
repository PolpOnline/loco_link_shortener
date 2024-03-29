use axum::http::HeaderMap;
use axum_client_ip::SecureClientIp;

/// Get the IP address from the request headers (railway.app includes the real
/// IP in the "x-Envoy-external-Address" or "x-forwarded-for" headers)
pub fn get_ip(secure_ip: &SecureClientIp, headers: &HeaderMap) -> String {
    if let Some(ip) = headers
        .get("x-Envoy-external-Address")
        .and_then(|header| header.to_str().ok())
    {
        return ip.to_string();
    }

    if let Some(ip) = headers
        .get("x-forwarded-for")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.split(',').last())
    {
        return ip.to_string();
    }

    secure_ip.0.to_canonical().to_string()
}
