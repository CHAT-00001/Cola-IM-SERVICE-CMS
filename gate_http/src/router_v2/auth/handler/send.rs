// /send.rs  --
// 2026/5/19 19:03 by wx: cestbon10080

////////

use actix_web::HttpResponse;

/// # [HANDLER] - 发送短信验证码
pub async fn send_msm_code() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"message": "Logged out successfully"}))
}

/// # [HANDLER] - 发送邮箱验证码
pub async fn send_email_code() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"message": "Logged out successfully"}))
}
