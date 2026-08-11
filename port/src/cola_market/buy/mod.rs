// port/src/cola_video/order/mod.rs
// ⏩️ 端口 - MARKET - 订单 - mod
// 2026/8/11 05:17 Created.

////////

use crate::cola_market::buy::add::GoodsBuyOrderAddPort;
use crate::cola_market::buy::check::GoodsBuyOrderCheckPort;
use crate::cola_market::buy::del::GoodsBuyOrderDelPort;
use crate::cola_market::buy::get::GoodsBuyOrderGetPort;
use crate::cola_market::buy::list::GoodsBuyOrderListPort;
use crate::cola_market::buy::manage::GoodsBuyOrderManagePort;
use crate::cola_market::buy::stat::GoodsBuyOrderStatPort;
use std::sync::Arc;

////////

pub mod add;
pub mod alive;
pub mod check;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;
pub mod order;
pub mod stat;

////////

/// # [BUY PORTS]
/// * `desc`: `▶ MARKET - 商品购买订单 Ports`
#[derive(Clone)]
pub struct GoodsBuyOrderPort {
    pub add: Arc<dyn GoodsBuyOrderAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn GoodsBuyOrderCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn GoodsBuyOrderDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn GoodsBuyOrderGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn GoodsBuyOrderListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn GoodsBuyOrderManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn GoodsBuyOrderStatPort + Send + Sync + 'static>, // 统计
}

//////// END
