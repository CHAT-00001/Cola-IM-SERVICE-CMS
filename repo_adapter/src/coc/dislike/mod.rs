// repo_adapter/src/video/dislike/music.rs
// 🔌 适配器 - VIDEO - 不喜欢 - music
// 2026/8/8 Created.

////////

use port::cola_video::dislike::VideoDislikePort;
use std::sync::Arc;

////////

pub mod add; // 发布
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 计数

////////

/// # [BUILD] - 构建 COLLECT Port
/// * `desc`: 视频不喜欢端口构造器
pub fn build_video_dislike_port() -> VideoDislikePort {
    VideoDislikePort {
        add: Arc::new(add::VideoDislikeAddAdapter),
        del: Arc::new(del::dislikedelPortAdapter),
        get: Arc::new(get::VideoDislikeGetAdapter),
        list: Arc::new(list::VideoDislikeListAdapter),
        manage: Arc::new(manage::VideoDislikeManageAdapter),
        stat: Arc::new(stat::VideoDislikeStatAdapter),
    }
}

//////// END
