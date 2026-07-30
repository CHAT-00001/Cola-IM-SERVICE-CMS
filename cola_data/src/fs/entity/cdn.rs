// cola_data/src/fs/entity/cdn.rs  -- FS - 服务配置实体
// 2026/7/27 14:38

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - CDN配置
/// * `pg schema`: `cola_fs`
/// * `table name`: `cdn`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct CdnEntity {
    pub id: i64,
    pub type_id: i64,           // FK → three_type.id
    pub vendor_id: i64,         // FK → three_vendor.id
    pub name: String,           // 服务名称，如"视频主CDN"
    pub bucket: String,         // 存储桶名
    pub access_key: String,     // 访问密钥
    pub secret_key: String,     // 密钥（加密存储）
    pub endpoint: String,       // 接入端点
    pub region: String,         // 区域
    pub config_json: Option<serde_json::Value>, // 厂商特有配置（JSONB）
    pub remark: Option<String>, // 备注
    pub status: i16,            // 1启用 0禁用
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

////////

/// # [COLUMNS] - 查询字段常量
pub const THREE_FS_CDN_COLUMNS: &str = r#"
    id, type_id, vendor_id, name, bucket, access_key, secret_key,
    endpoint, region, config_json, remark, status, created_at, updated_at
"#;


//////// END