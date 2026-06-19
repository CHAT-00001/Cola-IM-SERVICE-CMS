// cola_data/src/auth/handler/session.rs  -- 数据 - 认证中心 - handler - session
// 2026/5/23 07:45 by wx: cestbon10080

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// # 1. 统一的登录会话查询字段 (1:1 严格对齐结构体，不带任何容易断句的内部注释)
pub const SESSION_COLUMNS: &str = r#"
    id, user_id, access_token, refresh_token, client_id,
    device_id, device_name, last_ip, platform, expired_time, last_active_at,
    status, created_time, updated_time
"#;

/// # [ENTITY] - 认证中心 - 会话
/// * table name: auth_session
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct AuthSessionEntity {
    pub id: i64,                                     // 会话 ID (PG 大自增)
    pub user_id: i64,                                // 用户 ID
    pub access_token: String,                        // 访问临牌 Token
    pub refresh_token: String,                       // 刷新 Token 哈希值
    pub client_id: String,                           // 客户端 ID
    pub device_id: String,                           // 设备号 ID
    pub access_expired_at: i64,                      // 临时token过期时间
    pub refresh_expired_at: i64,                     // 刷新token过期时间
    pub last_active_at: i64,                         // 最后活跃时间戳
    pub status: i16,                                 // 状态: 1有效, 0注销, -1被挤掉
    pub created_time: chrono::DateTime<chrono::Utc>, // 创建时间
    pub updated_time: chrono::DateTime<chrono::Utc>, // 同步更新时间
    pub platform: String,                            // 平台名称
}
