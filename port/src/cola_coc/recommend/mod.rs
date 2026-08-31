// port/sec/cola_video/hotlist/mod.rs
// ⏩️ 端口 - 可乐视频 - 推荐 - music
// 2026/8/5 15:48 Created.

////////

use crate::cola_video::recommend::add::VideoRecommendAddPort;
use crate::cola_video::recommend::check::VideoRecommendCheckPort;
use crate::cola_video::recommend::del::VideoRecommendDelPort;
use crate::cola_video::recommend::get::VideoRecommendGetPort;
use crate::cola_video::recommend::list::VideoRecommendListPort;
use crate::cola_video::recommend::manage::VideoRecommendManagePort;
use crate::cola_video::recommend::stat::VdieoRecommendStatPort;
use std::sync::Arc;

////////
pub mod active; // 活跃
pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod home; // 主页
pub mod list; // 前台列表
pub mod manage; // 管理
pub mod pool; // 推荐池
pub mod stat; // 统计

////////

/// # [VIDEO RECOMMEND PORT]
/// * `desc`: `▶ 可乐视频 - 推荐 Ports`
#[derive(Clone)]
pub struct VideoRecommendPort {
    pub add: Arc<dyn VideoRecommendAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn VideoRecommendCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn VideoRecommendDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn VideoRecommendGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn VideoRecommendListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn VideoRecommendManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn VdieoRecommendStatPort + Send + Sync + 'static>, // 状态
}

//////// END
