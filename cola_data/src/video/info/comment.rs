// cola_data/src/new/info/comment.rs  --
// 数据 - VIDEO - 元信息 - 评论
// 2026/5/21 00:58

////////

use crate::video::entity::comment::comment::VideoCommentEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 视频 评论
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoCommentInfo {
    pub id: i64,                           // 评论 ID
    pub user_id: i64,                      // 用户 ID
    pub video_id: i64,                     // 视频 ID
    pub parent_id: Option<i64>,            // 爸爸评论（可选）
    pub content: String,                   // 内容
    pub likes: i32,                        // 点赞数量
    pub reply: i32,                        // 回复数量
    pub add_time: i64,                     // 发布时间
    pub created_at: Option<DateTime<Utc>>, // 创建时间 (新版)
}

/// # 构造评论
impl VideoCommentInfo {
    /// 构造评论基础信息
    pub fn new(
        id: i64,
        user_id: i64,
        video_id: i64,
        parent_id: Option<i64>,
        content: String,
        likes: i32,
        reply: i32,
        add_time: i64,
        created_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            user_id,
            video_id,
            parent_id,
            content,
            likes,
            reply,
            add_time,
            created_at,
        }
    }

    /// 纯净的从数据库实体转换为评论域模型
    pub fn from_entity(entity: VideoCommentEntity) -> Self {
        Self {
            id: entity.id,
            user_id: entity.user_id,
            video_id: entity.video_id,
            parent_id: entity.parent_id,
            content: entity.content,
            likes: entity.likes,
            reply: entity.reply,
            add_time: entity.add_time,
            created_at: entity.created_at,
        }
    }
}

//////// END