// api/src/router2  -- 传输层 - 根 - 路由器
// 2025/12/25 08:00 by wx: cestbon10080

////////

use actix_web::web;
use crate::{http};


/// # [ROUTER] - v1 - 2级路由器
/// * 指向不同的业务模块
pub fn boot_router_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(

        // API Server version v1
        web::scope("/v1")
            // 认证中心
            .configure(http::auth::router::auth_router)
            // 礼物
            //.configure(handlers::gift::config)
            // 动态
            .configure(http::dynamic::router::dynamic_router)
            // 音乐
            .configure(http::music::router::music_router)
            // 用户
            .configure(http::user::router::user_router)
            // 视频
            .configure(http::video::router2::video_router)
            // 客户端
            .configure(http::client::router::config)
            // XX
            //.configure(music::routes::config)
    );
}

//////// END