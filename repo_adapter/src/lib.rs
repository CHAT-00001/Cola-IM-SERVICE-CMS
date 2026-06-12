// repo_adapter/src/lib.rs
// 2026-06-12
// 适配器层：将 repo 层的静态服务函数包装为 cola_data 中定义的 port trait
// 并提供 build 函数将各 Adapter 组装为 AppContext

use std::sync::Arc;

pub mod video;
pub mod user;

use cola_data::app::ctx::AppContext;
use cola_data::auth::port::AuthServicePorts;
use cola_data::live::port::ColaLivePort;
use cola_data::music::port::MusicServicePorts;
use cola_data::user::port::ColaUserPort;
use cola_data::video::port::ColaVideoPort;

/// 构建完整的 AppContext，注入所有 Adapter 实现
pub fn build_app_context() -> AppContext {
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
        info: Arc::new(user::info::UserInfoPortAdapter),
        view: Arc::new(user::view::UserViewPortAdapter),
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

    // ---------- Auth (目前为空结构体) ----------
    let auth = AuthServicePorts {};

    AppContext::default(auth, live, music, user, video)
}
