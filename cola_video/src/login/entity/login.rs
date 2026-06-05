// cola_video/src/login/gateway/session

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LoginSessionEntity {
    pub id: i64,                     // id
    pub uuid: String,                // Session唯一标识
    pub user_id: i64,                // 关联用户ID
    pub token_hash: String,          // Token哈希
    pub client_id: Option<String>,   // 客户端id
    pub device_id: String,           // 设备唯一标识 (指纹)
    pub device_name: Option<String>, // 设备名称
    pub last_ip: String,             // 最后登录ip
    pub platform: String,            // 平台类型 (ios, android, web)
    pub expired_at: i64,             // 过期时间戳
    pub last_active_at: i64,         // 最后活跃时间
    pub status: i8,                  // 1:有效, 0:注销, -1:被挤掉
    pub created_at: i64,             // 首次登录时间
    pub updated_at: i64,             // 记录更新时间
}
