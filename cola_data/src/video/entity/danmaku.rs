// cola_video/src/live/gate_grpc/danmark.rs  -- 弹幕 - 实体
// 2026-01-16 09:37:10

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::{Uuid, uuid};

////////

/// # [ENTITY] - 视频弹幕实体
/// * table name: video_danmaku
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct DanmakuEntity {
    pub id: i64,                 // 弹幕 ID
    pub send_id: Option<String>, // 发送 ID
    pub user_id: i64,            // 用户 ID
    pub video_id: i64,           // 视频 ID
    pub channel_id: i16,         // 通道
    pub content: String,         // // 内容
    pub likes: i32,              // 点赞数量
    pub dislikes: i32,           // 不喜欢数量
    pub visibility: i16,         // 可见性
    pub color: Option<String>,   // 颜色（默认 #FFFFFF）
    pub mode: i16,               // 模式: 0. 固定 1. 滚动
    pub play_time: i32,          // 播放器规定时间（ms）
    pub duration: i16,           // 弹幕留存时间（固定字幕时生效）
    pub send_time: i64,          // 发送时间（客户端生成）
    pub sync_time: i64,          // 服务器同步时间
    pub status: i16,
}

//////// END
