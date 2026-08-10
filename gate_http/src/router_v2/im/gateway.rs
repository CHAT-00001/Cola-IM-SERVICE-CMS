// gate_http/src/router_v2/im/gateway.rs
// 🔌 IM - Gateway - 网关主分发器
// 2026/8/8 Created.

////////

use super::{card, chat, contact, message};
use crate::kits::response::IntoApi;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use cola_data::app::data::AppData;
use cola_data::cola_auth::info::auth::AuthContext;
use serde::Deserialize;
use std::time::Instant;
use app_config::app_state::AppState;

////////

/// # 网关请求体
struct GatewayRequest {
    auth: AuthContext,
    action: i16,
    service: String,
    query: Option<String>,
    body: web::Bytes,
    path: String,
}

/// # 统一的 Query 提取结构体
#[derive(Deserialize)]
pub struct GatewayQuery {
    pub service: String,
    pub action: Option<i16>,
    pub video_id: Option<i64>,
    pub page: Option<i64>,
    pub qty: Option<i64>,
}

////////

/// # [ROUTER] - IM 即时通讯 - 网关路由
pub fn im_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cola_im")
            .route("/", web::get().to(root))
            .route("/gateway", web::get().to(im_gateway))
            .route("/gateway", web::post().to(im_gateway)),
    );
}

// ROOT
pub async fn root() -> HttpResponse {
    HttpResponse::Ok().json(vec!["Cole", "IM", "ROUTER"])
}

////////

/// # [GATEWAY] - 可乐 IM 网关 - 主分发器
async fn im_gateway(
    req: HttpRequest,
    query: web::Query<GatewayQuery>,
    body: web::Bytes,
    state: web::Data<AppState>,
) -> impl Responder {
    let start = Instant::now();

    // 获取登录用户 UID
    let uid = match req.extensions().get::<i64>().copied() {
        Some(id) => id,
        None => 1, // 测试环境默认 uid
    };

    let auth = AuthContext {
        uid,
        access_token: String::new(),
        refresh_token: String::new(),
        device_id: String::new(),
        roles: vec![],
        is_anonymous: false,
    };

    let gateway_req = GatewayRequest {
        auth,
        action: query.action.unwrap_or(0),
        service: query.service.clone(),
        query: Some(req.query_string().to_string()),
        body: body.clone(),
        path: req.path().to_string(),
    };

    // 🌟 按模块前缀分发到各个子分发器
    match gateway_req.service.as_str() {
        // 1xxx - Contact (联系人)
        service if service.starts_with("contact_") => {
            contact::contact_dispatch(service, &req, &query, &body, &state, auth, uid, start).await
        }

        // 2xxx - Card (名片)
        service if service.starts_with("card_") => {
            card::card_dispatch(service, &req, &query, &body, &state, auth, uid, start).await
        }

        // 3xxx - Message (消息)
        service if service.starts_with("message_") => {
            message::message_dispatch(service, &req, &query, &body, &state, auth, uid, start).await
        }

        // 4xxx - Chat (聊天)
        service if service.starts_with("chat_") => {
            chat::chat_dispatch(service, &req, &query, &body, &state, auth, uid, start).await
        }

        // 测试接口
        "view" => {
            let video_id = query.video_id.unwrap_or(0);
            let data = serde_json::json!({
                "id": video_id,
                "user_id": 1,
                "title": "测试视频标题",
                "description": "这是一个测试视频描述",
                "href": "https://example.com/video/1001",
                "cover": "https://example.com/cover/1001.jpg",
                "views": 12345,
                "likes": 678,
                "comments": 90,
                "duration": 120.5,
                "width": 1920,
                "height": 1080,
                "status": 1,
                "created_at": "2026-06-12T07:00:00Z"
            });
            AppData::ok(data).finish(&req, start)
        }

        "publish_video" => {
            let data = serde_json::json!({
                "video_id": 12345,
                "user_id": uid,
                "title": "示例视频标题",
                "status": "published"
            });
            AppData::ok(data).finish(&req, start)
        }

        "publish_comment" => {
            let data = serde_json::json!({
                "comment_id": 67890,
                "user_id": uid,
                "video_id": query.video_id.unwrap_or(0),
                "content": "示例评论内容"
            });
            AppData::ok(data).finish(&req, start)
        }

        _ => AppData::<()>::err(
            400,
            format!("[🌐 GATEWAY]: ⚠️ Unknown Api [💬 IM] service: {}", gateway_req.service),
            None,
        )
        .finish(&req, start),
    }
}

//////// END
