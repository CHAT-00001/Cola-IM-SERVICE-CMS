// router_v2/mod  - 路由器 mod
// 2026-06-12 10:40

//////

pub mod auth;
pub mod dynamic;
mod fs;
pub mod gift;
pub mod gis;
pub mod live;
pub mod music;
pub mod response;
pub mod three;
pub mod user;
pub mod video;

//////

use actix_web::web;
use app_config::app_state::AppState;

//////

/// # [ROUTER] - v2
/// * `desc` 2级路由器
pub fn boot_router_v2(cfg: &mut web::ServiceConfig, app_state: AppState) {
    // 把 AppState 包装成 web::Data
    let shared_data = web::Data::new(app_state);

    cfg.service(
        // API Server version v2
        web::scope("/v2")
            .app_data(shared_data)
            // 认证中心
            .configure(auth::gateway::auth_router)
            // 动态
            .configure(dynamic::gateway::dynamic_router)
            // 文件存储
            .configure(fs::gateway::fs_router)
            // 动态
            .configure(gis::gateway::gis_router)
            // 直播
            .configure(live::gateway::live_router)
            // 音乐
            .configure(music::router::music_router)
            // 第三方服务
            .configure(three::gateway::three_router)
            // 用户
            .configure(user::gateway::user_router)
            // 视频
            .configure(video::gateway::video_router),
    );
}

//////// END
