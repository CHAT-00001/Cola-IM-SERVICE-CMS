// cola_data/src/new/entity/new  -- 数据中心 - VIDEO - entity - 视频
// 2026/5/19 21:34

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

////////

/// # [ENTITY] - 短视频 实体表
/// * `pg schema`: `cola_video`
/// * `table name`: `new`
/// * 权限: 0. 关闭 1. 自己 2. 朋友 3. 关注 4. 粉丝 5. 所有人（拉黑除外）
#[derive(Debug, Clone, Default, FromRow)]
pub struct VideoEntity {
    pub id: i64,                                   // 视频 ID
    pub uid: i64,                                  // 用户 ID
    pub channel_id: i16,                           // 频道 ID
    pub title: String,                             // 标题
    pub title_at_uids: Option<Vec<i64>>,           // 标题@的IDs
    pub description: Option<String>,               // 描述
    pub desc_at_uids: Option<Vec<i64>>,            // 标题@的IDs
    pub thumb: String,                             // 封面
    pub thumb_s: Option<String>,                   // 封面w
    pub thumbnail: Option<String>,                 // 封面 2
    pub cover_url: Option<String>,                 // 封面
    pub href: String,                              // 视频url
    pub href_w: Option<String>,                    // 视频url w
    pub original_url: Option<String>,              // 视频原始url
    pub tags: Option<Vec<String>>,                 // 标签
    pub lat: Option<f64>,                          // 纬度
    pub lng: Option<f64>,                          // 经度
    pub duration: Option<String>,                  // 时长
    pub width: Option<i16>,                        // 帧宽度
    pub height: Option<i16>,                       // 帧高度
    pub fps: Option<i16>,                          // 帧数
    pub bit: Option<i16>,                          // 色深
    pub views: i32,                                // 浏览量
    pub likes: i32,                                // 点赞量
    pub dislike: i32,                              // 被踩数量
    pub collects: i32,                             // 收藏量
    pub comments: i32,                             // 评论数量
    pub danmakus: i32,                             // 弹幕数量
    pub recommends: i32,                           // 推荐数量
    pub shares: i32,                               // 分享数量
    pub is_public: Option<bool>,                   // 是否公开
    pub done_play_qty: Option<i32>,                // 完成播放数量
    pub is_del: i16,                               // 是否删除
    pub status: i16,                               // 状态 0. 下架 1. 正常 2. 草稿 3. 冻结
    pub music_id: Option<i64>,                     // 音乐id
    pub goods_id: Option<i64>,                     // 商品id
    pub visibility_perm: i16,                      // 可见权限
    pub comment_perm: i16,                         // 评论权限
    pub danmaku_perm: i16,                         // 弹幕权限
    pub collect_perm: i16,                         // 收藏权限
    pub download_perm: i16,                        // 下载权限
    pub addtime: i64,                              // 创建时间（兼容旧版PHP）
    pub sync_at: Option<i64>,                      // 同步时间
    pub created_at: Option<DateTime<Utc>>,         // 创建时间
    pub updated_at: Option<DateTime<Utc>>,         // 更新时间
    pub del_time: Option<i64>,                     // Unix 时间戳（秒）
    pub deleted_at: Option<chrono::NaiveDateTime>, // 删除时间 人类可读
}

//////// EDN
