// repo_adapter/src/cola_video/view/music.rs
// 🔌 插头 - 可乐视频 - 浏览 - 模块
// 2026/8/6 19:30 Created.

////////

use crate::video::danmaku::check;
use port::cola_video::danmaku::VideoDanmakuPort;
use port::cola_video::view::VideoViewPort;
use std::sync::Arc;

pub mod active; // 存活
pub mod add; //发布
pub mod del; // 删除
pub mod get; // 获取IDs
pub mod list; // 浏览列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [BUILD] - 构建 COLLECT Port
/// * `desc`: 视频浏览端口构造器
pub fn build_video_view_port() -> VideoViewPort {
    VideoViewPort {
        active: Arc::new(active::VideoViewActiveAdapter),
        add: Arc::new(add::VideoViewAddAdapter),
        del: Arc::new(del::VideoViewDeleteAdapter),
        get: Arc::new(get::VideoViewGetAdapter),
        list: Arc::new(list::ViewListService),
        manage: Arc::new(manage::VideoViewManageAdapter),
        stat: Arc::new(stat::VideoViewStatAdapter),
    }
}

//////// END
