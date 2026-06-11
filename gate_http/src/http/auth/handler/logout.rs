// /logout.rs  -- 
// 2026/5/25 02:54 by wx: cestbon10080

////////

use actix_web::HttpResponse;


/// # [HANDLER] - 登出
pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"message": "Logged out successfully"}))
}
