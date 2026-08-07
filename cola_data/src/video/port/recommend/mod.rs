// video/prot/recommend/mod.rs
// 视频 - port - 推荐 - 模块
// 2026/8/5 15:48 Created.

use crate::video::port::recommend::add::AddPort;
use crate::video::port::recommend::check::RecommendCheckPort;
use crate::video::port::recommend::del::VideoRecommendDelPort;
use crate::video::port::recommend::get::VideoRecommendGetPort;
use crate::video::port::recommend::list::VideoRecommendListPort;
use crate::video::port::recommend::manage::VideoRecommendManagePort;
use crate::video::port::recommend::stat::VdieoRecommendStatPort;
use std::sync::Arc;

////////
mod active; // 活跃
mod add; // 发布
mod check; // 检查
mod del;
mod get; // 获取
mod home; // 主页
mod list;
mod manage; // 管理
mod pool; // 推荐池
mod stat;
// 统计

////////

/// # [RECOMMEND PORT]
/// * `desc`: `推荐 Ports`
#[derive(Clone)]
pub struct RecommendPort {
    pub add: Arc<dyn AddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn RecommendCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn VideoRecommendDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn VideoRecommendGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn VideoRecommendListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn VideoRecommendManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn VdieoRecommendStatPort + Send + Sync + 'static>, // 状态
}

//////// END
