// repo_adapter/src/video/share/mod.rs
// 🔌 适配器 - ▶ 视频 - 分享 - mod
// 2026/8/10 02:54 Created.

////////

use std::sync::Arc;
use port::cola_video::danmaku::VideoDanmakuPort;
use port::cola_video::share::VideoSharePort;

pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计


////////

/// # [BUILD] - 构建 COLLECT Port
/// * `desc`: 视频分享端口构造器
pub fn build_video_share_port() -> VideoSharePort {
    VideoSharePort {
        add: Arc::new(add::VideoShareAddAdapter),
        check: Arc::new(check::VideoShareCheckAdapter),
        del: Arc::new(del::VideoShareDelAdapter),
        get: Arc::new(get::VideShareGetAdapter),
        list: Arc::new(list::VideoShareListAdapter),
        manage: Arc::new(manage::VideoShareManageAdapter),
        stat: Arc::new(stat::VideoShareStatAdapter),
    }
}

//////// END