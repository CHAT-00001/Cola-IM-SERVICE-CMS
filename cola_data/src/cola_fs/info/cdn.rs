// data/src/cola_fs/info/cdn.rs
// 🗄 数据 - ⏹ 可乐FS - info - CDN
// 2026/8/9 07:38 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// # [INFO] - Cdn信息
/// * `desc`: `安全简洁的信息`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdnInfo {
    pub id: i64,           // ID
    pub media_type: i16,   // 媒体类型
    pub fs: String,        // 文件名称
    pub thumbnail: String, // 缩略图
    pub url: String,       // 地址
    pub width: i16,        // 帧宽度
    pub height: i16,       // 帧高度
    pub fps: f32,          // 每秒帧数
    pub duration: i32,     // 时长
}

//////// END