// port/src/cola_video/view/mod.rs
// ⏩️ 端口 - ▶ 可乐视频 -  浏览 - 模块
// 2026/8/5 14:32 Created.

////////

use std::sync::Arc;
use crate::market::view::active::GoodsViewActivePort;
use crate::market::view::add::GoodsViewAddPort;
use crate::market::view::del::GoodsViewDelPort;
use crate::market::view::get::GoodsViewGetPort;
use crate::market::view::list::GoodsViewListPort;
use crate::market::view::manage::GoodsViewManagePort;
use crate::market::view::stat::GoodsViewStatPort;

////////
pub mod active; // 活跃
pub mod add; // 发布
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 计数

////////

/// # [VIEW PORTS]
/// * `desc`: `视频浏览服务端口`
#[derive(Clone)]
pub struct GoodsViewPort {
    pub active: Arc<dyn GoodsViewActivePort + Send + Sync + 'static>,
    pub add: Arc<dyn GoodsViewAddPort + Send + Sync + 'static>,
    pub del: Arc<dyn GoodsViewDelPort + Send + Sync + 'static>,
    pub get: Arc<dyn GoodsViewGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn GoodsViewListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn GoodsViewManagePort + Send + Sync + 'static>,
    pub stat: Arc<dyn GoodsViewStatPort + Send + Sync + 'static>,
}

//////// END
