// cola_dynamic/port/share/mod.rs
// 动态 - port - 分享 - 模块
// 2026/8/5 15:15 Created.

////////

use crate::cola_dynamic::port::share::add::AddPort;
use crate::cola_dynamic::port::share::check::CheckPort;
use crate::cola_dynamic::port::share::del::DelPort;
use crate::cola_dynamic::port::share::get::GetPort;
use crate::cola_dynamic::port::share::list::ListPort;
use crate::cola_dynamic::port::share::manage::ManagePort;
use crate::cola_dynamic::port::share::stat::StatPort;
use std::sync::Arc;

////////
mod add;
mod check;
mod del;
mod get;
mod list;
mod manage;
mod stat;

////////

/// # [SHARE PORTS]
/// * `desc`: `动态分享 Ports`
#[derive(Clone)]
pub struct DynamicSharePort {
    pub add: Arc<dyn AddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn CheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn DelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn GetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn ListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn ManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn StatPort + Send + Sync + 'static>, // 状态
}

//////// END
