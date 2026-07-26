// cola_data/src/three/entity/fs_media.rs  -- 数据中心 - DYNAMIC - entity - FS媒体
// 2026/7/26 10:10

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 动态 媒体
/// * `pg schema`: `cola_dynamic`
/// * `table name`: `three_fs_file`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct FsMediaEntity {
    pub id: i64,
    pub bucket_id: i64,             // FK → three_fs_bucket.id（存储桶配置）
    pub media_type: String,         // audio / video / image / livephoto ..
    pub name: String,               // 文件名（不含路径）
    pub size: i64,                  // 文件大小（字节）
    pub width: i16,                 // 帧宽度
    pub height: i16,                // 帧高度
    pub duration: i16,              // 时长(ms)
    pub file_type: String,          // 文件类型分类，如 "image", "video", "document"
    pub extension: String,          // 文件扩展名，如 ".pdf"
    pub etag: String,               // S3 ETag（文件哈希校验值）
    pub storage_class: String,      // 存储类型，如 "STANDARD", "GLACIER"
    pub version_id: Option<String>, // S3版本ID（启用版本控制时）
    pub metadata_json: Option<serde_json::Value>, // 用户自定义元数据（JSONB）
    pub tags: Option<Vec<String>>,  // 文件标签（用于分类检索）
    pub is_public: bool,            // 是否公开访问
    pub status: i16,                // 1正常 0已删除/回收站
    pub generated_at: DateTime<Utc>, // 文件生成时间（上传或创建时间）
    pub expires_at: Option<DateTime<Utc>>, // 过期时间（用于生命周期管理）
    pub last_accessed_at: Option<DateTime<Utc>>, // 最后访问时间
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

////////

/// # [COLUMNS] - 查询字段常量
pub const DYNAMIC_FS_MEDIA_COLUMNS: &str = r#"
    id, bucket_id, path, name, size, mime_type, file_type, extension,
    etag, storage_class, version_id, metadata_json, tags, is_public,
    status, generated_at, expires_at, last_accessed_at, created_at, updated_at
"#;

//////// END
