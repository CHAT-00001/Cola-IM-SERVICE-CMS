// data/src/cola_video/vo/video.rs
// 🗄 数据 - 可乐视频 - vo - 视频
// 2026/6/19 17:35

////////

use crate::cola_user::vo::user::UserVo;
use crate::cola_video::info::video::VideoInfo;
use serde::{Deserialize, Serialize};

////////

/// # [VO] - 视频 视图对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoVo {
    #[serde(flatten)]
    pub info: VideoInfo,
    pub user: UserVo,
    pub is_like: bool,
    pub is_collect: bool,
}
