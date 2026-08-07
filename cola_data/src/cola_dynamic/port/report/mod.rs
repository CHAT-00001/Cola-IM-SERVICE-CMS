// cola_dynamic/port/report/mod.rs
// 动态 - port - 举报 - 模块
// 2026/6/10 08:28

////////
use crate::cola_dynamic::port::report::add::ReportAddPort;
use crate::cola_dynamic::port::report::check::ReportCheckPort;
use crate::cola_dynamic::port::report::del::ReportDelPort;
use crate::cola_dynamic::port::report::get::ReportGetPort;
use crate::cola_dynamic::port::report::list::ReportListPort;
use crate::cola_dynamic::port::report::manage::ReportManagePort;
use crate::cola_dynamic::port::report::stat::ReportStatPort;
use std::sync::Arc;

////////
mod add;
mod alive;
mod check;
mod del;
mod get;
mod list;
mod manage;
mod stat;

////////

/// # [REPORT PORT]
/// * `desc`: `举报 Ports`
#[derive(Clone)]
pub struct ReportPort {
    pub add: Arc<dyn ReportAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn ReportCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn ReportDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn ReportGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn ReportListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn ReportManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn ReportStatPort + Send + Sync + 'static>, // 统计
}

//////// END
