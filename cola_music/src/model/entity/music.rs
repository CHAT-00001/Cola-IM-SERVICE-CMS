// cola_music/src/entity/user  -- MUSIC - Entity - 音乐
// 2026/5/19 21:12 by wx: cestbon10080

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::{Uuid, uuid};

////////

/// # [ENTITY] - 音乐 数据实体
/// * table name: music
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MusicEntity {
    pub id: i64,                     // id
    pub uuid: Option<String>,        // uuid
    pub author: Option<i64>,         // 用户id
    pub title: Option<String>,       // 标题
    pub description: Option<String>, // 描述
    pub cover_url: Option<String>,   // 封面url
    pub file_url: Option<String>,    // 音频文件url
    pub href: String,                // 视频url
    pub length: Option<String>,      // 时长（旧版）
    pub duration: Option<String>,    // 时长
    pub views: Option<i64>,          // 浏览量
    pub likes: Option<i64>,          // 点赞量
    pub steps: Option<i64>,          // 被踩数量
    pub collects: Option<i64>,       // 收藏量
    pub comments: Option<i64>,       // 评论数量
    pub shares: Option<i64>,         // 分享数量
    pub is_public: Option<bool>,     // 是否公开
    pub status: Option<i16>,         // 状态
    pub use_nums: Option<i64>,       // 被使用数量
    pub add_time: Option<i32>,       // 创建时间（旧版）
    // -- new
    pub created_at: Option<String>, // 创建于
    pub updated_at: Option<String>, // 更新于
}

//////// END
