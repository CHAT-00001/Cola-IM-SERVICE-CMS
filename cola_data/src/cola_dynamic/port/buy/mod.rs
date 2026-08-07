// cola_dynamic/port/buy/mod.rs
// 动态 - port - 购买 - 模块
// 2026/8/5 15:53 Created.

////////

use crate::cola_dynamic::port::buy::add::BuyAddPort;
use crate::cola_dynamic::port::buy::check::BuyCheckPort;
use crate::cola_dynamic::port::buy::del::BuyDelPort;
use crate::cola_dynamic::port::buy::get::BuyGetPort;
use crate::cola_dynamic::port::buy::list::BuyListPort;
use crate::cola_dynamic::port::buy::manage::BuyManagePort;
use crate::cola_dynamic::port::buy::stat::BuyStatPort;
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
mod order;
mod stat;

////////

/// # [BUY PORTS]
/// * `desc`: `动态购买 Ports`
#[derive(Clone)]
pub struct DynamicBuyPort {
    pub add: Arc<dyn BuyAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn BuyCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn BuyDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn BuyGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn BuyListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn BuyManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn BuyStatPort + Send + Sync + 'static>, // 统计
}

//////// END