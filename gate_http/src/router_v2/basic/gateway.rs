// gate_http/src/router_v2/aaaa/gateway.rs -- HTTP 网关 - BASIC - 基础管理业务网关
// 2026/09/30 10:30 Created.

////////

use crate::kits::response::IntoApi;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use app_config::app_state::AppState;
use cola_aaaa::api::app::AppApi;
use cola_aaaa::api::binding::BindingApi;
use cola_aaaa::api::node::NodeApi;
use cola_auth::api::seesion::state::SessionStateApi;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::info::auth::AuthContext;
use cola_data::auth::request::session::SessionContext;
use service::cola_user::permission::query::UserPermissionQueryService;
use std::time::Instant;
use tracing::{info, warn};

////////

/// # [GATEWAY] - 解析 BASIC 命令 Body
/// * `desc`: `兼容 Body 直接传命令和 { cmd: {...} } 包装格式`
fn parse_body_cmd<T>(api_req: &ApiGatewayRequest) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    let body = api_req
        .body
        .clone()
        .ok_or_else(|| "[🌐 GATEWAY]: ❌️ 缺少 JSON Body".to_string())?;
    let value = body.get("cmd").cloned().unwrap_or(body);
    serde_json::from_value(value)
        .map_err(|error| format!("[🌐 GATEWAY]: ❌️ cmd 参数解析失败: {error}"))
}

////////

/// # [GATEWAY] - BASIC 未实现能力兜底
fn not_implemented(req: &HttpRequest, start: Instant, message: &str) -> HttpResponse {
    AppData::<()>::err(5010, format!("[🌐 GATEWAY]: ⚠️ {message}"), None).finish(req, start)
}

////////

const BASIC_PERMISSION_LEVEL: i16 = 2;

////////

/// # [ROUTER] - BASIC 命令网关
/// * `desc`: `提供 app、binding、node 的 CRUD 命令入口`
pub fn basic_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // 🌟 BASIC 基础管理路由
        web::scope("/aaaa")
            .route("/", web::get().to(root))
            .route("/gateway", web::get().to(basic_gateway))
            .route("/gateway", web::post().to(basic_gateway)),
    );
}

////////

/// # [GATEWAY] - BASIC 根入口
pub async fn root() -> HttpResponse {
    HttpResponse::Ok().json(vec!["Cola", "BASIC", "GATEWAY"])
}

////////

/// # [GATEWAY] - BASIC 命令网关
/// * `desc`: `Body(JSON) 优先于 URL 参数，在单个 match 中按业务类别分发`
/// * `condition`: `无 auth 或无 token 时使用游客身份，仅用于伪鉴权和路由联调`
pub async fn basic_gateway(
    req: HttpRequest,
    url: web::Query<ApiGatewayRequest>,
    body: web::Bytes,
    state: web::Data<AppState>,
) -> impl Responder {
    let start = Instant::now();
    let url_req = url.into_inner();
    let mut api_req = if body.is_empty() {
        url_req
    } else {
        let body_value: serde_json::Value = match serde_json::from_slice(&body) {
            Ok(value) => value,
            Err(error) => {
                return AppData::<()>::err(
                    4001,
                    format!("[🌐 GATEWAY]: ❌️ BASIC Body JSON 解析失败: {error}"),
                    None,
                )
                .finish(&req, start);
            }
        };
        let mut body_req: ApiGatewayRequest = match serde_json::from_value(body_value.clone()) {
            Ok(value) => value,
            Err(error) => {
                return AppData::<()>::err(
                    4001,
                    format!("[🌐 GATEWAY]: ❌️ BASIC Body 参数解析失败: {error}"),
                    None,
                )
                .finish(&req, start);
            }
        };
        body_req.body = Some(body_value);
        url_req.merge(body_req)
    };

    // 🌐 GATEWAY - 伪鉴权：真实 token 能校验则使用真实身份，否则降级为游客。
    let auth_request = api_req.auth.clone().unwrap_or_default();
    let (session, permission_context) = if auth_request.has_token() {
        match SessionStateApi::verify_session(&auth_request, &state.ctx.auth).await {
            AppData {
                data: Some(session),
                ..
            } if !session.is_anonymous => {
                let context =
                    UserPermissionQueryService::resolve_permission_context(session.uid).await;
                (session, context)
            }
            _ => {
                warn!("[🌐 GATEWAY]: ❌️ BASIC 会话无效，降级为游客访问");
                let guest = SessionContext {
                    uid: 0,
                    iam_roles: vec![],
                    device_id: String::new(),
                    is_anonymous: true,
                    access_token: String::new(),
                };
                let context = UserPermissionQueryService::resolve_permission_context(0).await;
                (guest, context)
            }
        }
    } else {
        info!("[🌐 GATEWAY]: 👤 BASIC 未携带 token，按游客访问处理 - 权限等级=1");
        let guest = SessionContext {
            uid: 0,
            iam_roles: vec![],
            device_id: String::new(),
            is_anonymous: true,
            access_token: String::new(),
        };
        let context = UserPermissionQueryService::resolve_permission_context(0).await;
        (guest, context)
    };
    let auth = AuthContext {
        uid: session.uid,
        access_token: session.access_token,
        refresh_token: auth_request.refresh_token.clone().unwrap_or_default(),
        device_id: session.device_id,
        iam_roles: session.iam_roles,
        is_anonymous: session.is_anonymous,
        permission_context: Some(permission_context),
    };
    api_req.uid = Some(auth.uid);
    api_req = api_req.build();
    let service = api_req.service.clone().unwrap_or_default();
    info!(
        "[🌐 GATEWAY]: ✅️ BASIC 命令分发: service={}, uid={}, anonymous={}",
        service, auth.uid, auth.is_anonymous
    );

    //////// APP - 应用管理

    match service.as_str() {
        //////// APP

        // 💡 - APP 列表
        "app_list" => {
            if !auth.has_permission_level(1) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足", None)
                    .finish(&req, start);
            }
            AppApi::api_get_app_list(api_req.clone(), &state.ctx)
                .await
                .finish(&req, start)
        }

        // 💡 - APP 详情
        "app_get" => {
            if !auth.has_permission_level(1) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足", None)
                    .finish(&req, start);
            }
            let app_id = api_req.params.get("app_id").cloned().unwrap_or_default();
            AppApi::api_get_app(app_id, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 💡 - 添加
        "app_add" => {
            if !auth.has_permission_level(BASIC_PERMISSION_LEVEL) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足：需要权限 >= 2", None)
                    .finish(&req, start);
            }
            let cmd = match parse_body_cmd(&api_req) {
                Ok(cmd) => cmd,
                Err(error) => return AppData::<()>::err(4001, error, None).finish(&req, start),
            };
            AppApi::api_add_app(auth.uid, cmd, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 💡 - 删除
        "app_del" => {
            if !auth.has_permission_level(BASIC_PERMISSION_LEVEL) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足：需要权限 >= 2", None)
                    .finish(&req, start);
            }
            AppApi::api_del_app(auth.uid, api_req.id, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 💡 - 搜索
        "app_search" => {
            if !auth.has_permission_level(1) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足", None)
                    .finish(&req, start);
            }
            AppApi::api_search_app(auth.uid, api_req.keyword.clone(), &state.ctx)
                .await
                .finish(&req, start)
        }

        //////// BINDING - 应用绑定管理

        // 💡 - 绑定列表
        "binding_list" => {
            if !auth.has_permission_level(1) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足", None)
                    .finish(&req, start);
            }
            BindingApi::api_get_binding_list(api_req.clone(), &state.ctx)
                .await
                .finish(&req, start)
        }

        // 💡 - 详情
        "binding_get" => {
            if !auth.has_permission_level(1) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足", None)
                    .finish(&req, start);
            }
            let app_id = api_req.params.get("app_id").cloned().unwrap_or_default();
            BindingApi::api_get_binding(app_id, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 💡 - 添加绑定
        "binding_add" => {
            if !auth.has_permission_level(BASIC_PERMISSION_LEVEL) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足：需要权限 >= 2", None)
                    .finish(&req, start);
            }
            let cmd = match parse_body_cmd(&api_req) {
                Ok(cmd) => cmd,
                Err(error) => return AppData::<()>::err(4001, error, None).finish(&req, start),
            };
            BindingApi::api_add_binding(auth.uid, cmd, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 💡 - 删除绑定
        "binding_del" => {
            if !auth.has_permission_level(BASIC_PERMISSION_LEVEL) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足：需要权限 >= 2", None)
                    .finish(&req, start);
            }
            BindingApi::api_delete_binding(auth.uid, api_req.id, &state.ctx)
                .await
                .finish(&req, start)
        }

        //////// NODE - 节点管理

        // 🌐 - NODE - 节点列表
        "node_list" => {
            if !auth.has_permission_level(1) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足", None)
                    .finish(&req, start);
            }
            NodeApi::api_get_node_list(
                api_req.params.get("app_id").cloned(),
                api_req.limit,
                api_req.offset,
                &state.ctx,
            )
            .await
            .finish(&req, start)
        }

        // 💡 - 详情
        "node_get" => {
            if !auth.has_permission_level(1) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足", None)
                    .finish(&req, start);
            }
            let node_id = match api_req
                .params
                .get("node_id")
                .and_then(|value| value.parse().ok())
            {
                Some(value) => value,
                None => {
                    return AppData::<()>::err(4001, "[🌐 GATEWAY]: ❌️ node_id 参数无效", None)
                        .finish(&req, start);
                }
            };
            NodeApi::api_get_node(node_id, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 💡 - 添加
        "node_add" => {
            if !auth.has_permission_level(BASIC_PERMISSION_LEVEL) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足：需要权限 >= 2", None)
                    .finish(&req, start);
            }
            not_implemented(&req, start, "node create 参数模型仍复用临时 CDN 结构")
        }

        // 💡 - 删除
        "node_del" => {
            if !auth.has_permission_level(BASIC_PERMISSION_LEVEL) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足：需要权限 >= 2", None)
                    .finish(&req, start);
            }
            NodeApi::api_delete_node(auth.uid, api_req.id, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 💡 - 状态
        "node_status" => {
            if !auth.has_permission_level(BASIC_PERMISSION_LEVEL) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足：需要权限 >= 2", None)
                    .finish(&req, start);
            }
            let status = api_req
                .params
                .get("status")
                .and_then(|value| value.parse().ok())
                .unwrap_or(1);
            NodeApi::api_change_node_status(auth.uid, api_req.id, status, &state.ctx)
                .await
                .finish(&req, start)
        }

        //////// SERVICE / UNKNOWN - 兜底
        "service" => not_implemented(&req, start, "service CRUD 下层 CASE 尚未接入"),
        _ => AppData::<()>::err(
            4000,
            format!("[🌐 GATEWAY]: ⚠️ Unknown BASIC service: {service}"),
            None,
        )
        .finish(&req, start),
    }
}

//////// END
