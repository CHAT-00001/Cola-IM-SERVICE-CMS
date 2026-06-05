// cola_video/src/vo/vo/video.rs  -- VIDEO - Model - Vo - 短视频
// 2026/5/21 03:47

////////

use serde::Serialize;
use cola_data::app::page::PageInfo;
use cola_music::model::info::music::MusicInfo;
use cola_user::model::info::user::UserInfo;
//
use crate::model::info::video::VideoInfo;

////////

/// # [MODEL] - 视频视图模型
#[derive(Debug, Serialize, Clone)]
pub struct VideoVo {
    // 🌟 直接扁平化展开（Flatten），前端甚至感觉不到结构体拆分了！
    // 访问时依然是 video.id, video.title，不需要 video.info.title
    #[serde(flatten)]
    pub video: VideoInfo,

    // 动态组装的关联数据
    pub user_info: UserInfo,
    pub music_info: MusicInfo,

    // 针对当前登录用户的动态交互显示
    pub is_visited: bool,
    pub is_like: bool,
    pub is_collect: bool,
    pub is_push: bool,
}

/// # [BUILD] - 构建视频视图
impl VideoVo {
    /// 在 BIZ 层进行大聚合的构造函数
    pub fn combine(
        video: VideoInfo,
        user: UserInfo,
        music: MusicInfo,
        // 也可以选择把交互状态作为参数传进来，或者提供 set 方法
    ) -> Self {
        Self {
            video,
            user_info: user,
            music_info: music,
            is_visited: false,
            is_like: false,
            is_collect: false,
            is_push: false,
        }
    }
}

impl Default for VideoVo {
    fn default() -> Self {
        Self {
            video: VideoInfo::empty(),
            user_info: UserInfo::default(),
            music_info: MusicInfo::default(),
            is_visited: false,
            is_like: false,
            is_collect: false,
            is_push: false,
        }
    }
}

/// # [RESPONSE] - 单视频响应
#[derive(Debug, Serialize)]
pub struct VideoSingleResponse {
    pub info: VideoVo, // 吐给前端完美的、组装好的 VO 列表
}

/// # [RESPONSE] - 多视频响应
#[derive(Debug, Serialize)]
pub struct VideoListResponse {
    pub list: Vec<VideoVo>, // 吐给前端完美的、组装好的 VO 列表
    pub page_info: PageInfo,
}

//////// END
