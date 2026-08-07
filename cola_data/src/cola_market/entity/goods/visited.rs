// cola_market/entity/goods/visited.rs
// 数据中心 - MARKET - entity - 商品 - 浏览记录表
// 2026/8/3 23:03 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 市场 - 商品 - 浏览记录表
/// * `pg schema`: `cola_market`
/// * `table name`: `goods_visit`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct GoodsVisitEntity {
    pub id: i64,                          // ID (自增 / 雪花)
    pub uuid: Option<String>,             // uuid v4
    pub uid: i64,                         // 用户ID
    pub goods_id: i64,                    // 目标商品ID
    pub status: i16,                      // 状态
    pub add_time: i32,                    // 添加时间（兼容旧版PHP）
    pub upd_time: i32,                    // 更新时间（兼容旧版PHP）
    pub create_at: Option<DateTime<Utc>>, // 创建时间
    pub update_at: Option<DateTime<Utc>>, // 更新时间
}
