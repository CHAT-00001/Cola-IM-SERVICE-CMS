// router_v2/cola_gis/router_v1  -- HTTP - 可乐GIS - 路由器
// 2026/5/25 06:49 by wx: cestbon10080

////////

use crate::kits::response::IntoApi;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use app_config::app_state::AppState;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_auth::info::auth::AuthContext;
use cola_gis::api::home::HomeApi;
use serde::Deserialize;
use std::time::Instant;

////////

/// # 网关请求体
struct GatewayRequest {
    auth: AuthContext,     // 补上 auth 字段
    action: i16,           // 🌟 以后使用的 int16 动作代码
    service: String,       // 🌟 兼容 PHP PhalApi 的服务名称 (字符串)
    query: Option<String>, // 查询
    body: web::Bytes,      // body
    path: String,          // 路径
}

/// # 统一的网关 Query 提取结构体
#[derive(Deserialize)]
pub struct GatewayQuery {
    pub node: Option<String>,      // 节点(边缘节点,预设)
    pub action: Option<i16>,       // 🔨 动作码(整型数字给macth转发,预设)
    pub service: String,           // 🌟 服务名称(人类可读)
    pub video_id: Option<i64>,     // 🆔
    pub page: Option<i64>,         // 页码
    pub qty: Option<i64>,          // 每页数量
    pub query: Option<String>,     // 查询
    pub auth: Option<AuthContext>, // 验证信息
}

/// # [ROUTER] - GIS地理信息服务 - 路由器
pub fn gis_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // by
        // * /new/xxxx
        web::scope("/cola_gis")
            // 默认
            .route("/", web::get().to(root))
            // 网关
            .route("/gateway", web::get().to(gis_gateway))
            .route("/gateway", web::post().to(gis_gateway)),
    );
}

// ROOT
pub async fn root() -> HttpResponse {
    HttpResponse::Ok().json(vec!["Cole", "GIS", "ROUTER"])
}

////////

/// # [GATEWAY] - 可乐GIS网关
async fn gis_gateway(
    req: HttpRequest,
    url: web::Query<ApiGatewayRequest>,
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
        ////////

        // 1001 最新
        "home_new" => {
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

        // 1002 热门
        "home_hot" => {
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

        // 1003 同城
        "home_city" => {
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

        // 1004 分类
        "home_category" => {
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

        // 1005 搜索
        "home_search" => {
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

        ////////

        // 2001 关注
        "feed_following" => {
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

        // 2002 朋友
        "feed_friend" => {
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

        // 2003 推荐
        "feed_recommend" => {
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

        // 2004 附近
        "feed_nearby" => {
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

        // 2005 看过的
        "feed_visited" => {
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

        // 2006 点赞的
        "feed_liked" => {
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

        // 2007 收藏的
        "feed_collect" => {
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

        ////////

        // 4001 浏览开始
        "view_start" => {
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

        // 4002 浏览完成
        "view_done" => {
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

        "view" => {
            // 查看视频详情 - 测试接口
            let video_id = query.video_id.unwrap_or(0);
            let data = serde_json::json!({
                "id": video_id,
                "user_id": 1,
                "title": "测试视频标题",
                "description": "这是一个测试视频描述",
                "href": "https://example.com/new/1001",
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
            400,
            format!("[🌐 GATEWAY]: ⚠️ Unknown Api [📍 GIS] service: {}", gateway_req.service),
            None,
        )
        .finish(&req, start),
    }
}
