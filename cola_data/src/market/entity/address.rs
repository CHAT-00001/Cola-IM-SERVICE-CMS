// cola_data/src/market/entity/address.rs  -- 可乐数据中心 - MARKET - entity - 地址表
// 2026/6/18 10:37

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 市场 - 地址簿表
/// * table name: shop_address
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct AddressEntity {
    pub id: i64,                           // id
    pub uid: i64,                          // 操作者用户ID
    pub name: String,                      // 收件人名称
    pub country: String,                   // 国家/地区
    pub province: String,                  // 省/州
    pub city: String,                      // 城市
    pub area: String,                      // 区县
    pub address: String,                   // 详细地址(不可为空)
    pub area_code: String,                 // 地区号
    pub phone: String,                     // 电话号码
    pub is_default: i16,                   // 是否默认
    pub add_time: i32,                     // 添加时间（机器）
    pub upd_time: i32,                     // 更新时间（机器）
    pub create_at: Option<DateTime<Utc>>,  // 创建时间（人类）
    pub update_at: Option<DateTime<Utc>>,  // 更新时间（人类）
    pub id_del: i16,                       // 是否删除(默认0.否)
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间
}
