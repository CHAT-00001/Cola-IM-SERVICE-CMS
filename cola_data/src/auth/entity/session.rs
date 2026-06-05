// cola_data/src/auth/entity/session.rs  -- 数据 - 认证中心 - entity - session
// 2026/5/23 07:45 by wx: cestbon10080

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// # 1. 统一的登录会话查询字段 (1:1 严格对齐结构体，不带任何容易断句的内部注释)
pub const SESSION_COLUMNS: &str = r#"
    id, send_id, sync_id, user_id, access_token, refresh_token, client_id,
    device_id, device_name, last_ip, platform, expired_time, last_active_at,
    status, created_time, updated_time
"#;

/// # [ENTITY] - 认证中心 - 会话
/// * table name: auth_session
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct AuthSessionEntity {
    pub id: i64,                                     // 会话 ID (PG 大自增)
    pub send_id: String,                             // 发送 ID （客户端生成短 UUID）
    pub sync_id: String,                             // 同步 ID (服务端分布式唯一 ID)
    pub user_id: i64,                                // 用户 ID
    pub access_token: String,                        // 访问临牌 Token
    pub refresh_token: String,                       // 刷新 Token 哈希值
    pub client_id: i32,                              // 客户端 ID
    pub device_id: String,                              // 设备号 ID (如果你上层是用字符串，这里建议换成 String，如果是纯数字 ID 则保留 i32)
    pub device_name: String,                         // 设备名称 (如: "iPhone 15 Pro")
    pub last_ip: String,                             // ⚠️ 修正：最后登录的IP建议用 String，因为 i32 存不下 IPv6 地址
    pub platform: i32,                               // 平台类型: 1-ios, 2-android, 3-web
    pub expired_time: i64,                           // ⚠️ 修正：时间戳建议用 i64，避免 2038 年 i32 溢出时间炸弹
    pub last_active_at: i64,                         // ⚠️ 修正：最后活跃时间戳用 i64
    pub status: i16,                                 // 状态: 1有效, 0注销, -1被挤掉
    pub created_time: chrono::DateTime<chrono::Utc>, // 创建时间
    pub updated_time: chrono::DateTime<chrono::Utc>, // 同步更新时间
}