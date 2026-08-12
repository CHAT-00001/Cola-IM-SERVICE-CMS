// port/src/cola_video/video/mod.rs
// ⏩️ 端口 - MARKET - 商店 - mod
// 2026/8/5 15:57 Created.

////////

use crate::market::shop::add::ShopAddPort;
use crate::market::shop::appy::ShopAppyPort;
use crate::market::shop::check::ShopCheckPort;
use crate::market::shop::del::ShopDeletePort;
use crate::market::shop::get::ShopGetPort;
use crate::market::shop::list::ShopListPort;
use crate::market::shop::manage::ShopManagePort;
use crate::market::shop::stat::ShopStatPort;
use std::sync::Arc;
////////

pub mod add; // 发布
pub mod appy;
pub mod check; // 检查
pub mod count; // 计数
pub mod del; // 删除
pub mod feed;
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat;
// 统计

////////

/// # [SHOP PORTS] - 商店
/// * `desc`: `▶ MARKET - 商店 Ports`
#[derive(Clone)]
pub struct ShopPort {
    pub add: Arc<dyn ShopAddPort + Send + Sync + 'static>, // 发布
    pub appy: Arc<dyn ShopAppyPort + Send + Sync + 'static>, // 申请
    pub check: Arc<dyn ShopCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn ShopDeletePort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn ShopGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn ShopListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn ShopManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn ShopStatPort + Send + Sync + 'static>, // 管理
}

//////// END
