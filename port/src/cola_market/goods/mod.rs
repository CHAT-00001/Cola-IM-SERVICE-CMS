// port/src/market/goods/mod.rs
// ⏩️ 端口 - MARKET - 商品 - mod
// 2026/8/5 15:57 Created.

////////

use crate::cola_market::goods::add::GoodsAddPort;
use crate::cola_market::goods::check::GoodsCheckPort;
use crate::cola_market::goods::delete::GoodsDeletePort;
use crate::cola_market::goods::get::GoodsGetPort;
use crate::cola_market::goods::list::GoodsListPort;
use crate::cola_market::goods::manage::GoodsManagePort;
use crate::cola_market::goods::stat::GoodsStatPort;
use std::sync::Arc;

////////

pub mod add; // 发布
pub mod check; // 检查
pub mod count; // 计数
pub mod delete; // 删除
pub mod feed; // 流
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [GOODS PORTS] - 商品
/// * `desc`: `MARKET - 商品 Ports`
#[derive(Clone)]
pub struct GoodsPort {
    pub add: Arc<dyn GoodsAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn GoodsCheckPort + Send + Sync + 'static>, // 检查
    pub delete: Arc<dyn GoodsDeletePort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn GoodsGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn GoodsListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn GoodsManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn GoodsStatPort + Send + Sync + 'static>, // 管理
}

//////// END
