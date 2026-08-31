use serde::{Deserialize, Serialize};

/// # ApiRequestBody
/// # 统一的请求体
/// * 用来追踪请求者的信息
/// ** 2025-09-16 14:40:10
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiRequest<T> {
    pub uid: Option<i64>,
    pub token: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub data: Option<T>,
}

/// # 客户端信息
/// # Client Info
/// ** 生产模式时，需要强验证客户端合法性。
/// --
/// DATE: 2025-09-16 14:46:10
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientInfo {
    pub client_id: i64,
    pub client_secret: String,
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub token_type: String,
}
