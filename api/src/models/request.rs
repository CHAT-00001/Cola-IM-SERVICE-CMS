// api/src/models/request.rs  -- Api 请求体
// 2026-01-04 07:10:10

use serde::{Deserialize, Serialize};

/// 通用 API 请求结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequest {
    pub request_id: String,
    pub client_id: Option<String>,
    pub user_id: Option<i64>,
    pub token: Option<String>,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
}
