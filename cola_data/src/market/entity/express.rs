// cola_data/src/market/handler/express  -- 可乐数据中心 - MARKET - handler - 快递
// 2026/6/18 10:53

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 市场 - 快递公司表
/// * table name: shop_express
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ExpressEntity {
    pub id: i64,                          // id
    pub express_name: String,             // 名称
    pub name_en: String,                  // 英文名称
    pub express_phone: String,            // 电话
    pub express_thumb: String,            // 封面
    pub express_status: i16,              // 状态: 0. 禁用  1. 启用
    pub express_code: String,             // 代码
    pub sort: i16,                        // 排序,默认9999
    pub add_time: i32,                    // 添加时间（机器）
    pub upd_time: i32,                    // 更新时间（机器）
    pub create_at: Option<DateTime<Utc>>, // 创建时间（人类）
    pub update_at: Option<DateTime<Utc>>, // 更新时间（人类）
    pub list_order: i32,                  // 老版排序
}
