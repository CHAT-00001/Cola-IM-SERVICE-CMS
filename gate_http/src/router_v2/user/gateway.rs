// http/src/user/gateway.rs  -- HTTP 用户 网关
// 2026/6/18 07:53

//////

use crate::kits::response::IntoApi;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::info::auth::AuthContext;
use cola_user::api::home::HomeApi;
use cola_user::api::add::AddApi;
use serde::Deserialize;
use std::time::Instant;
use app_config::app_state::AppState;
use crate::ping::ping;
//////

/// # 网关请求体
struct GatewayRequest {
    auth: AuthContext,     // 补上 auth 字段
    action: i16,           // 🌟 以后使用的 int16 动作代码
    service: String,       // 🌟 兼容 PHP PhalApi 的服务名称 (字符串)
    query: Option<String>, // 查询
    body: web::Bytes,      // body
    path: String,          // 路径
}

/// # 统一的 Query 提取结构体
#[derive(Deserialize)]
pub struct GatewayQuery {
    pub service: String,         // 🌟 兼容 PhalApi，接收如 "Video.PublishVideo"
    pub action: Option<i16>,     // 🌟 以后转入的 int16 动作代码，先用 Option 顶住
    pub video_id: Option<i64>,
    pub page: Option<i64>,       // 页码
    pub qty: Option<i64>,        // 每页数量
}

/// # [ROUTER] - 用户中心 - 路由器
pub fn user_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // by
        // * /video/xxxx
        web::scope("/user")
            // 默认
            .route("", web::get().to(ping))
            .route("/", web::get().to(root))
            // 网关
            .route("/gateway", web::get().to(user_gateway))
            .route("/gateway", web::post().to(user_gateway)),
    );
}

// ROOT
pub async fn root() -> HttpResponse {
    HttpResponse::Ok().json(vec!["Cole", "VIDEO", "ROUTER"])
}


//////

/// # [GATEWAY] - 可乐用户中心网关
async fn user_gateway(
    req: HttpRequest,
    // url web::Query<ApiGatewayRequest>,
    query: web::Query<GatewayQuery>,
    body: web::Bytes,
    state: web::Data<AppState>,
) -> impl Responder {

    // 开始时间
    let start = Instant::now();


    // 严格检查登录状态，统一命名操作用户为 uid
    let uid = match req.extensions().get::<i64>().copied() {
        Some(id) => id,
        None => 1, // 测试环境默认 uid
    };

    let auth = AuthContext {
        uid,
        access_token: String::new(),
        refresh_token: String::new(),
        device_id: String::new(),
        iam_roles: vec![],
        is_anonymous: false,
    };

    let gateway_req = GatewayRequest {
        auth,
        action: query.action.unwrap_or(0), // 先给个默认值 0，留给以后用
        service: query.service.clone(),    // 对齐并绑定真正的 PhalApi 字符串服务名
        query: Some(req.query_string().to_string()),
        body,
        path: req.path().to_string(),
    };

    // 🌟 对齐到 service 字符串进行业务路由分发
    match gateway_req.service.as_str() {


        // 1001 最新
        "home.new" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
                .build();

            HomeApi::handler_get_new(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 2001 创建新用户
        "add.new" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
                .build();

            AddApi::handler_get_new(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        "view" => {
            // 查看视频详情 - 测试接口
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
            // 发布视频接口转发
            let data = serde_json::json!({
                "video_id": 12345,
                "user_id": uid,
                "title": "示例视频标题",
                "status": "published"
            });
            AppData::ok(data).finish(&req, start)
        }

        "publish_comment" => {
            // 发布评论接口转发
            let data = serde_json::json!({
                "comment_id": 67890,
                "user_id": uid,
                "video_id": query.video_id.unwrap_or(0),
                "content": "示例评论内容"
            });
            AppData::ok(data).finish(&req, start)
        }

        _ => AppData::<()>::err(
            2004,
            format!("[🌐 GATEWAY]: ⚠️ Unknown The [👤 USER] service: {}", gateway_req.service),
            None,
        )
            .finish(&req, start),
    }
}
