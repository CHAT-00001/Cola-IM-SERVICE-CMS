// cola_data/src/handler/video/comment.rs  -- 数据 - handler -  短视频 - 评论表
// 2026-01-16 09:37:10
// * 1个结构体 1个构造函数
// * --------

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::{Uuid, uuid};

////////

/// # [ENTITY] - 视频 - 评论表
/// * `table name`: video_comment
/// * `类型`: 1. 文字 2. 语音 3. 照片 4. 视频 5. 位置 7. 表情包 8. 红包 9. 转账 ...
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct CommentEntity {
    pub id: i64,                           // id
    pub user_id: i64,                      // 作者 ID
    pub video_id: i64,                     // 视频 ID
    pub parent_id: Option<i64>,            // 父评论（可选）
    pub comment_type: i16,                 // 类型
    pub content: String,                   // 内容
    pub photos_url: Option<String>,        // 照片 url
    pub video_url: Option<String>,         // 视频 url
    pub voice_url: Option<String>,         // 语音 url
    pub duration: Option<i32>,             // 时长
    pub likes: i32,                        // 点赞量
    pub dislikes: i32,                     // 被踩数量
    pub collects: i32,                     // 收藏量
    pub reply: i32,                        // 回复数量（是父评论时）
    pub visibility: i16,                   // 可见范围
    pub region_code: Option<String>,       // i18n 地区码
    pub status: i16,                       // 状态
    pub add_time: i64,                     // 添加时间（机器）
    pub upd_time: i64,                     // 更新时间（机器）
    pub created_at: Option<DateTime<Utc>>, // 创建时间(人类)
    pub updated_at: Option<DateTime<Utc>>, // 更新时间(人类)
    pub send_time: i64,
    pub sync_time: i64,
}

// * --------
//////// END
