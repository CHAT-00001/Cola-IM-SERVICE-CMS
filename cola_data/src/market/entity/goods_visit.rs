// cola_data/src/market/handler/goods_visit.rs  -- 可乐数据中心 - MARKET - handler - 商品浏览记录表
// 2026/6/18 10:16

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 市场 - 商品浏览记录表
/// * table name: goods_visit
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct GoodsVisitEntity {
    pub id: i64,                          // id
    pub uid: i64,                         // 操作者用户ID
    pub goods_id: i64,                    // 目标商品ID
    pub status: i16,                      // 状态
    pub add_time: i32,                    // 添加时间（机器）
    pub upd_time: i32,                    // 更新时间（机器）
    pub create_at: Option<DateTime<Utc>>, // 创建时间（人类）
    pub update_at: Option<DateTime<Utc>>, // 更新时间（人类）
}
