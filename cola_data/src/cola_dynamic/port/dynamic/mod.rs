// cola_dynamic/port/cola_dynamic/mod.rs
// 动态 - port - 动态 - 模块
// 2026/8/5 15:57 Created.

////////

use crate::cola_dynamic::port::dynamic::check::CheckPort;
use crate::cola_dynamic::port::dynamic::del::DelPort;
use crate::cola_dynamic::port::dynamic::get::GetPort;
use crate::cola_dynamic::port::dynamic::list::ListPort;
use crate::cola_dynamic::port::dynamic::manage::ManagePort;
use crate::cola_dynamic::port::dynamic::stat::StatPort;
use add::AddPort;
use std::sync::Arc;

////////

mod active; // 活跃
mod add; // 发布
mod check; // 检查
mod count; // 计数
mod del; // 删除
mod feed;
mod get; // 获取
mod list; // 列表
mod manage; // 管理
mod stat; // 统计

////////

/// # [DYNAMIC PORTS]
/// * `desc`: `动态 Ports`
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