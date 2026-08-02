// cola_data/src/new/entity/fs.rs  -- 数据中心 - VIDEO - entity - 分类
// 2026/4/24 18:24 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};

////////

/// # [ENTITY] - 短视频 - 分类表
/// * `pg schema`: `cola_video`
/// * `table name`: `video_category`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoClassifyEntity {
    pub id: i16,             // ID
    pub name: String,        // 视频分类名称
    pub name_en: String,     // 英文名称
    pub description: String, // 描述
    pub add_time: i64,       // 添加时间 - 机器
    pub upd_time: i64,       // 更新时间 - 机器
}

//////// END
