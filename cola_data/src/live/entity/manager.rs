// /manager.rs  -- 
// 2026/6/13 07:07

////////



////////

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 直播流记录 实体
/// * `table_name` live_stream_shut
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveStreamShutEntity {
    pub id: i64,        // ID
    pub uid: i64,       // 操作者ID
    pub record_id: i64, // 直播场次记录ID
    pub user_id: i64,   // 目标用户ID
}
//////// END