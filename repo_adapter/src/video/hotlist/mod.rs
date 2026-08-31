// repo_adapter/src/video/hotlist/mod.rs -- 🔌 适配器 - VIDEO - 上热门 - mod
// 2026/8/9 22:41 Created.

////////

use port::cola_video::hotlist::VideoHotlistPort;
use std::sync::Arc;

////////

pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [BUILD] - 构建 COLLECT Port
/// * `desc`: 视频上热门端口构造器
pub fn build_video_hotlist_port() -> VideoHotlistPort {
    VideoHotlistPort {
        add: Arc::new(add::hotlistaddPortAdapter),
        check: Arc::new(check::VideoHotlistCheckAdapter),
        del: Arc::new(del::VideoHotlistDelAdapter),
        get: Arc::new(get::VideoHotlistGetAdapter),
        list: Arc::new(list::VideoHotlistListAdapter),
        manage: Arc::new(manage::VideoHotlistManageAdapter),
        stat: Arc::new(stat::VideoHotlistStatAdapter),
    }
}

//////// END
