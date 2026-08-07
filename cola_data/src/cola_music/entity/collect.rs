// entity/collect.rs
// 数据中心 - MUSIC - entity - 收藏
// 2026/8/3 20:51 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::{Uuid, uuid};

////////

/// # [ENTITY] - 音乐 收藏表
/// * `pg schema`: `cola_music`
/// * `table name`: `collect`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MusicCollectEntity {
    pub id: i64,                           // id (自增 / 雪花)
    pub uuid: Option<String>,              // uuid
    pub uid: i64,                          // 用户ID
    pub music_id: Option<String>,          // 音乐ID
    pub remark: Option<String>,            // 备注
    pub description: Option<String>,       // 描述
    pub album_id: Option<i64>,             // 专辑 ID
    pub status: Option<i16>,               // 状态（0-草稿, 1-正常, 2-审核中, 3-已下架等）
    pub is_deleted: Option<bool>,          // 是否删除: 默认false
    pub add_time: i64,                     // 创建时间（兼容PHP）
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间（软删除）
}

////////

/// # [COLUMNS] - 统一的SQLx查询字段
/// * `描述`: 给SQLX使用，严格与 MusicCollectEntity 结构体字段对齐
pub const MUSIC_COLLECT_COLUMNS: &str = r#"
    id, uuid, uid, music_id, remark, description, album_id, status,
    is_deleted, add_time, created_at, updated_at, deleted_at
"#;

//////// END