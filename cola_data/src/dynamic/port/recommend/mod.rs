// dynamic/prot/recommend/mod.rs
// 动态 - port - 推荐 - 模块
// 2026/8/5 15:48 Created.

////////

use crate::dynamic::port::recommend::add::AddPort;
use crate::dynamic::port::recommend::check::RecommendCheckPort;
use crate::dynamic::port::recommend::del::DelPort;
use crate::dynamic::port::recommend::get::GetPort;
use crate::dynamic::port::recommend::list::ListPort;
use crate::dynamic::port::recommend::manage::ManagePort;
use crate::dynamic::port::recommend::stat::StatPort;
use std::sync::Arc;

////////
mod active; // 活跃
mod add; // 发布
mod check; // 检查
mod del; // 删除
mod get; // 获取
mod home; // 主页
mod list; // 列表
mod manage; // 管理
mod pool; // 推荐池
mod stat; // 统计

////////

/// # [RECOMMEND PORTS]
/// * `desc`: `动态推荐 Ports`
#[derive(Clone)]
pub struct RecommendPort {
    pub add: Arc<dyn AddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn RecommendCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn DelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn GetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn ListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn ManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn StatPort + Send + Sync + 'static>, // 状态
}

//////// END