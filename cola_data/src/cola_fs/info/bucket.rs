// data/src/cola_fs/info/bucket.rs
// 数据 - FS - info - 存储桶
// 2026/8/9 07:38 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 存储桶信息
/// * `desc`: `存储桶安全的信息`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketInfo {
    pub id: i64,               // ID (自增ID)
    pub _id: Option<String>,   // UUID v4
    pub vendor: i16,           // 运营商
    pub fs: String,            // 文件名称
    pub thumbnail: String,     // 缩略图
    pub url: String,           // 地址
    pub nama: i16,             // 名称
    pub nama_zh: i16,          // 中文名称
    pub size: u64,             // 体积占用 (GB)
    pub fs_total: Option<u64>, // 文件数量
}

//////// END
