// market/port/mod.rs
// 市场 - port - 模块
// 2026/6/18 12:20

////////

use crate::gis::port::buy::BuyRepo;
use crate::gis::port::comment::CommentRepo;
use crate::gis::port::danmaku::DanmakuRepo;
use crate::gis::port::feed::FeedRepo;
use crate::gis::port::report::ReportRepo;
use crate::gis::port::share::ShareRepo;
use crate::gis::port::view::ViewPort;
use crate::market::port::address::AddressPort;
use crate::market::port::express::ExpressPort;
use crate::market::port::goods::GoodsPort;
use crate::market::port::goods_collect::GoodsCollectPort;
use crate::market::port::goods_view::GoodsViewPort;
use crate::market::port::shop_manage::ShopManagePort;
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
/// * `desc`: `可乐市场 ServicePort`
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
