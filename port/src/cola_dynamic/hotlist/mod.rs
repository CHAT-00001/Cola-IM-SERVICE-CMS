// port/src/cola_dynamic/hotlist/mod.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 上热门 - mod
// 2026/8/5 15:53 Created.

////////

use crate::cola_dynamic::hotlist::add::HotlistAddPort;
use crate::cola_dynamic::hotlist::check::HotlistCheckPort;
use crate::cola_dynamic::hotlist::del::HotlistDelPort;
use crate::cola_dynamic::hotlist::get::HotlistGetPort;
use crate::cola_dynamic::hotlist::list::HotlistListPort;
use crate::cola_dynamic::hotlist::manage::HotlistManagePort;
use crate::cola_dynamic::hotlist::stat::HotlistStatPort;
use std::sync::Arc;

////////
pub mod active;
pub mod add;
pub mod alive;
pub mod check;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;
pub mod stat;

////////

/// # [DYNAMIC HOTLIST PORTS]
/// * `desc`: `⏹ 可乐动态 - 上热门 Ports`
#[derive(Clone)]
pub struct DynamicHotlistPort {
    pub add: Arc<dyn HotlistAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn HotlistCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn HotlistDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn HotlistGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn HotlistListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn HotlistManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn HotlistStatPort + Send + Sync + 'static>, // 统计
}

//////// END
