// transport/src/handlers/session -- 路由器 AUTH相关
// 2026-01-02 10:25

//////

use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Deserialize;
use std::time::Instant;

//////

#[derive(Debug, Deserialize)]
pub struct LoginRequest {}

/// # [HANDLER] - 登录
pub async fn handle_login(req: HttpRequest) -> impl Responder {
    let start = Instant::now();
    // TODO: implement port
    HttpResponse::Ok().json(serde_json::json!({"msg": "port"}))
}
