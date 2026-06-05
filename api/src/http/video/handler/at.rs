// /at.rs  --
// 2026/5/19 19:44 by wx: cestbon10080

////////

// api http/video/handler/user  -- 短视频 - handler - TA的
// 2026/5/19 20:53 by wx: cestbon10080

////////

use crate::kits::response::IntoApi;
use actix_web::web::ServiceConfig;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, Scope, web};
use app_core::ctx::AppContext;
use data::app::request::ApiUrlParamsQuery;
use data::auth::info::auth::AuthContext;
use data::auth::entity::session::AuthSessionEntity;
use std::time::Instant;

// # [ROUTER] - 艾特我的
pub fn at_router() -> Scope {
    web::scope("/at")
        .route("/following", web::get().to(get_following_list))
        .route("/friend", web::get().to(get_friend_list))
        .route("/recommend", web::get().to(get_recommend_list))
        .route("/nearby", web::get().to(get_nearby_list))
}

//////

/// # 1. [Handler] - 关注的视频
async fn get_following_list(
    req: HttpRequest,
    query: web::Query<ApiUrlParamsQuery>,
    auth: web::ReqData<AuthSessionEntity>,
    ctx: web::Data<AppContext>,
) -> impl Responder {

    let start = Instant::now();

    let auth_ctx = AuthContext::new(
        Some(auth.user_id),
        auth.access_token.clone(),
        auth.refresh_token.clone(),
        auth.device_id.clone(),
    );

    let result = app_core::video::app::at::case_at_following(
        query.into_inner().build(),
        &auth_ctx,
        &ctx,
    )
        .await;

    result.finish(&req, start)
}

////////

/// # [HANDLER] - 朋友的视频
async fn get_friend_list() -> HttpResponse {
    HttpResponse::Ok().json(vec!["视频1", "视频2"])
}

////////

/// # [HANDLER] - 为我推荐的视频
async fn get_recommend_list() -> HttpResponse {
    HttpResponse::Ok().json(vec!["视频1", "视频2"])
}

////////

/// # [HANDLER] - 我附近的视频
async fn get_nearby_list() -> HttpResponse {
    HttpResponse::Ok().json(vec!["视频1", "视频2"])
}

//////// END
