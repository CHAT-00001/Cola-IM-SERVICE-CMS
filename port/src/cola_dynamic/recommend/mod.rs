// port/src/cola_dynamic/recommend/mod.rs
// ⏩️ 端口 - 可乐动态 - 推荐 - mod
// 2026/8/5 15:48 Created.

////////

use add::DynamicRecommendAddPort;
use check::RecommendCheckPort;
use del::DynamicRecommendDelPort;
use get::DynamicRecommendGetPort;
use list::DynamicRecommendListPort;
use manage::DynamicRecommendManagePort;
use stat::DynamicRecommendStatPort;
use std::sync::Arc;

////////
mod active; // 活跃
pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod home; // 主页
pub mod list; // 列表
mod manage; // 管理
pub mod pool; // 推荐池
pub mod stat; // 统计

////////

/// # [DYNAMIC RECOMMEND PORTS]
/// * `desc`: `⏹ 可乐动态 - 动态推荐 Ports`
#[derive(Clone)]
pub struct DynamicRecommendPort {
    pub add: Arc<dyn DynamicRecommendAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn RecommendCheckPort + Send + Sync + 'static>,    // 检查
    pub del: Arc<dyn DynamicRecommendDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn DynamicRecommendGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn DynamicRecommendListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn DynamicRecommendManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn DynamicRecommendStatPort + Send + Sync + 'static>, // 状态
}

//////// END
