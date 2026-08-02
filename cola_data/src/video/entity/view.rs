// cola_data/src/new/entity/view.rs  --
// 数据 - VIDEO - entity - 浏览记录表
// 2026/3/28 05:56 创建

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] -- 短视频 浏览记录
/// * `desc`: `视频浏览记录实体`
/// * `pg schema`: `cola_video`
/// * `table name`: `view`
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

////////

/// # [COLUMNS] - 数据表原始字段
/// * `desc`: `给SQLx提供的表字段映射`
pub const VIDEO_VIEW_COLUMNS: &str = r#"
    id, user_id, video_id, time, remark, addtime, status, created_at, updated_at
"#;

//////// END