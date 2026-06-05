use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use app_config::app_state::AppState;
use app_config::config::Health;
use tracing::info;

pub async fn start_health(health_config: &Health, app_state: AppState) {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())  // ✅ 启用访问日志
            .app_data(web::Data::new(app_state.clone()))
            .route("/ping", web::get().to(ping))
            // 2级路由器
            .route("/", web::get().to(index))
    })
        .bind((health_config.host.as_str(), health_config.port))
        .expect("Failed to bind API server")
        .run()
        .await
        .expect("API server runtime error");
}



/// # PING
/// * PONG
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}


/// # PING
/// * PONG
async fn index() -> impl Responder {
    HttpResponse::Ok().body("pong")
}