// api http/video/handler/del.rs  -- 删除
// 2026/5/25 04:08 by wx: cestbon10080

////////

use actix_web::web::ServiceConfig;
use actix_web::{HttpResponse, Scope, web};
use serde_json::json;

////////

/// # [HANDLER] - 删除视频
pub async fn del_video() -> HttpResponse {
    HttpResponse::Ok().json(vec!["删除视频", "successfully"])
}

////////

/// # [HANDLER] - 删除评论
pub async fn del_comment() -> HttpResponse {
    HttpResponse::Ok().json(vec!["删除评论", "successfully"])
}

////////

/// # [HANDLER] - 删除弹幕
pub async fn del_danmaku() -> HttpResponse {
    HttpResponse::Ok().json(vec!["删除弹幕", "successfully"])
}

////////

/// # [HANDLER] - 删除分享
pub async fn del_share() -> HttpResponse {
    HttpResponse::Ok().json(vec!["删除分享", "successfully"])
}

////////

/// # [HANDLER] - 删除推荐
pub async fn del_recommend() -> HttpResponse {
    HttpResponse::Ok().json(vec!["删除推荐", "successfully"])
}

////////

/// # [HANDLER] - 删除上热门
pub async fn del_hotlist() -> HttpResponse {
    HttpResponse::Ok().json(vec!["删除上热门", "successfully"])
}
