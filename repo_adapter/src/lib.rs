// repo_adapter/src/lib.rs
// 🔌 插头 - lib
// 2026-06-12

////////

use cola_data::app::ctx::AppContext;
use cola_data::auth::port::AuthServicePorts;
use cola_data::gis::port::ColaGisPort;
use cola_data::im::port::ColaImPort;
use cola_data::live::port::ColaLivePort;
use cola_data::market::port::ColaMarketPort;
use cola_data::music::port::ColaMusicPort;
use cola_data::three::port::ColaThreePort;
use cola_data::user::port::ColaUserPort;
use cola_data::user::port::ban::BanPort;
use cola_data::user::port::black::BlackPort;
use cola_data::user::port::follow::FollowPort;
use cola_data::user::port::friend::FriendPort;
use cola_data::user::port::role::RolePort;
use cola_data::user::port::share::SharePort;
use cola_data::user::port::user::UserPort;
use cola_data::user::port::view::ViewPort;
use cola_data::user::port::vip::VipPort;
use cola_data::video::port::ColaVideoPort;
use cola_data::video::port::buy::BuyPort;
use cola_data::video::port::collect::CollectPort;
use cola_data::video::port::comment::CommentPort;
use cola_data::video::port::danmaku::DanmakuPort;
use cola_data::video::port::dislike::DislikePort;
use cola_data::video::port::hotlist::HotlistPort;
use cola_data::video::port::like::LikePort;
use cola_data::video::port::recommend::RecommendPort;
use cola_data::video::port::report::ReportPort;
use cola_data::video::port::share::VideoSharePort;
use cola_data::video::port::video::VideoPort;
use cola_data::video::port::view::VideoViewPort;
use std::sync::Arc;

////////

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

////////

/// # [BUILD] - 构建APP上下文
pub fn build_app_context() -> AppContext {
    //

    ////////

    // ---------- Auth ----------
    // 🆔 可乐IAM
    let auth = AuthServicePorts {
        session: Arc::new(auth::session::SessionPortAdapter),
    };

    ////////

    // ---------- Dynamic ----------
    // 🕳️ 可乐动态 (预设)

    ////////

    // ---------- GIS ----------
    // 📍 可乐GIS
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

    ////////

    // ---------- Live ----------
    // 📺 可乐直播
    let live = ColaLivePort {
        add: Arc::new(()),
        like: Arc::new(()),
        view: Arc::new(()),
    };

    ////////

    // ---------- Music ----------
    // 🎶 可乐音乐
    let music = ColaMusicPort {
        add: Arc::new(()),
        like: Arc::new(()),
        view: Arc::new(()),
    };

    ////////

    // 👤 可乐用户 SERVICE PORTS
    let user = ColaUserPort {
        add: Arc::new(user::add::UserAddPortAdapter),
        // BAN - 封禁
        ban: BanPort {
            add: Arc::new(user::ban::add::BanAddService),
            check: Arc::new(user::ban::check::BanCheckService),
            del: Arc::new(user::ban::del::BanDelService),
            get: Arc::new(user::ban::get::BanGetService),
            list: Arc::new(user::ban::list::BanListService),
            manage: Arc::new(user::ban::manage::BanManageService),
        },
        // BLACK - 黑名单
        black: BlackPort {
            add: Arc::new(user::black::add::BlackAddAdapter),
            check: Arc::new(user::black::check::BlackCheckAdapter),
            del: Arc::new(user::black::del::BlackDelAdapter),
            get: Arc::new(user::black::get::BlackGetAdapter),
            list: Arc::new(user::black::list::BlackListAdapter),
            manage: Arc::new(user::black::manage::BlackManageAdapter),
        },

        // FOLLOW - 关注
        cate: Arc::new(()),
        category: Arc::new(()),
        con: Arc::new(()),
        follow: FollowPort {
            add: Arc::new(user::follow::FollowAdapter),
            check: Arc::new(user::follow::FollowAdapter),
            del: Arc::new(user::follow::FollowAdapter),
            get: Arc::new(user::follow::FollowAdapter),
            list: Arc::new(user::follow::FollowAdapter),
            manage: Arc::new(user::follow::FollowAdapter),
        },
        // FRIEND - 朋友
        friend: FriendPort {
            add: Arc::new(()),
            check: Arc::new(()),
            del: Arc::new(()),
            get: Arc::new(()),
            list: Arc::new(()),
            manage: Arc::new(()),
        },
        home: Arc::new(()),
        info: Arc::new(()),
        // ROLE - 角色
        role: RolePort {
            add: Arc::new(()),
            check: Arc::new(()),
            del: Arc::new(()),
            get: Arc::new(()),
            list: Arc::new(()),
            manage: Arc::new(()),
        },
        share: SharePort {
            add: Arc::new(()),
            check: Arc::new(()),
            del: Arc::new(()),
            get: Arc::new(()),
            list: Arc::new(()),
            manage: Arc::new(()),
        },
        // USER - 用户
        user: UserPort {
            add: Arc::new(user::user::add::UserAddAdapter),
            check: Arc::new(user::user::check::UserCheckAdapter),
            del: Arc::new(user::user::del::UserDelAdapter),
            get: Arc::new(user::user::get::UserGetAdapter),
            list: Arc::new(user::user::list::UserListAdapter),
            manage: Arc::new(user::user::manage::UserManageAdapter),
        },
        // VIEW - 浏览
        view: ViewPort {
            add: Arc::new(user::view::add::ViewAddService),
            check: Arc::new(user::view::check::ViewCheckService),
            del: Arc::new(user::view::del::ViewDelService),
            get: Arc::new(user::view::get::ViewGetService),
            list: Arc::new(user::view::list::ViewListService),
            manage: Arc::new(user::view::manage::ViewManageService),
        },

        // VIP - 贵宾
        vip: VipPort {
            add: Arc::new(user::vip::VipAdapter),
            check: Arc::new(user::vip::VipAdapter),
            del: Arc::new(user::vip::VipAdapter),
            get: Arc::new(user::vip::VipAdapter),
            list: Arc::new(user::vip::VipAdapter),
            manage: Arc::new(user::vip::VipAdapter),
        },
    };

    ////////

    // ---------- Video ----------
    // 🎥 可乐视频 SERVICE PORTS
    let video = ColaVideoPort {
        add: Arc::new(()),
        buy: BuyPort {
            add: Arc::new(()),
            check: Arc::new(()),
            del: Arc::new(()),
            get: Arc::new(()),
            list: Arc::new(()),
            manage: Arc::new(()),
            stat: Arc::new(()),
        },
        collect: CollectPort {
            add: Arc::new(()),
            check: Arc::new(()),
            del: Arc::new(()),
            get: Arc::new(()),
            list: Arc::new(()),
            manage: Arc::new(()),
            stat: Arc::new(()),
        },
        comment: CommentPort {
            add: Arc::new(()),
            check: Arc::new(()),
            del: Arc::new(()),
            get: Arc::new(()),
            like: Arc::new(()),
            list: Arc::new(()),
            manage: Arc::new(()),
            report: Arc::new(()),
            stat: Arc::new(()),
        },
        danmaku: DanmakuPort {
            add: Arc::new(()),
            check: Arc::new(()),
            del: Arc::new(()),
            get: Arc::new(()),
            like: Arc::new(()),
            list: Arc::new(()),
            manage: Arc::new(()),
            stat: Arc::new(()),
            report: Arc::new(()),
            step: Arc::new(()),
        },
        dislike: DislikePort {
            add: Arc::new(()),
            del: Arc::new(()),
            get: Arc::new(()),
            list: Arc::new(()),
            manage: Arc::new(()),
            stat: Arc::new(()),
        },
        hotlist: HotlistPort {
            add: Arc::new(()),
            check: Arc::new(()),
            del: Arc::new(()),
            get: Arc::new(()),
            list: Arc::new(()),
            manage: Arc::new(()),
            stat: Arc::new(()),
        },
        like: LikePort {
            add: Arc::new(()),
            check: Arc::new(()),
            del: Arc::new(()),
            get: Arc::new(()),
            list: Arc::new(()),
            manage: Arc::new(()),
            stat: Arc::new(()),
        },
        recommend: RecommendPort {
            add: Arc::new(()),
            check: Arc::new(()),
            del: Arc::new(()),
            get: Arc::new(()),
            list: Arc::new(()),
            manage: Arc::new(()),
            stat: Arc::new(()),
        },
        report: ReportPort {
            add: Arc::new(()),
            check: Arc::new(()),
            del: Arc::new(()),
            get: Arc::new(()),
            list: Arc::new(()),
            manage: Arc::new(()),
            stat: Arc::new(()),
        },
        share: VideoSharePort {
            add: Arc::new(()),
            check: Arc::new(()),
            del: Arc::new(()),
            get: Arc::new(()),
            list: Arc::new(()),
            manage: Arc::new(()),
            stat: Arc::new(()),
        },
        video: VideoPort {
            add: Arc::new(video::video::add::VideoAddAdapter),
            check: Arc::new(video::video::check::VideoCheckAdapter),
            del: Arc::new(video::video::del::VideoDelAdapter),
            get: Arc::new(video::video::get::VideoGetAdapter),
            list: Arc::new(video::video::list::VideoListAdapter),
            manage: Arc::new(video::video::manage::VideoManageAdapter),
            stat: Arc::new(video::video::stat::VideoStatAdapter),
        },
        view: VideoViewPort {
            add: Arc::new(()),
            check: Arc::new(()),
            del: Arc::new(()),
            get: Arc::new(()),
            list: Arc::new(()),
            manage: Arc::new(()),
        },
    };

    // 🛒 可乐市场
    let market = ColaMarketPort {
        address: Arc::new(market::address::AddressAdapter),

        buy: Arc::new(()),
        feed: Arc::new(()),
        express: Arc::new(market::express::ExpressAdapter),
        goods: Arc::new(market::goods::GoodsAdapter),
        goods_collect: Arc::new(market::goods_collect::GoodsCollectAdapter),
        goods_view: Arc::new(market::goods_view::GoodsViewAdapter),

        goods_mange: Arc::new(()),
        danmaku: Arc::new(()),
        share: Arc::new(()),
        shop_manage: Arc::new(market::shop::ShopManageAdapter),

        report: Arc::new(()),
        view: Arc::new(()),
    };

    // 🔑 可乐三方
    let three = ColaThreePort {
        r#type: Arc::new(three::three_type::TypeAdapter),
        vendor: Arc::new(three::three_vendor::VendorAdapter),
        config: Arc::new(three::three_config::ConfigAdapter),
        binding: Arc::new(three::three_biz_binding::BindingAdapter),
    };

    // ---------- IM ----------
    // 💬 可乐IM
    let im = ColaImPort {
        contact: Arc::new(im::contact::ContactPortAdapter),
        contact_request: Arc::new(im::contact_request::ContactRequestPortAdapter),
        card: Arc::new(im::card::CardPortAdapter),
        message: Arc::new(im::message::MessagePortAdapter),
        chat: Arc::new(im::chat::ChatPortAdapter),
    };

    AppContext::default(auth, gis, live, market, music, three, user, video, im)
}

//////// END
