// repo_adapter/src/lib.rs
// 2026-06-12
// * 适配器层：将 repository 层的静态服务函数包装为 cola_data 中定义的 port trait
// * 并提供 build 函数将各 Adapter 组装为 AppContext

use std::sync::Arc;

pub mod auth;
pub mod dynamic;
pub mod gift;
pub mod gis;
pub mod im;
pub mod live;
pub mod market;
pub mod photo;
pub mod three;
pub mod user;
pub mod video;
pub mod wallet;

use cola_data::app::ctx::AppContext;
use cola_data::auth::port::AuthServicePorts;
use cola_data::gis::port::ColaGisPort;
use cola_data::im::port::ColaImPort;
use cola_data::live::port::ColaLivePort;
use cola_data::market::port::ColaMarketPort;
use cola_data::music::port::MusicServicePorts;
use cola_data::three::port::ColaThreePort;
use cola_data::user::port::ColaUserPort;
use cola_data::video::port::ColaVideoPort;

//////

/// 构建完整的 AppContext，注入所有 Adapter 实现
pub fn build_app_context() -> AppContext {
    // ---------- Gis ----------
    let gis = ColaGisPort {
        add: Arc::new(gis::add::AddPortAdapter),
        buy: Arc::new(gis::buy::BuyPortAdapter),
        feed: Arc::new(gis::feed::FeedPortAdapter),
        hotlist: Arc::new(gis::hot::HotlistPortAdapter),
        collect: Arc::new(gis::collect::CollectPortAdapter),
        comment: Arc::new(gis::comment::CommentPortAdapter),
        danmaku: Arc::new(gis::danmaku::DanmakuPortAdapter),
        share: Arc::new(gis::share::SharePortAdapter),
        like: Arc::new(gis::like::LikePortAdapter),
        report: Arc::new(gis::report::ReportPortAdapter),
        view: Arc::new(gis::view::ViewPortAdapter),
    };

    // ---------- Video ----------
    let video = ColaVideoPort {
        add: Arc::new(video::add::AddPortAdapter),
        buy: Arc::new(video::buy::BuyPortAdapter),
        feed: Arc::new(video::feed::FeedPortAdapter),
        hotlist: Arc::new(video::hot::HotlistPortAdapter),
        collect: Arc::new(video::collect::CollectPortAdapter),
        comment: Arc::new(video::comment::CommentPortAdapter),
        danmaku: Arc::new(video::danmaku::DanmakuPortAdapter),
        share: Arc::new(video::share::SharePortAdapter),
        like: Arc::new(video::like::LikePortAdapter),
        report: Arc::new(video::report::ReportPortAdapter),
        view: Arc::new(video::view::ViewPortAdapter),
    };

    // ---------- User ----------
    let user = ColaUserPort {
        add: Arc::new(user::add::UserAddPortAdapter),
        black: Arc::new(user::blacklist::UserBlacklistPortAdapter),
        con: Arc::new(user::config::UserConfigPortAdapter),
        following: Arc::new(user::following::UserFollowingPortAdapter),
        friend: Arc::new(user::friend::UserFriendPortAdapter),
        home: Arc::new(user::home::UserHomePortAdapter),
        info: Arc::new(user::info::UserInfoPortAdapter),
        view: Arc::new(user::view::UserViewPortAdapter),
    };

    // ---------- Market ----------
    let market = ColaMarketPort {
        address: Arc::new(market::address::AddressAdapter),
        buy: Arc::new(video::buy::BuyPortAdapter),
        feed: Arc::new(video::feed::FeedPortAdapter),
        express: Arc::new(market::express::ExpressAdapter),
        goods: Arc::new(market::goods::GoodsAdapter),
        goods_collect: Arc::new(market::goods_collect::GoodsCollectAdapter),
        goods_view: Arc::new(market::goods_view::GoodsViewAdapter),
        goods_mange: Arc::new(video::comment::CommentPortAdapter),
        danmaku: Arc::new(video::danmaku::DanmakuPortAdapter),
        share: Arc::new(video::share::SharePortAdapter),
        // 店铺管理
        shop_manage: Arc::new(market::shop::ShopManageAdapter),
        report: Arc::new(video::report::ReportPortAdapter),
        view: Arc::new(video::view::ViewPortAdapter),
    };

    // ---------- Three ----------
    let three = ColaThreePort {
        r#type: Arc::new(three::three_type::TypeAdapter),
        vendor: Arc::new(three::three_vendor::VendorAdapter),
        config: Arc::new(three::three_config::ConfigAdapter),
        binding: Arc::new(three::three_biz_binding::BindingAdapter),
    };

    // ---------- Music (复用 video port trait) ----------
    let music = MusicServicePorts {
        add: Arc::new(video::add::AddPortAdapter),
        like: Arc::new(video::like::LikePortAdapter),
        view: Arc::new(video::view::ViewPortAdapter),
    };

    // ---------- Live (复用 video port trait) ----------
    let live = ColaLivePort {
        add: Arc::new(video::add::AddPortAdapter),
        like: Arc::new(video::like::LikePortAdapter),
        view: Arc::new(video::view::ViewPortAdapter),
    };

    // ---------- IM ----------
    let im = ColaImPort {
        contact: Arc::new(im::contact::ContactPortAdapter),
        contact_request: Arc::new(im::contact_request::ContactRequestPortAdapter),
        card: Arc::new(im::card::CardPortAdapter),
        message: Arc::new(im::message::MessagePortAdapter),
        chat: Arc::new(im::chat::ChatPortAdapter),
    };

    // ---------- Auth (目前为空结构体) ----------
    let auth = AuthServicePorts {};

    AppContext::default(auth, gis, live, market, music, three, user, video, im)
}

/////// END