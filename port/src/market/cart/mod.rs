// port/src/market/cart/mod.rs
// ⏩️ 端口 - MARKET - 购物车 - mod
// 2026/8/5 15:57 Created.

////////

use crate::market::cart::add::CartAddPort;
use crate::market::cart::check::CartCheckPort;
use crate::market::cart::del::CartDeletePort;
use crate::market::cart::get::CartGetPort;
use crate::market::cart::list::CartListPort;
use crate::market::cart::manage::CartManagePort;
use crate::market::cart::stat::CartStatPort;
use std::sync::Arc;
////////

pub mod add; // 发布
pub mod check; // 检查
pub mod count; // 计数
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [CART PORTS] - 购物车
/// * `desc`: `MARKET - 购物车 Ports`
#[derive(Clone)]
pub struct CartPort {
    pub add: Arc<dyn CartAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn CartCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn CartDeletePort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn CartGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn CartListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn CartManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn CartStatPort + Send + Sync + 'static>, // 管理
}

//////// END
