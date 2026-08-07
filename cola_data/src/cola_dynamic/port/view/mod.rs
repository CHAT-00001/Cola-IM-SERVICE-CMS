// cola_dynamic/port/view/mod.rs
// 动态 - port - 浏览 - 模块
// 2026/8/5 14:32 Created.

////////

use crate::cola_dynamic::port::view::active::ViewActivePort;
use crate::cola_dynamic::port::view::add::ViewAddPort;
use crate::cola_dynamic::port::view::del::ViewDelPort;
use crate::cola_dynamic::port::view::get::ViewGetPort;
use crate::cola_dynamic::port::view::manage::ViewManagePort;
use crate::cola_dynamic::port::view::stat::ViewStatPort;
use std::sync::Arc;

////////
pub mod active; // 活跃
pub mod add; // 发布
pub mod del; // 删除
pub mod get; // 获取
pub mod manage; // 管理
pub mod stat; // 计数

////////

/// # [VIEW PORTS]
/// * `desc`: `动态浏览端口`
#[derive(Clone)]
pub struct ViewPort {
    pub active: Arc<dyn ViewActivePort + Send + Sync + 'static>,
    pub add: Arc<dyn ViewAddPort + Send + Sync + 'static>,
    pub del: Arc<dyn ViewDelPort + Send + Sync + 'static>,
    pub get: Arc<dyn ViewGetPort + Send + Sync + 'static>,
    pub manage: Arc<dyn ViewManagePort + Send + Sync + 'static>,
    pub stat: Arc<dyn ViewStatPort + Send + Sync + 'static>,
}

//////// END
