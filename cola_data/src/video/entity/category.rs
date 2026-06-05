// cola_video/src/video/entity/category.rs  -- VIDEO - Entity - 分类
// 2026/4/24 18:24 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};

/// # [ENTITY] - 短视频 - 分类表
/// * table name: video_category
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoClassifyEntity {
    pub name: String, // 视频分类名称
    pub name_en: String,
    pub description: String,
}

//////// END