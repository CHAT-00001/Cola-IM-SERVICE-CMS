// repo_adapter/src/market/mod.rs
// 🔌 适配器 - MARKET - mod
// 2026/8/10 20:00 Updated.

////////

use crate::stub;
use port::market::ColaMarketPort;

////////

pub mod address; // 地址
pub mod buy; // 购买订单
pub mod cart; // 购物车
pub mod cate; // 分类
pub mod collect; // 收藏夹
pub mod comment; // 商品评论
pub mod express; // 快递
pub mod goods; // 商品
pub mod shop; // 店铺
pub mod view; // 商品浏览记录

////////

/// # [BUILD] - 构建 MARKET Port
/// * `desc`: 构建市场 Port 聚合体，包含商品、订单、店铺等所有功能
pub fn build_market_port() -> ColaMarketPort {
    ColaMarketPort {
        address: address::build_address_port(),    // ✅ 地址 builder
        buy: buy::build_buy_port(),                // ✅ 购买 builder
        cart: cart::build_cart_port(),             // ✅ 购物车 builder
        cate: cate::build_cate_port(),             // ✅ 分类 builder
        collect: collect::build_collect_port(),    // ✅ 收藏 builder
        comment: comment::build_comment_port(),    // ✅ 评论 builder
        express: express::build_express_port(),    // ✅ 快递 builder
        goods: goods::build_goods_port(),          // ✅ 商品 builder
        shop: shop::build_shop_port(),             // ✅ 店铺 builder
        goods_view: view::build_goods_view_port(), // ✅ 浏览 builder
    }
}

//////// END
