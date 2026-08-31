// port/src/market/mod.rs -- 端口 - MARKET - mod
// 2026/6/18 12:20 Created.

////////

use crate::market::address::AddressPort;
use crate::market::buy::GoodsBuyPort;
use crate::market::cart::CartPort;
use crate::market::cate::GoodsCatePort;
use crate::market::collect::GoodsCollectPort;
use crate::market::comment::GoodsCommentPort;
use crate::market::express::ExpressPort;
use crate::market::goods::GoodsPort;
use crate::market::shop::ShopPort;
use crate::market::view::GoodsViewPort;
use std::sync::Arc;

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

/// # [COLA MARKET PORTS]
/// * `desc`: `MARKET - Market Ports`
#[derive(Clone)]
pub struct ColaMarketPort {
    pub address: AddressPort,      // 快递公司
    pub buy: GoodsBuyPort,         // 购买订单
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
