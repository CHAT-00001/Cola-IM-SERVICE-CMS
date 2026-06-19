// cola_data/src/market/port/mod.rs  --
// 2026/6/18 12:20

////////

use std::sync::Arc;
use crate::market::port::address::AddressPort;
use crate::market::port::express::ExpressPort;
use crate::market::port::goods::GoodsPort;
use crate::market::port::goods_collect::GoodsCollectPort;
use crate::market::port::goods_view::GoodsViewPort;
use crate::market::port::shop_manage::ShopManagePort;
use crate::video::port::add::AddPort;
use crate::video::port::buy::BuyRepo;
use crate::video::port::comment::CommentRepo;
use crate::video::port::danmaku::DanmakuRepo;
use crate::video::port::feed::FeedRepo;
use crate::video::port::report::ReportRepo;
use crate::video::port::share::ShareRepo;
use crate::video::port::view::ViewPort;

pub mod shop_appy;
pub mod shop_manage;
pub mod address;
pub mod goods;
pub mod goods_view;
pub mod goods_collect;
pub mod express;
pub mod goods_feed;
////////

/// # [SERVICE] - 市场 ServicePort
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
    // 店铺管理
    pub shop_manage: Arc<dyn ShopManagePort + Send + Sync + 'static>,
    pub report: Arc<dyn ReportRepo + Send + Sync + 'static>,
    pub view: Arc<dyn ViewPort + Send + Sync + 'static>,
}

////////
