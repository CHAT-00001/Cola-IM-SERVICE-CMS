// entity/currency.rs  -- entity  币种
// 2026/6/26 01:30

////////

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde_json::Value as JsonValue;

////////

/// # [ENTITY] - 货币种类表
/// * `table name`: `wallet_currencies`
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WalletCurrency {
    pub id: i16,                     // 货币ID
    pub code: String,                // 货币代码：USD, CNY, JPY, KRW, HKD, GBP, SGD, EUR, COIN, POINT
    pub name: String,                // 货币名称
    pub name_en: String,             // 英文名称
    pub symbol: String,              // 货币符号
    pub symbol_native: String,       // 本地货币符号
    pub decimal_places: i16,         // 小数位数（USD/CNY=2, JPY=0, COIN=0）
    pub usd_rate: Decimal,           // 对美元汇率（1 USD = ? 该货币）
    pub is_base: bool,               // 是否是基准货币（USD）
    pub is_crypto: bool,             // 是否是虚拟货币
    pub is_point: bool,              // 是否是积分
    pub is_enabled: bool,            // 是否启用
    pub sort_order: i16,             // 排序权重
    pub icon_url: Option<String>,    // 图标URL
    pub extra: JsonValue,            // 扩展信息 (JSONB)
    pub created_at: DateTime<Utc>,   // 创建时间
    pub updated_at: DateTime<Utc>,   // 更新时间
}