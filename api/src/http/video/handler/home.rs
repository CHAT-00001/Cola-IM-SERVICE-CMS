// api http/video/handler/video  -- 短视频 - handler - 主页
// 2026/5/19 16:16 by wx: cestbon10080

////////

use crate::kits::response::IntoApi;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web, Scope};
use app_core::ctx::AppContext;
use app_core::video::app::home;
use data::app::request::ApiUrlParamsQuery;
use serde_json::json;
use std::time::Instant;
use tracing::info;
use app_config::app_state::AppState;
////////


/// # [HANDLER] - 查看弹幕 by video id
pub async fn get_home_new() -> HttpResponse {
    HttpResponse::Ok().json(vec!["最新视频1", "最新视频2"])
}

/// # [HANDLER] - 最新
pub async fn home_new(
    req: HttpRequest,
    query: web::Query<ApiUrlParamsQuery>,
    ctx: web::Data<AppContext>,
) -> impl Responder {
    let start = Instant::now();

    info!("[Handler] - [video] - [home] ------------------------------------");

    let uid = req.extensions().get::<i64>().copied();

    let query_params = query.into_inner().build();

    let result = home::case_home_new(
        query_params,
        &ctx,
    )
        .await;

    result.finish(&req, start)
}

////////

/// # [HANDLER] - 热门
pub async fn home_hot(
    req: HttpRequest,
    query: web::Query<ApiUrlParamsQuery>,
    ctx: web::Data<AppContext>,
) -> impl Responder {

    let start = Instant::now();

    info!("[Handler] - [video] - [home]");

    let result = home::case_home_hotlist(
        query.into_inner().build(),
        &ctx,
    )
        .await;

    result.finish(&req, start)
}

////////

/// # [HANDLER] - 同城
pub async fn home_city(id: web::Path<i64>) -> impl Responder {
    let city_id = id.into_inner();

    HttpResponse::Ok().json(json!({
        "city_id": city_id,
        "videos": vec!["同城视频1", "同城视频2"]
    }))
}

////////

/// # [HANDLER] - 频道
pub async fn home_cate(id: web::Path<i64>) -> impl Responder {
    // web::Path<i64> 解构出内部的 i64
    let channel_id = id.into_inner();

    HttpResponse::Ok().json(json!({
        "channel_id": channel_id,
        "videos": vec!["频道视频1", "频道视频2"]
    }))
}

////////

/// # [HANDLER] - 搜索
pub async fn home_search(kw: web::Path<String>) -> impl Responder {
    // web::Path<i64> 解构出内部的 i64
    let keyword = kw.into_inner();

    HttpResponse::Ok().json(json!({
        "关键词": keyword,
        "videos": vec!["搜索视频1", "搜索视频2"]
    }))
}

//////// END
