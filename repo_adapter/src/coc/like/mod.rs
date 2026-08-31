// repo_adapter/src/video/like/mod.rs
// 🔌 适配器 - ▶ 视频 - 点赞 - music
// 2026/8/9 23:11 Created.

////////

use port::cola_video::danmaku::VideoDanmakuPort;
use port::cola_video::like::VideoLikePort;
use std::sync::Arc;

pub mod add;
pub mod check;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;
pub mod stat;

////////

/// # [BUILD] - 构建 COLLECT Port
/// * `desc`: 视频点赞端口构造器
pub fn build_video_like_port() -> VideoLikePort {
    VideoLikePort {
        add: Arc::new(add::VideoLikeAddAdapter),
        check: Arc::new(check::VideoLikeCheckAdapter),
        del: Arc::new(del::VideoLikeDelAdapter),
        get: Arc::new(get::VideoLikeGetAdapter),
        list: Arc::new(list::VideoLikeListAdapter),
        manage: Arc::new(manage::VideoLikeManageAdapter),
        stat: Arc::new(stat::LikeStatPortAdapter),
    }
}
