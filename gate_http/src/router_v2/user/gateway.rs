// gate_http/src/user/gateway.rs -- HTTP 用户 网关
// 2026/6/18 07:53

//////

use crate::kits::response::IntoApi;
use crate::ping::ping;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use app_config::app_state::AppState;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::info::auth::AuthContext;
use cola_gis::api::home::HomeApi;
use cola_user::api::user::add::UserAddApi;
use cola_user::api::user::history::{UpdateProfileRequest, UserHistoryApi};
use std::time::Instant;
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

/// # [ROUTER] - 用户中心 - 路由器
pub fn user_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // by
        // * /new/xxxx
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
    HttpResponse::Ok().json(vec!["Cole", "USER CENTER", "ROUTER"])
}

//////

/// # [GATEWAY] - 可乐用户中心网关
async fn user_gateway(
    req: HttpRequest,
    // url web::Query<ApiGatewayRequest>,
    query: web::Query<ApiGatewayRequest>,
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
        permission_context: None,
    };

    let url_req = query.into_inner();
    let body_req = if body.is_empty() {
        ApiGatewayRequest::default()
    } else {
        match serde_json::from_slice::<ApiGatewayRequest>(&body) {
            Ok(request) => request,
            Err(error) => {
                return AppData::<()>::err(4001, format!("网关 JSON 参数无效: {error}"), None)
                    .finish(&req, start);
            }
        }
    };
    let mut query = url_req.merge(body_req);
    // URL/body 中的 UID 均不可信，自助接口只使用 session 注入的身份。
    query.uid = Some(uid);

    let gateway_req = GatewayRequest {
        auth,
        action: query.action.unwrap_or(0),
        service: query.service.clone().unwrap_or_default(),
        query: Some(req.query_string().to_string()),
        body,
        path: req.path().to_string(),
    };

    ////////

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

            UserAddApi::api_add_new(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        //////// AVATAR

        // 修改头像
        "edit_avatar" => {
            let request = match serde_json::from_slice::<UpdateProfileRequest>(&gateway_req.body) {
                Ok(request) => request,
                Err(error) => {
                    return AppData::<()>::err(4001, format!("请求参数无效: {error}"), None)
                        .finish(&req, start);
                }
            };
            UserHistoryApi::update(
                uid,
                UpdateProfileRequest {
                    nickname: None,
                    signature: None,
                    bg_img: None,
                    sns_url: None,
                    email: None,
                    phone: None,
                    birthday: None,
                    lat: None,
                    lng: None,
                    ..request
                },
                &state.ctx,
            )
            .await
            .finish(&req, start)
        }

        // 修改昵称
        "update_nickname" => {
            let request = match serde_json::from_slice::<UpdateProfileRequest>(&gateway_req.body) {
                Ok(request) => request,
                Err(error) => {
                    return AppData::<()>::err(4001, format!("请求参数无效: {error}"), None)
                        .finish(&req, start);
                }
            };
            UserHistoryApi::update(
                uid,
                UpdateProfileRequest {
                    avatar: None,
                    avatar_thumb: None,
                    signature: None,
                    bg_img: None,
                    sns_url: None,
                    email: None,
                    phone: None,
                    birthday: None,
                    lat: None,
                    lng: None,
                    ..request
                },
                &state.ctx,
            )
            .await
            .finish(&req, start)
        }

        // 修改资料
        "update_profile" => {
            let request = match serde_json::from_slice::<UpdateProfileRequest>(&gateway_req.body) {
                Ok(request) => request,
                Err(error) => {
                    return AppData::<()>::err(4001, format!("请求参数无效: {error}"), None)
                        .finish(&req, start);
                }
            };
            UserHistoryApi::update(uid, request, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 头像列表
        "get_avatar_list" | "user.nickname.history.list" => {
            let is_avatar = gateway_req.service == "user.avatar.history.list";
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();
            UserHistoryApi::list(uid, is_avatar, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 从记录中启用头像
        "activate_avatar" | "user.nickname.activate" => {
            let is_avatar = gateway_req.service == "user.avatar.history.activate";
            let id = query.id;
            UserHistoryApi::activate(uid, is_avatar, id, &state.ctx)
                .await
                .finish(&req, start)
        }

        "user.avatar.history.delete" | "user.nickname.history.delete" => {
            let is_avatar = gateway_req.service == "user.avatar.history.delete";
            let id = query.id;
            UserHistoryApi::delete(uid, is_avatar, id, &state.ctx)
                .await
                .finish(&req, start)
        }

        ////////

        // 测试路由(不可删除)
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

        // 测试路由(不可删除)
        "publish_comment" => {
            // 发布评论接口转发
            let data = serde_json::json!({
                "comment_id": 67890,
                "user_id": uid,
                "video_id": query.video_id,
                "content": "示例评论内容"
            });
            AppData::ok(data).finish(&req, start)
        }

        ////////

        // 兜底路由
        _ => AppData::<()>::err(
            2004,
            format!(
                "[🌐 GATEWAY]: ⚠️ Unknown The [👤 USER] service: {}",
                gateway_req.service
            ),
            None,
        )
        .finish(&req, start),
    }
}

//////// END
