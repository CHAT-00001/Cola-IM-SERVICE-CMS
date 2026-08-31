// gate_http/router_v2/video/gateway.rs
// 2026/6/13 10:21

////////

use crate::kits::response::IntoApi;
use crate::ping::ping;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use app_config::app_state::AppState;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_video::api::video::home::HomeApi;
use std::time::Instant;

////////

/// # [ROUTER] - 短视频 - 路由器
pub fn video_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // by
        // * /new/xxxx
        web::scope("/video")
            // 默认
            .route("/", web::get().to(ping))
            // 网关
            .route("/gateway", web::get().to(video_gateway))
            .route("/gateway", web::post().to(video_gateway)),
    );
}

////////

/// # [GATEWAY] - 可乐视频网关
pub async fn video_gateway(
    req: HttpRequest,
    url: web::Query<ApiGatewayRequest>,
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

    let url_req = url.into_inner();
    let mut api_req = if body.is_empty() {
        url_req
    } else {
        let body_value: serde_json::Value = match serde_json::from_slice(&body) {
            Ok(value) => value,
            Err(error) => {
                return AppData::<()>::err(
                    4001,
                    format!("[🌐 GATEWAY]: ❌️ 视频 Body JSON 解析失败: {}", error),
                    None,
                )
                .finish(&req, start);
            }
        };
        // 兼容 Body 直接传网关参数，以及统一协议的 { "cmd": { ... } } 包装格式。
        let request_value = body_value
            .get("cmd")
            .cloned()
            .unwrap_or_else(|| body_value.clone());
        let mut body_req: ApiGatewayRequest = match serde_json::from_value(request_value) {
            Ok(value) => value,
            Err(error) => {
                return AppData::<()>::err(
                    4001,
                    format!("[🌐 GATEWAY]: ❌️ 视频 Body 参数解析失败: {}", error),
                    None,
                )
                .finish(&req, start);
            }
        };
        body_req.body = Some(body_value);
        url_req.merge(body_req)
    };
    api_req.uid = Some(uid);
    api_req = api_req.build();
    let auth = cola_data::auth::info::auth::AuthContext {
        uid,
        access_token: String::new(),
        refresh_token: String::new(),
        device_id: String::new(),
        iam_roles: vec![],
        is_anonymous: false,
    };

    // 🌟 对齐到 service 字符串进行业务路由分发
    match api_req.service.clone().unwrap_or_default().as_str() {
        // 1001 最新
        "home_new" => HomeApi::home_new(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start),

        // 1002 热门
        "home_hot" => HomeApi::home_hot(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start),

        // 1003 推荐
        "home_recommend" => HomeApi::home_recommend(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start),

        // 1004 同城
        "home_city" => HomeApi::home_city(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start),

        // 1005 分类
        "home_category" => HomeApi::home_category(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start),

        // 1006 精选
        "home_featured" => HomeApi::home_featured(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start),

        // 1007 搜索
        "home_search" => HomeApi::home_search(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start),

        "view" => {
            // 查看视频详情 - 测试接口
            let video_id = api_req.video_id;
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
                "video_id": api_req.video_id,
                "content": "示例评论内容"
            });
            AppData::ok(data).finish(&req, start)
        }

        _ => AppData::<()>::err(
            2004,
            format!(
                "[🌐 GATEWAY]: ⚠️ Unknown The [▶ VIDEO] service: {}",
                api_req.service.unwrap_or_default()
            ),
            None,
        )
        .finish(&req, start),
    }
}

//////// END
