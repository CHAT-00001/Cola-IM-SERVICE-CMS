// cola_data/src/video/vo/video.rs  -- VIDEO - VO - 视频
// 2026/6/19 17:35

////////

use crate::user::vo::user::UserVo;
use crate::video::info::video::VideoInfo;
use serde::{Deserialize, Serialize};

////////

/// # [VO] - 短视频 视图模型 结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoVo {
    #[serde(flatten)]
    pub info: VideoInfo,
    pub user: UserVo,
    pub is_like: bool,
    pub is_collect: bool,
}
