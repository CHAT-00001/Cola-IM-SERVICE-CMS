// data/src/fs/command/media.rs
// 数据 - FS - 命令 - 媒体
// 2026/8/14 11:51 Created.

////////

use serde::{Deserialize, Serialize};

/// # [CMD] - 创建媒体资源参数载荷
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMediaCmd {
    pub _id: Option<String>,
    pub app_id: Option<String>,
    pub media_type: i16,
    pub cover_file_id: Option<i64>,
    pub main_file_id: Option<i64>,
    pub aux_file_id: Option<i64>,
    pub hls_playlist_url: Option<String>,
    pub variants_meta: Option<String>,
    pub duration: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
}