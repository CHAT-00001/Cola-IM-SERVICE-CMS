use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use app_config::app_state::AppState;
use app_config::config::Health;
use tracing::info;

pub async fn start_health(health_config: &Health, app_state: AppState) {
    HttpServer::new(move || {
        App::new()
            // 自定义 Logger: URL 使用 OSC 8 超链接（颜色 #0033ff + 下划线），终端可点击
            .wrap(Logger::new("%a \"%m \x1b]8;uri=router_v2://%{Host}i%U%q\x1b\\\x1b[38;2;0;51;255m\x1b[4m%U%q\x1b[0m\x1b]8;;\x1b\\ %H\" %share %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
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