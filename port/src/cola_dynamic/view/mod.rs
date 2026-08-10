// port/src/cola_dynamic/view/mod.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 浏览 - mod
// 2026/8/5 14:32 Created.

////////

use crate::cola_dynamic::view::add::DynamicViewAddPort;
use crate::cola_dynamic::view::check::DynamicViewCheckPort;
use crate::cola_dynamic::view::del::DynamicViewDelPort;
use crate::cola_dynamic::view::get::DynamicViewGetPort;
use crate::cola_dynamic::view::list::DynamicViewListPort;
use crate::cola_dynamic::view::manage::DynamicViewManagePort;
use crate::cola_dynamic::view::stat::DynamicViewStatPort;
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

/// # [DYNAMIC VIEW PORTS]
/// * `desc`: `⏹ 可乐动态 - 动态浏览端口`
#[derive(Clone)]
pub struct DynamicViewPort {
    pub add: Arc<dyn DynamicViewAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn DynamicViewCheckPort + Send + Sync + 'static>, // 获取
    pub del: Arc<dyn DynamicViewDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn DynamicViewGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn DynamicViewListPort + Send + Sync + 'static>, // 获取
    pub manage: Arc<dyn DynamicViewManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn DynamicViewStatPort + Send + Sync + 'static>, // 统计
}

//////// END
