// port/src/cola_market/mod.rs
// ⏩️ 端口 - MARKET - mod
// 2026/6/18 12:20 Created.

////////

use crate::cola_market::buy::GoodsBuyOrderPort;
use crate::cola_market::cart::CartPort;
use crate::cola_market::cate::GoodsCatePort;
use crate::cola_market::collect::GoodsCollectPort;
use crate::cola_market::comment::GoodsCommentPort;
use crate::cola_market::express::ExpressPort;
use crate::cola_market::goods::GoodsPort;
use crate::cola_market::shop::ShopPort;
use crate::cola_market::view::GoodsViewPort;
use std::sync::Arc;
use crate::cola_market::address::AddressPort;
////////

pub mod address; // 地址
pub mod buy; // 购买
pub mod cart; // 购物车
pub mod cate; // 分类
pub mod collect; // 收藏夹
pub mod comment; // 商品评论
pub mod express; // 快递
pub mod goods; // 商品
pub mod shop; // 店铺
pub mod view; // 商品浏览记录

////////

/// # 🛒 [COLA MARKET PORTS]
/// * `desc`: `⏹ 可乐市场 - Market Ports`
#[derive(Clone)]
pub struct ColaMarketPort {
    pub address: AddressPort,      // 快递公司
    pub buy: GoodsBuyOrderPort,    // 购买订单
    pub cart: CartPort,            // 购物车
    pub cate: GoodsCatePort,       // 商品分类
    pub collect: GoodsCollectPort, // 商品收藏夹
    pub comment: GoodsCommentPort, // 商品评论
    pub express: ExpressPort,      // 快递公司
    pub goods: GoodsPort,          // 商品
    pub shop: ShopPort,            // 商店
    pub goods_view: GoodsViewPort, // 商品浏览记录
}

//////// END
