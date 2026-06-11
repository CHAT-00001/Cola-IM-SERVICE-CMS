// /router2  --
// 2026/5/25 06:49 by wx: cestbon10080

////////

use std::io::Bytes;
use crate::kits::response::IntoApi;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use std::time::Instant;
use serde::Deserialize;
use cola_data::app::data::AppData;
use cola_data::auth::info::auth::AuthContext;

////////

/// # 网关请求体
struct GatewayRequest {
    action: String,
    query: Option<String>,
    body: web::Bytes,
    path: String,
}
/// # 统一的 Query 提取结构体（假设 action 通过 query 传递，如 /gate_grpc?action=get_categories）
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
            .route("/", web::get().to(get_categories))
            // 验证网关
            .route("/gateway", web::get().to(feed_gateway)),
    );
}

// 分类相关
pub async fn get_categories() -> HttpResponse {
    HttpResponse::Ok().json(vec!["Action", "Comedy", "Drama"])
}

pub async fn get_videos_by_category() -> HttpResponse {
    HttpResponse::Ok().json(vec!["Video1", "Video2"])
}

////////

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