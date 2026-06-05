// cola_data/src/dynamic/entity/dynamic.rs  -- DYNAMIC - Entity - 动态表
// 2026/5/22 20:12 by wx: cestbon10080
// * --------
// * --------

////////

use chrono::{DateTime, Utc};
use futures::stream::Collect;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::{Uuid, uuid};

////////

/// # [ENTITY] - 动态 - 表
/// * table name: dynamic
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct DynamicEntity {
    // base
    pub id: i64,                      // 动态 ID
    pub send_id: Option<String>,      // 发送 ID
    pub user_id: i64,                 // 用户 ID
    pub r#type: i16,                  // 类型
    pub title: Option<String>,        // 标题
    pub original_url: Option<String>, // 原始 URL
    pub description: Option<String>,  // 描述
    pub thumb: Option<String>,        // 封面 （旧版）
    pub cover_url: Option<String>,    // 封面 url
    pub video_url: Option<String>,    // 视频 url
    pub voice_url: Option<String>,    // 语音 url
    pub href: Option<String>,         // 视频 url （旧版）
    pub length: Option<i64>,          // 时长（旧版PHP）
    pub address: Option<String>,      // 地址
    pub lat: Option<String>,          // 经度（兼容旧版）
    pub lng: Option<String>,          // 纬度（兼容旧版）
    pub geo: Option<String>,          // GEO（PG数据库原生扩展）
    pub views: i64,                   // 浏览量
    pub likes: i64,                   // 点赞量
    pub steps: i64,                   // 被踩数量
    pub collects: i64,                // 收藏量
    pub comments: i64,                // 评论数量
    pub shares: i64,                  // 分享数量
    pub is_ad: Option<i16>,           // 广告视频（兼容旧版）
    pub is_public: Option<i16>,       // 是否公开（兼容旧版）
    pub status: i16,                  // 状态
    pub music_id: Option<i64>,        // 音乐id
    pub goods_id: Option<i64>,        // 商品id
    pub add_time: i64,                // 时间戳
    pub pub_time: i64,                // 时间戳
    pub created_at: Option<String>,   // 创建于 UTC
    pub updated_at: Option<String>,   // 更新于 UTC
}

//////// END
