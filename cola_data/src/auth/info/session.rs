// cola_auth/vo/info/session.rs  --  AUTH - Info - 会话信息
// 2026/4/12 03:20 by wx: cestbon10080

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] 认证中心 - 会话信息
/// *
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthSessionInfo {
    pub key: String,                 // 兼容性字段，通常放用户 ID 或 UUID
    pub access_token: String,        // 临时 Token (JWT)
    pub refresh_token: String,       // 刷新 Token
    pub access_expired_at: DateTime<Utc>,  // 临时 Token 过期时间
    pub refresh_expired_at: DateTime<Utc>, // 长期 Token 过期时间
}

//////// EDN
