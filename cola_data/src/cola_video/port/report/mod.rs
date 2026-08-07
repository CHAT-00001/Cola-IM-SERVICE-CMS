// cola_video/port/report/mod.rs
// 视频 - port - 举报 - 模块
// 2026/6/10 08:28

////////

use crate::cola_video::port::report::add::VideoReportAddPort;
use crate::cola_video::port::report::check::VideoReportCheckPort;
use crate::cola_video::port::report::del::VideoReportDelPort;
use crate::cola_video::port::report::get::VideoReportGetPort;
use crate::cola_video::port::report::list::VideoReportListPort;
use crate::cola_video::port::report::manage::ReportManagePort;
use crate::cola_video::port::report::stat::ReportStatPort;
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

/// # [REPORT PORTS] - 举报
/// * `desc`: `视频举报 Ports`
#[derive(Clone)]
pub struct ReportPort {
    pub add: Arc<dyn VideoReportAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn VideoReportCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn VideoReportDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn VideoReportGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn VideoReportListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn ReportManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn ReportStatPort + Send + Sync + 'static>,    // 统计
}

//////// END
