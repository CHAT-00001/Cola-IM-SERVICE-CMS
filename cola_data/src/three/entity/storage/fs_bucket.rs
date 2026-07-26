// cola_data/src/three/entity/storage/fs_bucket.rs  -- 数据中心 - 第三方 - entity - storage - 存储桶
// 2026/7/24 02:54

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 第三方服务配置
/// * `pg schema`: `cola_three`
/// * `table name`: `three_fs`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct FileBucketEntity {
    pub id: i64,
    pub type_id: i64,                           // FK → three_type.id（业务/分类归属）
    pub vendor_id: i64,                         // FK → three_vendor.id（厂商归属，如 AWS、阿里云等）
    pub name: String,                           // 服务名称，如 "视频主CDN"
    pub bucket: String,                         // 存储桶名
    pub access_key: String,                     // 访问密钥 (AK)
    #[serde(skip_serializing)]                  // 安全建议：防止 API 意外将密钥序列化外泄
    pub secret_key: String,                     // 密钥 (SK，建议加密存储)
    pub endpoint: String,                       // 接入端点，如 "s3.us-east-1.amazonaws.com"
    pub region: String,                         // 区域，如 "us-east-1"
    pub config_json: Option<serde_json::Value>, // 厂商特有配置（JSONB，如 path_style 等扩展）
    pub remark: Option<String>,                 // 备注
    pub status: i16,                            // 1启用 0禁用
    pub created_at: Option<DateTime<Utc>>,      // 创建时间
    pub updated_at: Option<DateTime<Utc>>,      // 更新时间
}

////////

/// # [COLUMNS] - 查询字段常量
pub const THREE_FS_BUCKET_COLUMNS: &str = r#"
    id, type_id, vendor_id, name, bucket, access_key, secret_key,
    endpoint, region, config_json, remark, status, created_at, updated_at
"#;

//////// END