// /router2  -- 
// 2026/5/19 17:03 by wx: cestbon10080

//////

use actix_web::web;

/// [Router] - 动态 - 路由器
pub fn dynamic_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/dynamic")
            .route("", web::get().to(|| async { web::Json(serde_json::json!({"msg": "dynamic"})) })),
    );
}

//////
