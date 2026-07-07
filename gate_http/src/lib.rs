// gate_http/src/lib.rs -- API lib
// 2026-01-16 13:48:00

////////
pub mod kits;
pub mod middleware;
pub mod models;
mod router_v1;
mod router_v2;
mod ping;

////////

use crate::middleware::auth_check::JwtAuth;
use actix_web::middleware::Logger;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use app_config::app_state::AppState;
use app_config::config::Api;
use crate::ping::ping;

////////

/// # [RUN] - Start_api_server

// gate_http/src/lib.rs

pub async fn start_api(api_config: &Api, app_state: AppState) {


    // 1. 提前克隆一份，准备送进 HttpServer 闭包
    let state_for_app = app_state.clone();

    // [HTTP]
    HttpServer::new(move || {
        // 2. 🌟 关键：在每一路 Worker 线程内再克隆一份，专门喂给路由层
        let state_for_router = state_for_app.clone();

        App::new()

            // Logger:
            //.wrap(Logger::new("%a \"%m \x1b]8;uri=router_v2://%{Host}i%U%q\x1b\\\x1b[38;2;0;51;255m\x1b[4m%U%q\x1b[0m\x1b]8;;\x1b\\ %H\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
            .wrap(Logger::new(
                "%a \"%m http://%{Host}i%U% %H\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"
            ))
            .wrap(JwtAuth)
            .app_data(web::Data::new(state_for_app.clone()))
            .route("/ping", web::get().to(ping))

            // API服务 - 根路由
            // v1 v2 双路由分叉
            .service(
                web::scope("/api")
                    // 喂给 v1，记得把 state 克隆一份传过去
                    .configure({
                        let state = state_for_router.clone();
                        move |cfg| router_v1::boot_router_v1(cfg, state)
                    })
                    // 喂给 v2，把最后的这份 state 直接 move 进去
                    .configure({
                        let state = state_for_router;
                        move |cfg| router_v2::boot_router_v2(cfg, state)
                    })
            )
    })
        .bind((api_config.host.as_str(), api_config.port))
        .expect("Failed to bind API server")
        .run()
        .await
        .expect("API server runtime error");
}

//////// END
