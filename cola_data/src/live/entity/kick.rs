// handler/kick.rs  -- handler 踢人
// 2026/6/13 07:06

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::time::Duration;

////////

/// # [ENTITY] - 直播流踢人 实体
/// * `table_name` live_stream_kick
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveStreamKickEntity {
    pub id: i64,                           // ID
    pub operator_uid: i64,                 // 操作者ID
    pub live_id: i64,                      // 直播间ID
    pub record_id: i64,                    // 直播场次记录ID
    pub user_id: i64,                      // 目标用户ID
    pub duration: i64,                     // 时长
    pub status: i16,                       // 状态: 0. 失效 1. 有效(默认)
    pub add_time: i32,                     // 添加时间 - 机器
    pub upd_time: i32,                     // 更新时间 - 机器
    pub created_at: chrono::NaiveDateTime, // 创建于 - 人类
    pub updated_at: chrono::NaiveDateTime, // 更新于 - 人类
}

//////// END
