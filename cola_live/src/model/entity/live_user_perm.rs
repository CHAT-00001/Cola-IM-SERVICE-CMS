// cola_data/src/live/entity//live_user_perm.rs  -- LIVE - Entity - 用户直播权限表
// 2026/5/22 18:55 by wx: cestbon10080
// * --------
// * --------

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 直播 - 用户权限
/// * table name: live_user_permission
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveUserPermEntity {
    pub user_id: i64,             // 用户 ID
    pub start_perm: i16,          // 开播权限
    pub watch_perm: i16,          // 观看权限
    pub chat_perm: i32,           // 聊天权限
    pub link_perm: i16,           // 连麦权限
    pub status: i16,              // 状态
    pub create_time: Option<i32>, // 创建时间戳（UTC）
    pub update_time: Option<i32>, // 修改时间 （UTC）
}
