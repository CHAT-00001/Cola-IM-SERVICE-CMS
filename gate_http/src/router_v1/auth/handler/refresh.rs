// /refresh.rs  --
// 2026/5/25 02:54 by wx: cestbon10080

////////

use crate::router_v2::auth::handler::login::LoginResponse;
use actix_web::HttpResponse;

/// # [HANDLER] - 刷新 Token
pub async fn refresh_token() -> HttpResponse {
    let response = LoginResponse {
        token: "new_jwt_token_here".to_string(),
        refresh_token: "new_refresh_token_here".to_string(),
        expires_in: 3600,
    };

    HttpResponse::Ok().json(response)
}
