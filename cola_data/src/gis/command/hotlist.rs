// cola_data/src/video/dynamic/hotlist.rs  -- VIDEO - Command - 涓婄儹闂ㄥ懡浠?
// 2026/5/22 20:45 by wx: cestbon10080
// * --------
// * --------

use serde::{Deserialize, Serialize};

////////

/// #  [COMMAND] - 鐭棰?- 涓婄儹闂ㄥ懡浠?
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HotlistCommand {
    pub send_id: String,        // 鍙戦€?ID
    pub user_id: i64,           // 鐢ㄦ埛 ID
    pub album_id: Option<i64>,  // 涓撹緫 ID
    pub video_id: i64,          // 瑙嗛 ID
    pub remark: Option<String>, // 澶囨敞
    pub qty: i32,               // 鏇濆厜閲?
    pub time_zone: String,      // 鏃堕棿鍖洪棿
    pub older: Option<String>,  // 璁㈠崟 ID
}

