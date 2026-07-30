// repository/src/live/gate_grpc/video_response  -- 浏览记录实体
// 2026/3/28 05:56 by wx: cestbon10080

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// # ENTITY -- layer
/// # 视频浏览记录实体
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VideoViewEntity {
    pub id: i64,                           // ID
    pub user_id: i64,                      // 用户ID
    pub video_id: i64,                     // 视频ID
    pub time: i32,                         // 观看到的时间
    pub remark: Option<String>,            // 备注
    pub addtime: i32,                      // 创建时间（兼容PHP旧版）
    pub status: i16,                       // 状态 0. 失效  1. 正常
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}
