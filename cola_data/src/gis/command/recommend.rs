// cola_data/src/video/dynamic/recommend.rs  -- VIDEO - Command - 鎺ㄨ崘鍛戒护
// 2026/5/22 20:45 by wx: cestbon10080
// * --------
// * --------

use serde::{Deserialize, Serialize};

////////

/// #  [COMMAND] - 鐭棰?- 鎺ㄨ崘鍛戒护
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecommendCommand {
    pub send_id: String,        // 鍙戦€?ID
    pub user_id: i64,           // 鐢ㄦ埛 ID
    pub album_id: Option<i64>,  // 涓撹緫 ID
    pub video_id: i64,          // 瑙嗛 ID
    pub remark: Option<String>, // 澶囨敞
}

