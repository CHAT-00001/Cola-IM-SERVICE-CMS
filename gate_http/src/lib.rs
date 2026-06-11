// gate_http/src/lib.rs -- API lib
// 2026-01-16 13:48:00

////////
mod router;
pub mod http;
pub mod grpc;
pub mod models;
pub mod kits;
pub mod middleware;

////////

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;
use app_config::app_state::AppState;
use app_config::config::Api;
use crate::middleware::auth_check::JwtAuth;

////////

/// # [RUN] - Start_api_server

pub async fn start_api(api_config: &Api, app_state: AppState) {


    // [HTTP]
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(JwtAuth)
            .app_data(web::Data::new(app_state.clone()))
            .route("/ping", web::get().to(ping))
            // API服务 - 根路由
            .service(web::scope("/gate_http").configure(router::boot_router_v1))
    })
        .bind((api_config.host.as_str(), api_config.port))
        .expect("Failed to bind API server")
        .run()
        .await
        .expect("API server runtime error");
}


/// # PING
/// * PONG
async fn ping() -> impl Responder {
    let utc_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let uuid = Uuid::new_v4();
    let uuid_prefix = &uuid.to_string()[0..8];

    // 重点：里面的双引号改成单引号 ' 就不会报错了
    let body = format!(
        "<h1 style='color: #0066ff'>pong</h1>
        <p style='color: #00ff00; font-size: 16px'>UTC Time: {} | UUID (first 8 chars): {}</p>",
        utc_time, uuid_prefix
    );

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}


//////// END