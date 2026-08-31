// repo_adapter/src/cola_video/cola_video/mod.rs
// 🔌 适配器 - VIDEO - 视频 - 模块
// 2026/8/6 19:19 Created.

////////

use port::cola_video::danmaku::VideoDanmakuPort;
use port::cola_video::video::VideoPort;
use std::sync::Arc;

pub mod add; // 发布
pub mod alive; // 存活
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [BUILD] - 构建 COLLECT Port
/// * `desc`: 视频内容端口构造器
pub fn build_video_video_port() -> VideoPort {
    VideoPort {
        add: Arc::new(add::VideoAddAdapter),
        check: Arc::new(check::VideoCheckAdapter),
        del: Arc::new(del::VideoDelAdapter),
        get: Arc::new(get::VideoGetAdapter),
        list: Arc::new(list::VideoListAdapter),
        manage: Arc::new(manage::VideoManageAdapter),
        stat: Arc::new(stat::VideoStatAdapter),
    }
}

//////// END
