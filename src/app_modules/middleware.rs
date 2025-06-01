use axum::{
    body::Body,
    extract::ConnectInfo,
    http::{HeaderMap, Request},
    middleware::Next,
    response::Response,
};
use std::net::{IpAddr, SocketAddr};

/// Represents a client's IP address
#[derive(Clone, Copy, Debug)]
pub struct ClientIp(pub IpAddr);

impl ClientIp {
    /// Extracts the client IP from headers or falls back to socket address
    fn extract_from(headers: &HeaderMap, socket_addr: &SocketAddr) -> Self {
        let ip = Self::extract_from_x_forwarded_for(headers).unwrap_or_else(|| socket_addr.ip());

        ClientIp(ip)
    }

    /// Attempts to extract client IP from X-Forwarded-For header
    fn extract_from_x_forwarded_for(headers: &HeaderMap) -> Option<IpAddr> {
        headers
            .get("x-forwarded-for")
            .and_then(|header_value| header_value.to_str().ok())
            .and_then(|header_str| header_str.split(',').next()) // Get first IP if multiple
            .and_then(|ip_str| ip_str.trim().parse().ok())
    }
}

/// Middleware that extracts the client IP address and adds it to request extensions
pub async fn extract_client_ip(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    // Extract client IP and store in request extensions
    let client_ip = ClientIp::extract_from(&headers, &addr);
    req.extensions_mut().insert(client_ip);

    // Continue processing the request
    next.run(req).await
}
