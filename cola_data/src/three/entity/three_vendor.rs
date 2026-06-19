// cola_data/src/three/entity/three_vendor.rs  -- THREE - 厂商实体
// 2026/6/18

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

//////

/// # [ENTITY] - 第三方厂商
/// * table name: three_vendor
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ThreeVendorEntity {
    pub id: i64,
    pub code: String,           // private / tencent / qiniu / aliyun / rongcloud / aws / google
    pub name: String,           // 中文名
    pub sort: i16,
    pub status: i16,            // 1启用 0禁用
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// # 查询字段常量
pub const THREE_VENDOR_COLUMNS: &str = r#"
    id, code, name, sort, status, created_at, updated_at
"#;
