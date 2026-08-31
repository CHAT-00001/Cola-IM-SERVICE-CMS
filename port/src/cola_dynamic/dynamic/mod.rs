// port/src/cola_dynamic/dynamic/music.rs
// ⏩️ 端口 - ⏹ 可乐动态 -  动态 - 模块
// 2026/8/5 15:57 Created.

////////

use crate::cola_dynamic::dynamic::check::CheckPort;
use crate::cola_dynamic::dynamic::del::DelPort;
use crate::cola_dynamic::dynamic::get::GetPort;
use crate::cola_dynamic::dynamic::list::ListPort;
use crate::cola_dynamic::dynamic::manage::ManagePort;
use crate::cola_dynamic::dynamic::stat::StatPort;
use add::AddPort;
use std::sync::Arc;

////////

pub mod active; // 活跃
pub mod add; // 发布
pub mod check; // 检查
pub mod count; // 计数
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [DYNAMIC PORTS]
/// * `desc`: `⏹ 可乐动态 - 动态 Ports`
#[derive(Clone)]
pub struct DynamicPort {
    pub add: Arc<dyn AddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn CheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn DelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn GetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn ListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn ManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn StatPort + Send + Sync + 'static>, // 统计
}

//////// END
