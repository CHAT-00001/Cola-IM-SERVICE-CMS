// cola_data/src/live/entity/live_user_profile.rs  --  LIVE - Entity - 用户资料
// 2026/5/22 19:03 by wx: cestbon10080
// * --------
// * --------

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 直播 - 用户资料
/// * table name: live_user_profile
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveUserProfileEntity {
    pub user_id: i64,             // 用户 ID
    pub username: String,         // 主播名称
    pub category_id: i64,         // 分类
    pub city: i16,                // 城市
    pub watch_level: i16,         // 观众等级
    pub author_level: i16,        // 主播等级
    pub status: i16,              // 状态
    pub create_time: Option<i32>, // 创建时间戳（UTC）
    pub update_time: Option<i32>, // 修改时间 （UTC）
}

//////// END