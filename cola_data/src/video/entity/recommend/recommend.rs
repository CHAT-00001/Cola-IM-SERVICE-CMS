// cola_data/src/video/entity/recommend/recommend.rs
// 数据 - VIDEO - entity - recommend - recommend record 记录
// 2026/8/2 14:00 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 视频 推荐表
/// * `pg schema`: `cola_video`
/// * `table name`: `recommend_record`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct RecommendRecordEntity {
    pub id: i64,                           // ID
    pub uid: i64,                          // 谁推荐的
    pub video_id: i64,                     // 推荐了哪个视频
    pub remark: Option<String>,            // 备注
    pub status: i16,                       // 状态码: 0失效 1有效
    pub is_deleted: bool,                  // 是否删除: 默认false
    pub add_time: i64,                     // 添加时间(时间戳)
    pub created_at: DateTime<Utc>,         // 创建时间
    pub updated_at: DateTime<Utc>,         // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}

////////

/// # [CONSTANT] - 推荐记录表字段常量定义
pub const RECOMMEND_RECORD_COLUMNS: &str = "\
    id, uid, video_id, remark, \
    status, is_deleted, \
    add_time, created_at, updated_at, deleted_at\
    ";

//////// END
