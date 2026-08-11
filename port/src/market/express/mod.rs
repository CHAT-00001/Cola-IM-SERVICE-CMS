// port/src/market/express/mod.rs
// ⏩️ 端口 - MARKET - 快递公司 - mod
// 2026/8/5 14:32 Created.

////////

use crate::market::express::active::ExpressActivePort;
use crate::market::express::add::ExpressAddPort;
use crate::market::express::check::ExpressCheckPort;
use crate::market::express::delete::ExpressDeletePort;
use crate::market::express::get::ExpressGetPort;
use crate::market::express::list::ExpressListPort;
use crate::market::express::manage::ExpressManagePort;
use crate::market::express::stat::ExpressStatPort;
use std::sync::Arc;

////////
pub mod active; // 活跃
pub mod add; // 发布
pub mod check; // 检查
pub mod delete; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 计数

////////

/// # [VIEW PORTS]
/// * `desc`: `视频浏览服务端口`
#[derive(Clone)]
pub struct ExpressPort {
    pub active: Arc<dyn ExpressActivePort + Send + Sync + 'static>,
    pub add: Arc<dyn ExpressAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn ExpressCheckPort + Send + Sync + 'static>,
    pub delete: Arc<dyn ExpressDeletePort + Send + Sync + 'static>,
    pub get: Arc<dyn ExpressGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn ExpressListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn ExpressManagePort + Send + Sync + 'static>,
    pub stat: Arc<dyn ExpressStatPort + Send + Sync + 'static>,
}

//////// END
