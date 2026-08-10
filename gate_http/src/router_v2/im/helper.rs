// gate_http/src/router_v2/im/helper.rs
// 🔌 IM - Helper - 辅助函数
// 2026/8/8 Created.

////////

use actix_web::HttpRequest;

////////

/// # [HELPER] - 从 HttpRequest 提取客户端真实 IP
/// * `desc`: 优先级: X-Forwarded-For > X-Real-IP > peer_addr
pub fn extract_client_ip(req: &HttpRequest) -> String {
    if let Some(ip) = req
        .headers()
        .get("X-Forwarded-For")
        .and_then(|v| v.to_str().ok())
    {
        return ip.to_string();
    }
    if let Some(ip) = req.headers().get("X-Real-IP").and_then(|v| v.to_str().ok()) {
        return ip.to_string();
    }
    if let Some(addr) = req.peer_addr() {
        return addr.ip().to_string();
    }
    "0.0.0.0".to_string()
}

/// # [HELPER] - 从 body 中提取 cmd（最小侵入）
pub fn extract_cmd<T>(body: &actix_web::web::Bytes) -> Option<T>
where
    T: serde::de::DeserializeOwned,
{
    let v: serde_json::Value = serde_json::from_slice(body).ok()?;
    v.get("cmd")
        .cloned()
        .and_then(|cmd| serde_json::from_value(cmd).ok())
}

//////// END
