// port/src/basic/app/mod.rs -- PORT - BASIC - APP - mod
// 2026/8/5 15:11 Created.

////////

use crate::basic::app::add::AppAddPort;
use crate::basic::app::check::AppCheckPort;
use crate::basic::app::del::AppDelPort;
use crate::basic::app::get::AppGetPort;
use crate::basic::app::list::AppListPort;
use crate::basic::app::manage::AppManagePort;
use crate::basic::app::stat::AppStatPort;
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

/// # [S3 BUCKET PORT]
/// * `desc`: `S3 FS - 存储桶 Ports`
#[derive(Clone)]
pub struct AppPort {
    pub add: Arc<dyn AppAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn AppCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn AppDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn AppGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn AppListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn AppManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn AppStatPort + Send + Sync + 'static>, // 统计
}

//////// END
