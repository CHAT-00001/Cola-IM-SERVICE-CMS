// store src/gift/info/gateway.rs -- 礼物信息 - 数据表映射
// 2026-02-06 20:41:46

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// ENTITY - 礼物道具实体 (软删除增强版)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GiftEntity {
    pub id: i64,                   // 礼物ID
    pub mark: Option<i32>,         // 标识：0普通，1热门，2守护
    pub r#type: Option<i32>,       // 类型：0普通，1豪华，2贴纸，3手绘
    pub sid: Option<i32>,          // 分类ID
    pub name: String,              // 中文名称
    pub name_en: Option<String>,   // 英文名称
    pub thumb: Option<String>,     // 封面图
    pub price: Option<i64>,        // 价格 (新版)
    pub need_coin: i32,            // 价格 (PHP旧版)
    pub gift_icon: Option<String>, // 图标
    pub list_order: i32,           // 序号
    pub swf_type: Option<i32>,     // 动画类型：0.gif, 1.svga
    pub swf: Option<String>,       // 动画链接
    pub swf_time: Option<f32>,     // 动画时长
    pub is_plat_gift: Option<i32>, // 全站礼物：0否 1是
    pub sticker_id: Option<i32>,   // 贴纸ID
    pub sort: Option<i32>,         // 排序 (默认9999)

    // --- 状态标识 ---
    pub is_del: i16, // 软删除标识：0.正常 1.已删除

    // --- 时间轴 ---
    pub add_time: Option<i32>,    // 添加时间戳 (PHP)
    pub update_time: Option<i32>, // 更新时间戳 (PHP)

    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub created_at: Option<DateTime<Utc>>, // 创建时间 (Rust)

    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>, // 更新时间 (Rust)
}
