// network lib src/http/client.rs -- Http 客户端处理
// 2026-03-11 06:51:10

/// # 客户机状态
#[derive(Debug, Clone)]
pub struct Client {
    pub id: String,     // 客户端ID
    pub secret: String, // 秘钥
    pub token: String,  // 偷啃
    pub host: String,   // 客户端IP
    pub device_id: u16, // 设备ID
}
