// cola_market/mod.rs
// ⏩️ 端口 - 可乐市场 - mod
// 2026/6/18 12:20 Created.

////////

use crate::cola_gis::buy::BuyRepo;
use crate::cola_gis::comment::CommentRepo;
use crate::cola_gis::danmaku::DanmakuRepo;
use crate::cola_gis::feed::FeedRepo;
use crate::cola_gis::report::ReportRepo;
use crate::cola_gis::share::ShareRepo;
use crate::cola_market::address::AddressPort;
use crate::cola_market::express::ExpressPort;
use crate::cola_market::goods::GoodsPort;
use crate::cola_market::goods_collect::GoodsCollectPort;
use crate::cola_market::goods_view::GoodsViewPort;
use crate::cola_market::shop_manage::ShopManagePort;
use crate::cola_music::view::ViewPort;
use std::sync::Arc;

////////

pub mod address; // 地址
pub mod express; // 快递
pub mod goods; // 商品
pub mod goods_collect; // 商品收藏
pub mod goods_feed; //  商品feed
pub mod goods_view; // 商品浏览
pub mod shop_appy; // 商店
pub mod shop_manage; // 商店管理

////////

/// # 🛒 [COLA MARKET PORTS]
/// * `desc`: `⏹ 可乐市场 - Market Ports`
#[derive(Clone)]
pub struct ColaMarketPort {
    pub address: Arc<dyn AddressPort + Send + Sync + 'static>,
    pub buy: Arc<dyn BuyRepo + Send + Sync + 'static>,
    pub feed: Arc<dyn FeedRepo + Send + Sync + 'static>,
    pub express: Arc<dyn ExpressPort + Send + Sync + 'static>,
    pub goods: Arc<dyn GoodsPort + Send + Sync + 'static>,
    pub goods_collect: Arc<dyn GoodsCollectPort + Send + Sync + 'static>,
    pub goods_view: Arc<dyn GoodsViewPort + Send + Sync + 'static>,
    pub goods_mange: Arc<dyn CommentRepo + Send + Sync + 'static>,
    pub danmaku: Arc<dyn DanmakuRepo + Send + Sync + 'static>,
    pub share: Arc<dyn ShareRepo + Send + Sync + 'static>,
    pub shop_manage: Arc<dyn ShopManagePort + Send + Sync + 'static>,
    pub report: Arc<dyn ReportRepo + Send + Sync + 'static>,
    pub view: Arc<dyn ViewPort + Send + Sync + 'static>,
}

//////// END
