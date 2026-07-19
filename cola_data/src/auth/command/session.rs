// cola_data/src/auth/dynamic/session.rs  -- 可乐数据中心 - AUTH - Command - session
// 2026/06/05 06:50

////////

use serde::{Deserialize, Serialize};
use validator::Validate;

////////

/// # [COMMAND] - 会话命令
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SessionCommand {
    pub client_id: String,                                 // 客户端 ID
    pub device_id: String,                                 // 设备 ID
    pub access_token: String,                              // jwt token
    pub refresh_token: String,                             // aes token
    pub access_expires_at: chrono::DateTime<chrono::Utc>,  // 临时token过期时间
    pub refresh_expires_at: chrono::DateTime<chrono::Utc>, // 刷新Token过期时间
    pub last_active_at: chrono::DateTime<chrono::Utc>,     // 最后一次激活时间(更新时间)
}

// 构造函数
impl SessionCommand {
    ////////

    /// # 构造函数：基于基本信息生成标准有效期 Token 命令
    /// * 10分钟短 Token + 180天长 Refresh Token
    pub fn new_with_defaults(access_token: String, refresh_token: String) -> Self {
        let now = chrono::Utc::now();
        let client_id = "a".to_string();
        let device_id = "b".to_string();
        let access_expires_at = now + chrono::Duration::minutes(10);
        let refresh_expires_at = now + chrono::Duration::days(180);
        let last_active_at = now;

        Self {
            client_id,
            device_id,
            access_token,
            refresh_token,
            access_expires_at,
            refresh_expires_at,
            last_active_at,
        }
    }
}

//////// END
