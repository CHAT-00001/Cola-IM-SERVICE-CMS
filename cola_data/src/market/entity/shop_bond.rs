//  cola_data/src/market/handler/shop_bond  -- 可乐数据中心 - MARKET - handler - 店铺保证金
// 2026/6/18 10:34

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 市场 - 店铺保证金
/// * table name: shop_bond
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ShopBondEntity {
    pub id: i64,                          // id
    pub uid: i64,                         // 操作者用户ID
    pub shop_id: i64,                     // 商店ID
    pub bond: i64,                        // 保证金
    pub status: i16,                      // 状态
    pub add_time: i32,                    // 添加时间（机器）
    pub upd_time: i32,                    // 更新时间（机器）
    pub create_at: Option<DateTime<Utc>>, // 创建时间（人类）
    pub update_at: Option<DateTime<Utc>>, // 更新时间（人类）
}
