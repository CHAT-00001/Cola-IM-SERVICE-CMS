// /router2  --
// 2026/5/25 06:49 by wx: cestbon10080

////////

use std::io::Bytes;
use crate::kits::response::IntoApi;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use app_core::ctx::AppContext;
use app_core::video::app::view::ViewQuery;
use app_core::video::app::{view};
use data::auth::info::auth::AuthContext;
use std::time::Instant;
use serde::Deserialize;
use data::app::data::AppData;

////////

/// # 网关请求体
struct GatewayRequest {
    action: String,
    query: Option<String>,
    body: web::Bytes,
    path: String,
}
/// # 统一的 Query 提取结构体（假设 action 通过 query 传递，如 /gateway?action=get_categories）
#[derive(Deserialize)]
pub struct GatewayQuery {
    pub action: String,
    pub video_id: Option<i64>,
}
/// # [ROUTER] - 短视频 - 路由器
pub fn video_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // by
        // * /video/xxxx
        web::scope("/video")
            // 公共网关
            .route("/gateway1", web::get().to(public_gateway))
            // 验证网关
            .route("/gateway2", web::get().to(feed_gateway)),
    );
}

// 分类相关
pub async fn get_categories() -> HttpResponse {
    HttpResponse::Ok().json(vec!["Action", "Comedy", "Drama"])
}

pub async fn get_videos_by_category() -> HttpResponse {
    HttpResponse::Ok().json(vec!["Video1", "Video2"])
}


// # HANDLER - 公共网关 (不需要登录)
/// * 路径: /video/gateway1
async fn public_gateway(
    req: HttpRequest,
    query: web::Query<GatewayQuery>,
    _body: web::Bytes,
) -> impl Responder {
    let start = Instant::now();

    // 🫡 遵照指令：统一命名为 uid，没登录就是 0！
    let uid: i64 = req.extensions().get::<i64>().cloned().unwrap_or(0);

    // 统一处理所有 action
    match query.action.as_str() {

        // 公共接口 - 视频详情
        "video_detail" => {
            let video_id = match query.video_id {
                Some(id) => id,
                None => return AppData::<()>::err(400, "缺少 video_id 参数", None).finish(&req, start)
            };

            let backend_uid = if uid == 0 { None } else { Some(uid) };
            let view_query = ViewQuery::new(video_id, backend_uid);

            // 直接调用 case_get_video_detail，它返回 AppData<VideoSingleResponse>
            let result = view::case_get_video_detail(view_query).await;

            // result 本身就是 AppData，调用 finish 计时
            result.finish(&req, start)
        },

        _ => HttpResponse::BadRequest().json("Unknown public action"),
    }
}


/// # [HANDLER] - 验证网关 (走验证通道)
async fn feed_gateway(
    req: HttpRequest,
    query: web::Query<GatewayQuery>,
    body: web::Bytes,
    ctx: web::Data<AppContext>,
) -> impl Responder {
    let start = Instant::now();

    // 严格检查登录状态，统一命名操作用户为 uid
    let uid = match req.extensions().get::<i64>().copied() {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().json("401 Unauthorized"),
    };

    let auth = AuthContext {
        user_id: Some(uid),
        access_token: String::new(),
        refresh_token: String::new(),
        device_id: String::new(),
        roles: vec![],
        is_anonymous: false,
    };

    let gateway_req = GatewayRequest {
        auth,
        action: query.action.clone(),
        query: Some(req.query_string().to_string()),
        body,
        path: req.path().to_string(),
    };

    // 分发不同的业务 case
    match gateway_req.action.as_str() {
        "publish_video" => {
            // 发布视频接口转发
            let data = serde_json::json!({
                "video_id": 12345,
                "user_id": uid,
                "title": "示例视频标题",
                "status": "published"
            });
            AppData::ok(data).finish(&req, start)
        },

        "publish_comment" => {
            // 发布评论接口转发
            let data = serde_json::json!({
                "comment_id": 67890,
                "user_id": uid,
                "video_id": query.video_id.unwrap_or(0),
                "content": "示例评论内容"
            });
            AppData::ok(data).finish(&req, start)
        },

        _ => {
            AppData::<()>::err(400, format!("Unknown auth action: {}", gateway_req.action), None)
                .finish(&req, start)
        }
    }
}