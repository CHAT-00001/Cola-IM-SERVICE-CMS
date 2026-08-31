// router_v1/music - 路由器 music

////////

pub mod auth;
pub mod dynamic;
pub mod gift;
pub mod live;
pub mod music;
pub mod response;
pub mod user;
pub mod video;

////////

use actix_web::web;
use app_config::app_state::AppState;

////////

/// # [ROUTER] - v1
/// * `desc` 2级路由器
pub fn boot_router_v1(cfg: &mut web::ServiceConfig, app_state: AppState) {
    // 把 AppState 包装成 web::Data
    let shared_data = web::Data::new(app_state);

    cfg.service(
        // API Server version v1
        web::scope("/v1")
            .app_data(shared_data) // 🚀 直接强灌，100% 成功，绝无丢数据的可能！
            // 认证中心
            .configure(auth::router::auth_router)
            // 动态
            .configure(dynamic::router::dynamic_router)
            // 直播
            .configure(live::gateway::live_router)
            // 音乐
            .configure(music::router::music_router)
            // 用户
            .configure(user::router::user_router)
            // 视频
            .configure(video::gateway::video_router), // 客户端
    );
}

//////// END
