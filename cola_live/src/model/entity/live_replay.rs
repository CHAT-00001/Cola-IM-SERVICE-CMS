// cola_data/src/live/entity/live_replay.rs
// LIVE - Entity - 直播回放记录
// 2026-05-22 by wx: cestbon10080

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 直播回放记录表
/// * table name: live_replay
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveReplayEntity {
    /// 主键ID（回放唯一标识）
    pub id: i64,

    /// 主播用户ID
    pub user_id: i64,

    /// 直播分片ID（1小时一个切片）
    pub node_id: i16,

    /// 直播标题/名称
    pub live_name: String,

    /// 直播分类ID
    pub category_id: i64,

    /// 省份编码
    pub province: i16,

    /// 城市编码
    pub city: i16,

    /// 本场最高点赞数
    pub max_like: i32,

    /// 峰值在线观看人数
    pub max_watching: i32,

    /// 回放地址（m3u8 / mp4）
    pub replay_url: String,

    /// 回放状态
    /// 0: 未生成 | 1: 生成中 | 2: 已完成 | 3: 已删除 | 4: 转码失败
    pub status: i8,

    /// 直播开始时间（时间戳）
    pub start_time: i64,

    /// 直播结束时间（时间戳）
    pub end_time: Option<i64>,

    /// 创建时间
    pub created_at: i64,

    /// 更新时间
    pub updated_at: i64,
}