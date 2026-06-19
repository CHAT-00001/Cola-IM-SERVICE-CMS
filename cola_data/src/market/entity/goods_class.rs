// cola_data/src/market/handler/goods_class.rs  -- 可乐数据中心 - MARKET - handler - 商品分类
// 2026/6/18 10:59

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 市场 - 商品分类表
/// * table name: shop_class
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ShopClassEntity {
    pub gc_id: i64,                       // id
    pub gc_name: i64,                     // 中文名称
    pub gc_name_en: String,               // 英文名称
    pub gc_parentid: i16,                 // 父级分类ID
    pub gc_one_id: i16,                   // 一级分类ID
    pub gc_sort: i16,                     // 老版排序
    pub gc_isshow: i16,                   // 是否展示 0 否 1 是
    pub sort: i16,                        // 排序 默认9999
    pub gc_grade: String,                 // 等级
    pub gc_icon: String,                  // 图标
    pub gc_isvirtual: i16,                // 是否虚拟:
    pub visibility: i32,                  // 可见度
    pub add_time: i32,                    // 添加时间（机器）
    pub upd_time: i32,                    // 更新时间（机器）
    pub create_at: Option<DateTime<Utc>>, // 创建时间（人类）
    pub update_at: Option<DateTime<Utc>>, // 更新时间（人类）
}
