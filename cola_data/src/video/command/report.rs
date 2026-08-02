// cola_data/src/new/command/report.rs  -- 数据中心 - VIDEO - Command - 举报
// 2026/5/22 20:45 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};

////////

/// #  [COMMAND] - 视频 举报 命令
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VideoReportCommand {
    pub send_id: String,          // 发送 ID
    pub user_id: i64,             // 用户 ID
    pub album_id: Option<i64>,    // 专辑 ID
    pub video_id: i64,            // 视频 ID
    pub report_type: String,      // 举报类型
    pub report_tags: Vec<String>, // 举报标签
    pub remark: Option<String>,   // 备注
}

//////// END
