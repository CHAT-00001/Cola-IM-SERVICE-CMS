// /session  --
// 2026/5/19 19:03 by wx: cestbon10080

////////

use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

////////

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

/// # [HANDLER] - 电话登录
pub async fn phone_login(login_req: web::Json<LoginRequest>) -> HttpResponse {
    let response = LoginResponse {
        token: "jwt_token_here".to_string(),
        refresh_token: "refresh_token_here".to_string(),
        expires_in: 3600,
    };

    HttpResponse::Ok().json(response)
}

/// # [HANDLER] - 邮箱登录
pub async fn email_login(login_req: web::Json<LoginRequest>) -> HttpResponse {
    let response = LoginResponse {
        token: "jwt_token_here".to_string(),
        refresh_token: "refresh_token_here".to_string(),
        expires_in: 3600,
    };

    HttpResponse::Ok().json(response)
}

/// # [HANDLER] - 微信登录
pub async fn wechat_login(login_req: web::Json<LoginRequest>) -> HttpResponse {
    let response = LoginResponse {
        token: "jwt_token_here".to_string(),
        refresh_token: "refresh_token_here".to_string(),
        expires_in: 3600,
    };

    HttpResponse::Ok().json(response)
}

/// # [HANDLER] - 谷歌登录
pub async fn google_login(login_req: web::Json<LoginRequest>) -> HttpResponse {
    let response = LoginResponse {
        token: "jwt_token_here".to_string(),
        refresh_token: "refresh_token_here".to_string(),
        expires_in: 3600,
    };

    HttpResponse::Ok().json(response)
}

/// # [HANDLER] - 苹果登录
pub async fn apple_login(login_req: web::Json<LoginRequest>) -> HttpResponse {
    let response = LoginResponse {
        token: "jwt_token_here".to_string(),
        refresh_token: "refresh_token_here".to_string(),
        expires_in: 3600,
    };

    HttpResponse::Ok().json(response)
}
