// cola_video/src/utils/mod.rs -- VIDEO - utils - mod
// 2026/9/29 04:22 Created.

////////

/// # [UTILS] - 参数 ID 检查工具
pub struct CheckId;

impl CheckId {
    ////////

    /// # 1. [UTILS] - 检查视频 ID
    /// * `desc`: `视频 ID 必须为正整数`
    pub fn video_id(video_id: i64) -> bool {
        video_id > 0
    }
}

//////// END
