// api http/video/handler/user  -- 短视频 - handler - TA的
// 2026/5/19 20:53 by wx: cestbon10080

////////

use crate::kits::response::IntoApi;
use actix_web::web::ServiceConfig;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, Scope, web};
use app_core::ctx::AppContext;
use data::app::request::ApiUrlParamsQuery;
use data::auth::info::auth::AuthContext;
use std::time::Instant;
use data::auth::entity::session::AuthSessionEntity;
//////


/// # 1. [Handler] - 用户发布的视频
/// * Url: /video/user/1234/publish
pub async fn user_publish(
    req: HttpRequest,
    id: web::Path<i64>,
    query: web::Query<ApiUrlParamsQuery>,
    auth: web::ReqData<AuthSessionEntity>,
    ctx: web::Data<AppContext>,
) -> impl Responder {

    let start = Instant::now();

    let target_user_id = id.into_inner();

    let auth_ctx = AuthContext::new(
        Some(auth.user_id),
        auth.access_token.clone(),
        auth.refresh_token.clone(),
        auth.device_id.clone(),
    );

    let result = app_core::video::app::user::case_user_publish(
        query.into_inner().build(),
        target_user_id,
        &auth_ctx,
        &ctx,
    )
        .await;

    result.finish(&req, start)
}

////////

/// # [HANDLER] - 用户看过的视频
pub async fn user_visited() -> HttpResponse {
    HttpResponse::Ok().json(vec!["评论1", "评论2"])
}

////////

/// # [HANDLER] - 用户喜欢的视频
pub async fn user_liked() -> HttpResponse {
    HttpResponse::Ok().json(vec!["弹幕1", "弹幕2"])
}

////////

/// # [HANDLER] - 用户收藏的视频
pub async fn user_collected() -> HttpResponse {
    HttpResponse::Ok().json(vec!["动态1", "动态2"])
}

////////

/// # [HANDLER] - 用户推荐的视频
pub async fn user_recommend() -> HttpResponse {
    HttpResponse::Ok().json(vec!["实况1", "实况2"])
}

//////// END
