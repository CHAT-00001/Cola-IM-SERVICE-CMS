// repo_adapter/src/video/hotlist/music.rs
// 🔌 适配器 - VIDEO - hotlist - music
// 2026/8/8 Created.

////////

use port::cola_video::recommend::VideoRecommendPort;
use std::sync::Arc;

pub mod add; // 发布
pub mod alive; // 存活
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; //
pub mod stat; // 统计

////////

/// # [BUILD] - 构建 COLLECT Port
/// * `desc`: 视频推荐端口构造器
pub fn build_video_recommend_port() -> VideoRecommendPort {
    VideoRecommendPort {
        add: Arc::new(add::VideoRecommendAddAdapter),
        check: Arc::new(check::recommendcheckPortAdapter),
        del: Arc::new(del::VideoRecommendDelAdapter),
        get: Arc::new(get::VideoRecommendGetAdapter),
        list: Arc::new(list::RecommendListAdapter),
        manage: Arc::new(manage::VideoRrecommendManageAdapter),
        stat: Arc::new(stat::recommendstatPortAdapter),
    }
}

//////// END
