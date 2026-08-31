// port/src/cola_video/order/music.rs
// ⏩️ 端口 - MARKET - 订单 - music
// 2026/8/11 05:17 Created.

////////

use crate::market::buy::del::GoodsBuyDelPort;

use crate::market::buy::add::GoodsBuyAddPort;
use crate::market::buy::check::GoodsBuyCheckPort;
use crate::market::buy::get::GoodsBuyGetPort;
use crate::market::buy::list::GoodsBuyListPort;
use crate::market::buy::manage::GoodsBuyManagePort;
use crate::market::buy::stat::GoodsBuyStatPort;
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
/// * `desc`: `MARKET - 商品购买订单 Ports`
#[derive(Clone)]
pub struct GoodsBuyPort {
    pub add: Arc<dyn GoodsBuyAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn GoodsBuyCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn GoodsBuyDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn GoodsBuyGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn GoodsBuyListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn GoodsBuyManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn GoodsBuyStatPort + Send + Sync + 'static>, // 统计
}

//////// END
