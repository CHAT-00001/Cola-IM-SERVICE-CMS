// repo_adapter/src/market/mod.rs
// 🔌 插头 - 可乐市场 - 模块
// 2026/8/10 20:00 Updated.

////////

use port::cola_market::ColaMarketPort;
use std::sync::Arc;
use crate::stub;

////////

pub mod goods;
pub mod address;
pub mod express;
pub mod shop;
pub mod shop_apply;
pub mod goods_feed;
pub mod goods_collect;
pub mod goods_view;

////////

/// # [BUILD] - 构建 MARKET Port
/// * `desc`: 构建市场 Port 聚合体，包含商品和店铺相关功能
pub fn build_market_port() -> ColaMarketPort {
    ColaMarketPort {
        address: Arc::new(address::AddressAdapter),
        buy: Arc::new(stub::GeneralStubAdapter),
        feed: Arc::new(stub::GeneralStubAdapter),
        express: Arc::new(express::ExpressAdapter),
        goods: Arc::new(goods::GoodsAdapter),
        goods_collect: Arc::new(goods_collect::GoodsCollectAdapter),
        goods_view: Arc::new(goods_view::GoodsViewAdapter),
        goods_mange: Arc::new(stub::GeneralStubAdapter),
        danmaku: Arc::new(stub::GeneralStubAdapter),
        share: Arc::new(stub::GeneralStubAdapter),
        shop_manage: Arc::new(shop::ShopManageAdapter),
        report: Arc::new(stub::GeneralStubAdapter),
        view: Arc::new(stub::GeneralStubAdapter),
    }
}

//////// END
