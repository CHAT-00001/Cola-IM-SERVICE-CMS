// cola_data/src/fs/command/bucket.rs
// 数据 - FS - 命令 - 存储桶
// 2026/7/27 14:39

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

////////

/// # [CMD] - 创建存储桶参数载荷
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateBucketCmd {
    pub _id: Option<String>,           // UUID v4
    pub app_id: Option<String>,        // 应用 ID
    pub bucket_key: String,            // 存储桶key
    pub cdn_domain: Option<String>,    // CDN 加速域名
    pub name: String,                  // 名称
    pub provider: i16,                 // 厂商
    pub s3_bucket: String,             // 存储桶
    pub s3_region: Option<String>,     //
    pub s3_endpoint: Option<String>,   //
    pub access_key: Option<String>,    // 公钥
    pub secret_key: Option<String>,    // 秘钥
    pub is_public: bool,               // 是否公开
    pub upload_policy: Option<String>, // 上传凭证
}

/// # [CMD] - 更新存储桶参数载荷
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBucketCmd {
    pub name: Option<String>,
    pub cdn_domain: Option<String>,
    pub provider: Option<i16>,
    pub s3_bucket: Option<String>,
    pub s3_region: Option<String>,
    pub s3_endpoint: Option<String>,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub is_public: Option<bool>,
    pub upload_policy: Option<String>,
    pub status: Option<i16>,
}

//////// END
