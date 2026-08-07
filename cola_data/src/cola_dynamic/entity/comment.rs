// cola_data/src/cola_dynamic/entity/add  -- 数据中心 - DYNAMIC - entity - 动态 评论
// 2026/6/19 16:22

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::{Uuid, uuid};

////////

/// # [ENTITY] - 动态 评论 实体表
/// * `pg schema`: `cola-cola_dynamic`
/// * `table name`: `dynamic_comment`
/// * `类型`: 1. 文字 2. 语音 3. 照片 4. 视频 5. 位置 7. 表情包 8. 红包 9. 转账 ...
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct DynamicCommentEntity {
    pub id: i64,                           // id
    pub uid: i64,                          // 用户ID
    pub dynamic_id: i64,                   // 动态 ID
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
    pub add_time: i64,                     // 添加时间(机器)
    pub upd_time: i64,                     // 更新时间(机器)
    pub created_at: Option<DateTime<Utc>>, // 创建时间(人类)
    pub updated_at: Option<DateTime<Utc>>, // 创建时间(人类)
    pub send_time: i64,
    pub sync_time: i64,
}

//////// END
