// repo_adapter/src/video/collect/music.rs
// 🔌 适配器 - ▶ 可乐视频 - 收藏 - 模块
// 2026/8/8 01:40 Created.

////////

use port::cola_video::collect::VideoCollectPort;
use std::sync::Arc;

////////

pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 计数

////////

/// # [BUILD] - 构建 COLLECT Port
/// * `desc`: 视频收藏夹端口构造器
pub fn build_video_collect_port() -> VideoCollectPort {
    VideoCollectPort {
        add: Arc::new(add::VideoCollectAddAdapter),
        check: Arc::new(check::VideoCollectCheckAdapter),
        del: Arc::new(del::VideoCollectDelAdapter),
        get: Arc::new(get::VideoCollectGetAdapter),
        list: Arc::new(list::VideoCollectListAdapter),
        manage: Arc::new(manage::CollectManageAdapter),
        stat: Arc::new(stat::VideoCollectStatAdapter),
    }
}
