// port/src/market/collect/mod.rs
// ⏩️ 端口 - MARKET - 商品收藏 - 模块
// 2026/6/10 08:23 Created.

////////

use crate::market::collect::add::GoodsCollectAddPort;
use crate::market::collect::check::GoodsCollectCheckPort;
use crate::market::collect::del::GoodsCollectDelPort;
use crate::market::collect::get::GoodsCollectGetPort;
use crate::market::collect::list::GoodsCollectListPort;
use crate::market::collect::manage::GoodsCollectManagePort;
use crate::market::collect::stat::GoodsCollectStatPort;
use std::sync::Arc;

////////

pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 软删除
pub mod get; // 获取
pub mod ids; // IDs
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [COLLECT PORTS]
/// * `desc`: `MARKET - 商品收藏 Ports`
#[derive(Clone)]
pub struct GoodsCollectPort {
    pub add: Arc<dyn GoodsCollectAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn GoodsCollectCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn GoodsCollectDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn GoodsCollectGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn GoodsCollectListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn GoodsCollectManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn GoodsCollectStatPort + Send + Sync + 'static>, // 统计
}

//////// END
