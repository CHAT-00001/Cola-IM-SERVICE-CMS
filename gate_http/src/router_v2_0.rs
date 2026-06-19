// /router_v2_0  -- 
// 2026/6/13 10:39

////////


// gate_http/src/router_v1  -- 传输层 - 根 - 路由器
// 2026-06-12 降维打击版

use actix_web::web;
use crate::{http};
use app_config::app_state::AppState;

/// # [ROUTER] - v1 - 2级路由器
/// * `desc` v1 路由器
pub fn boot_router_v2(cfg: &mut web::ServiceConfig, app_state: AppState) {

    // 把 AppState 包装成 web::Data
    let shared_data = web::Data::new(app_state);

    cfg.service(
        // API Server version v2
        web::scope("/v2")
            .app_data(shared_data) // 🚀 直接强灌，100% 成功，绝无丢数据的可能！
            // 认证中心
            .configure(http::auth::router::auth_router)
            // 动态
            .configure(http::dynamic::router::dynamic_router)
            // 直播
            .configure(http::live::gateway::live_router)
            // 音乐
            .configure(http::music::router::music_router)
            // 用户
            .configure(http::user::router::user_router)
            // 视频
            .configure(http::video::gateway::video_router)
            // 客户端
            .configure(http::client::router::config)
    );
}

//////// END