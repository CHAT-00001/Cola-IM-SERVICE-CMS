// api/http/video/add.rs  -- 传输层 - HTTP - 短视频 - Handler - 发布
// 2026/5/19 13:05 by wx: cestbon10080
// 9个方法
//

//////

use crate::kits::response::IntoApi;
use actix_web::web::ServiceConfig;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web, Scope};
use app_core::ctx::AppContext;
use data::app::request::ApiUrlParamsQuery;
use data::auth::info::auth::AuthContext;
use data::auth::entity::session::AuthSessionEntity;
use std::time::Instant;
use app_core::video::app::{add};
use data::video::command::video::VideoCommand;


//////

/// # [HANDLER] - 发布视频
/// * 路径: /video/add/video
pub(crate) async fn add_video(
    req: HttpRequest,
    cmd: web::Json<VideoCommand>,
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

    let result = add::case_add_video(
        &auth_ctx,
        cmd.into_inner(),
        &ctx,
    )
        .await;

    result.finish(&req, start)
}

////////

/// # [HANDLER] - 发布评论
pub async fn add_comment() -> HttpResponse {
    HttpResponse::Ok().json(vec!["发布评论", "successfully"])
}

////////

/// # [HANDLER] - 发布弹幕
pub async fn add_danmaku() -> HttpResponse {
    HttpResponse::Ok().json(vec!["发布弹幕", "successfully"])
}

////////

/// # [HANDLER] - 分享视频
pub async fn add_share() -> HttpResponse {
    HttpResponse::Ok().json(vec!["分享", "successfully"])
}

////////

/// # [HANDLER] - 添加收藏
pub async fn add_collect() -> HttpResponse {
    HttpResponse::Ok().json(vec!["添加收藏", "successfully"])
}

////////

/// # [HANDLER] - 推荐视频
pub async fn add_recommend() -> HttpResponse {
    HttpResponse::Ok().json(vec!["推荐", "successfully"])
}

////////

/// # [HANDLER] - 上热门
pub async fn add_hotlist() -> HttpResponse {
    HttpResponse::Ok().json(vec!["上热门", "successfully"])
}

////////

/// # [HANDLER] - 举报视频
pub async fn add_report() -> HttpResponse {
    HttpResponse::Ok().json(vec!["举报视频", "successfully"])
}
