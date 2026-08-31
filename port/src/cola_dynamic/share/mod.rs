// port/src/cola_dynamic/share/music.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 分享 - 模块
// 2026/8/5 15:15 Created.

////////

use crate::cola_dynamic::share::add::DynamicShareAddPort;
use crate::cola_dynamic::share::check::DynamicShareCheckPort;
use crate::cola_dynamic::share::del::DynamicShareDelPort;
use crate::cola_dynamic::share::get::DynamicShareGetPort;
use crate::cola_dynamic::share::list::DynamicShareListPort;
use crate::cola_dynamic::share::manage::DynamicShareManagePort;
use crate::cola_dynamic::share::stat::DynamicShareStatPort;
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

/// # [DYNAMIC SHARE PORTS]
/// * `desc`: `⏹ 可乐动态 - 动态分享 Ports`
#[derive(Clone)]
pub struct DynamicSharePort {
    pub add: Arc<dyn DynamicShareAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn DynamicShareCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn DynamicShareDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn DynamicShareGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn DynamicShareListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn DynamicShareManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn DynamicShareStatPort + Send + Sync + 'static>, // 状态
}

//////// END
