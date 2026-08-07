// gate_http/src/router_v2/cola_auth/router2 -- 路由器 AUTH相关
// 2026-01-02 10:25

//////

use actix_web::{web};
use crate::router_v2::auth::handler::login::{apple_login, email_login, google_login, phone_login, wechat_login};
use crate::router_v2::auth::handler::logout::logout;
use crate::router_v2::auth::handler::refresh::refresh_token;
use crate::router_v2::auth::handler::send::{send_email_code, send_msm_code};

//////

/// [ROUTER] -  认证中心 - 三级路由
pub fn auth_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cola_auth")
            .service(
                web::scope("/port")
                    .route("/phone", web::get().to(phone_login))
                    .route("/email", web::get().to(email_login))
                    .route("/wechat", web::get().to(wechat_login))
                    .route("/google", web::get().to(google_login))
                    .route("/apple", web::get().to(apple_login)),
            )
            .route("/logout", web::post().to(logout))
            .route("/refresh", web::post().to(refresh_token))
            .service(
                web::scope("/send")
                    .route("/sms", web::post().to(send_msm_code))
                    .route("/email", web::post().to(send_email_code)),
            )
            .route("/sms-send", web::post().to(refresh_token)),
    );
}