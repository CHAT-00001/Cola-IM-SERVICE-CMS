// cola_dynamic/port/hotlist/mod.rs
// 动态 - port - 上热门 - 模块
// 2026/8/5 15:53 Created.

////////

use crate::cola_dynamic::port::hotlist::add::HotlistAddPort;
use crate::cola_dynamic::port::hotlist::check::HotlistCheckPort;
use crate::cola_dynamic::port::hotlist::del::HotlistDelPort;
use crate::cola_dynamic::port::hotlist::get::HotlistGetPort;
use crate::cola_dynamic::port::hotlist::list::HotlistListPort;
use crate::cola_dynamic::port::hotlist::manage::HotlistManagePort;
use crate::cola_dynamic::port::hotlist::stat::HotlistStatPort;
use std::sync::Arc;

////////
mod active;
mod add;
mod alive;
mod check;
mod del;
mod get;
mod list;
mod manage;
mod stat;

////////

/// # [HOTLIST PORTS]
/// * `desc`: `动态上热门收藏 Ports`
#[derive(Clone)]
pub struct HotlistPort {
    pub add: Arc<dyn HotlistAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn HotlistCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn HotlistDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn HotlistGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn HotlistListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn HotlistManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn HotlistStatPort + Send + Sync + 'static>, // 统计
}

//////// END
