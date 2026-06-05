// /session.rs  --
// 2026/4/16 00:34 by wx: cestbon10080

////////

use chrono::{DateTime, Utc};
use ipnetwork::IpNetwork;

////////

/// # Session 领域模型（core使用）
///
/// ⚠️ 不包含 DB id / created_at / updated_at
/// ⚠️ 只表达“业务含义”
#[derive(Debug, Clone)]
pub struct AuthSessionEntity {
    pub id: i64,                           // ID
    pub user_id: i64,                      // 用户ID
    pub device_id: String,                 // 设备ID
    pub platform: String,                  // 平台（ios/android/web）
    pub login_ip: Option<IpNetwork>,       // 登录IP（可选）
    pub access_token: String,              // Access Token
    pub refresh_token: String,             // Refresh Token
    pub is_revoked: bool,                  // 是否被吊销
    pub last_active: DateTime<Utc>,        // 最后活跃时间
    pub expired_at: DateTime<Utc>,         // 会话过期时间（整体）
    pub access_expired_at: DateTime<Utc>,  // access token 过期时间
    pub refresh_expired_at: DateTime<Utc>, // refresh token 过期时间
    pub created_at: DateTime<Utc>,         // 创建时间
    pub updated_at: DateTime<Utc>,         // 更新时间
}
