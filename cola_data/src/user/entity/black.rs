// cola_data/src/user/handler/black.rs  -- 可乐数据中心 - USER - handler - 黑名单
// 2026/6/18 10:08

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 用户 - 黑名单表
/// * table name: user_black
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct UserBlackEntity {
    pub id: i64,                          // id
    pub uid: i64,                         // 操作者用户ID
    pub user_id: i64,                     // 目标用户ID
    pub status: i16,                      // 状态
    pub remark: Option<String>,           // 备注
    pub add_time: i32,                    // 添加时间（机器）
    pub upd_time: i32,                    // 更新时间（机器）
    pub create_at: Option<DateTime<Utc>>, // 创建时间（人类）
    pub update_at: Option<DateTime<Utc>>, // 更新时间（人类）
}
