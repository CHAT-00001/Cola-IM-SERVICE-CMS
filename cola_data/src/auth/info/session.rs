// cola_data/src/auth/info/session.rs  --  可乐数据中心 - AUTH - Info - 会话信息
// 2026/4/12 03:20 by wx: cestbon10080

////////

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use crate::auth::command::session::SessionCommand;
////////

/// # [INFO] - 会话信息
/// *
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionInfo {
    pub access_token: String,              // 临时 Token (JWT)
    pub refresh_token: String,             // 刷新 Token
    pub access_expired_at: DateTime<Utc>,  // 临时 Token 过期时间
    pub refresh_expired_at: DateTime<Utc>, // 长期 Token 过期时间
}

/// # [INFO] - 刷新ACCESS_TOKEN
/// *
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccessTokenInfo {
    pub access_token: String,              // 临时 Token (JWT)
    pub access_expired_at: DateTime<Utc>,  // 临时 Token 过期时间
}


impl From<SessionCommand> for SessionInfo {
    fn from(cmd: SessionCommand) -> Self {
        // 获取当前时间
        let now = Utc::now();

        Self {
            access_token: cmd.access_token,
            refresh_token: cmd.refresh_token,
            // 假设 access_token 有效期为 1 小时
            access_expired_at: now + Duration::hours(1),
            // 假设 refresh_token 有效期为 7 天
            refresh_expired_at: now + Duration::days(7),
        }
    }
}



//////// EDN
