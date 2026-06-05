// cola_data/src/auth/command/session.rs  -- 可乐数据中心 - AUTH - Command - session
// 2026/06/05 06:50 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};
use validator::Validate;

////////

/// # [COMMAND] - 会话创建/更新命令
/// * 机制：用于从 API 层携带完整的登录/会话上下文，直接映射至 AuthSessionEntity
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SessionCommand {
    // --- 核心鉴权信息 ---
    pub access_token: String,  // 修正拼写: assess_token -> access_token
    pub refresh_token: String,

    // --- 业务关联信息 ---
    pub user_id: i64,
    pub send_id: String,      // 客户端生成 UUID
    pub sync_id: String,      // 服务端分布式唯一 ID

    // --- 设备/环境上下文 ---
    pub client_id: i32,
    pub device_id: String,
    pub device_name: String,
    pub login_ip: String,
    pub platform: i32,        // 1-ios, 2-android, 3-web

    // --- 时效管理 ---
    // 💡 建议 Command 层为了灵活性传 i64 时间戳，Service 层再根据需要转 DateTime
    pub expired_time: chrono::DateTime<chrono::Utc>,
    pub last_active_at: chrono::DateTime<chrono::Utc>,
}

// cola_data/src/auth/command/session.rs

impl SessionCommand {
    /// # 构造函数：基于基本信息生成标准有效期 Token 命令
    /// * 10分钟短 Token + 180天长 Refresh Token
    pub fn new_with_defaults(
        user_id: i64,
        access_token: String,
        refresh_token: String,
        device_id: String,
        platform: i32,
    ) -> Self {
        let now = chrono::Utc::now();

        // 🚀 这里封装时效逻辑：10分钟过期 vs 180天过期
        let expired_time = now + chrono::Duration::minutes(10);
        let last_active_at = now + chrono::Duration::days(180);

        Self {
            access_token,
            refresh_token,
            user_id,
            send_id: uuid::Uuid::new_v4().to_string(), // 自动生成 send_id
            sync_id: uuid::Uuid::new_v4().to_string(), // 自动生成 sync_id
            client_id: 1, // 默认值
            device_id,
            device_name: "Unknown Device".to_string(),
            login_ip: "0.0.0.0".to_string(),
            platform,
            expired_time,
            last_active_at,
        }
    }
}




//////// END