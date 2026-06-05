// api/http/video/me.rs  -- 传输层 - HTTP - 短视频 - Handler - 我的
// 2026/5/19 13:05 by wx: cestbon10080

//////

use std::time::Instant;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use app_core::ctx::AppContext;
use data::app::request::ApiUrlParamsQuery;
use data::auth::entity::session::AuthSessionEntity;
use data::auth::info::auth::AuthContext;
use crate::kits::response::IntoApi;

//////

/// # 1. [Handler] - 我发布的视频
pub async fn me_publish(
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

    let result = app_core::video::app::me::case_me_publish(
        query.into_inner().build(),
        &auth_ctx,
        &ctx,
    )
        .await;

    result.finish(&req, start)
}

////////

/// # [HANDLER] - 查看评论 by video id
pub async fn get_my_vis() -> HttpResponse {
    HttpResponse::Ok().json(vec!["评论1", "评论2"])
}

////////

/// # [HANDLER] - 查看弹幕 by video id
pub async fn get_danmakus_by_video_id() -> HttpResponse {
    HttpResponse::Ok().json(vec!["弹幕1", "弹幕2"])
}

////////

/// # [HANDLER] - 查看动态 by video id
pub async fn get_dynamics_by_video_id() -> HttpResponse {
    HttpResponse::Ok().json(vec!["动态1", "动态2"])
}

////////

/// # [HANDLER] - 查看直播 by video id
pub async fn get_lives_by_video_id() -> HttpResponse {
    HttpResponse::Ok().json(vec!["实况1", "实况2"])
}

////////

/// # [HANDLER] - 查看活动 by video id
pub async fn get_activity_by_video_id() -> HttpResponse {
    HttpResponse::Ok().json(vec!["实况1", "实况2"])
}

//////// END