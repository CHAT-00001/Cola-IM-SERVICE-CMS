// /view  --
// 2026/5/19 16:19 by wx: cestbon10080

////////

use crate::kits::response::IntoApi;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web, Scope};
use app_core::ctx::AppContext;
use app_core::video::app::view;
use app_core::video::app::view::ViewQuery;
use serde_json::json;
use std::time::Instant;

////////

/// # [HANDLER] - 浏览视频
/// * 路径: /video/view/{id}
async fn view_handler(
    req: HttpRequest,
    id: web::Path<i64>,
    ctx: web::Data<AppContext>,
) -> impl Responder {

    let start = Instant::now();

    let video_id = id.into_inner();

    let uid = req
        .extensions()
        .get::<i64>()
        .copied();

    let query = ViewQuery::new(
        video_id,
        uid,
    );

    let result = view::case_get_video_detail(
        query,
        &ctx,
    )
        .await;

    result.finish(&req, start)
}
////////

/// # [HANDLER] - 查看视频 by video id
pub async fn get_video(id: web::Path<i64>) -> impl Responder {
    let video_id = id.into_inner();

    HttpResponse::Ok().json(json!({
        "video_id": video_id,
        "videos": vec!["第一个视频", "10081.mp4"]
    }))
}

////////

/// # [HANDLER] - 查看评论 by video id
pub async fn get_comments(id: web::Path<i64>) -> impl Responder {
    // web::Path<i64> 解构出内部的 i64
    let video_id = id.into_inner();

    HttpResponse::Ok().json(json!({
        "video_id": video_id,
        "comments": vec!["评论1", "评论2"]
    }))
}
////////

/// # [HANDLER] - 查看弹幕 by video id
pub async fn get_danmakus(id: web::Path<i64>) -> impl Responder {
    // web::Path<i64> 解构出内部的 i64
    let video_id = id.into_inner();

    HttpResponse::Ok().json(json!({
        "video_id": video_id,
        "danmakus": vec!["弹幕1", "弹幕2"]
    }))
}

////////

/// # [HANDLER] - 查看动态 by video id
pub async fn get_dynamics(id: web::Path<i64>) -> impl Responder {
    // web::Path<i64> 解构出内部的 i64
    let video_id = id.into_inner();

    HttpResponse::Ok().json(json!({
        "video_id": video_id,
        "dynamics": vec!["动态1", "动态2"]
    }))
}

////////

/// # [HANDLER] - 查看直播 by video id
pub async fn get_lives(id: web::Path<i64>) -> impl Responder {
    // web::Path<i64> 解构出内部的 i64
    let video_id = id.into_inner();

    HttpResponse::Ok().json(json!({
        "video_id": video_id,
        "lives": vec!["实况1", "实况2"]
    }))
}

////////

/// # [HANDLER] - 查看活动 by video id
pub async fn get_activity(id: web::Path<i64>) -> impl Responder {
    // web::Path<i64> 解构出内部的 i64
    let video_id = id.into_inner();

    HttpResponse::Ok().json(json!({
        "video_id": video_id,
        "videos": vec!["活动1", "活动2"]
    }))
}

//////// END
