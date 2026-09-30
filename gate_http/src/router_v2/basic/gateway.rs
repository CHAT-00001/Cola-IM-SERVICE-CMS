// gate_http/src/router_v2/basic/gateway.rs -- HTTP 网关 - BASIC - CRUD 命令网关
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
use service::cola_user::permission::query::UserPermissionQueryService;
use std::time::Instant;
use tracing::{info, warn};

////////

const BASIC_PERMISSION_LEVEL: i16 = 2;

////////

/// # [ROUTER] - BASIC 命令网关
/// * `desc`: `提供 app、node、service 的 CRUD 命令入口`
pub fn basic_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/basic")
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

/// # [GATEWAY] - 认证并构造权限上下文
/// * `desc`: `复用视频网关会话校验方式，BASIC CRUD 要求权限等级 >= 2`
async fn authenticate(
    api_req: &ApiGatewayRequest,
    state: &AppState,
) -> Result<AuthContext, AppData<()>> {
    let auth_request = api_req.auth.clone().unwrap_or_default();
    if !auth_request.has_token() {
        return Err(AppData::err(
            4003,
            "[🌐 GATEWAY]: ❌️ 权限不足：需要权限 >= 2",
            None,
        ));
    }

    let session = match SessionStateApi::verify_session(&auth_request, &state.ctx.auth).await {
        AppData {
            data: Some(session),
            ..
        } if !session.is_anonymous => session,
        _ => {
            warn!("[🌐 GATEWAY]: ❌️ BASIC 会话校验失败");
            return Err(AppData::err(4003, "[🌐 GATEWAY]: ❌️ 会话无效", None));
        }
    };

    let permission_context =
        UserPermissionQueryService::resolve_permission_context(session.uid).await;
    let auth = AuthContext {
        uid: session.uid,
        access_token: session.access_token,
        refresh_token: auth_request.refresh_token.unwrap_or_default(),
        device_id: session.device_id,
        iam_roles: session.iam_roles,
        is_anonymous: session.is_anonymous,
        permission_context: Some(permission_context),
    };

    if !auth.has_permission_level(BASIC_PERMISSION_LEVEL) {
        return Err(AppData::err(
            4003,
            "[🌐 GATEWAY]: ❌️ 权限不足：需要权限 >= 2",
            None,
        ));
    }

    Ok(auth)
}

////////

/// # [GATEWAY] - BASIC 命令网关
/// * `desc`: `Body(JSON) 优先于 URL 参数，按 service/action 分发 CRUD`
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

    let auth = match authenticate(&api_req, &state).await {
        Ok(auth) => auth,
        Err(error) => return error.finish(&req, start),
    };
    api_req.uid = Some(auth.uid);
    api_req = api_req.build();

    let service = api_req.service.clone().unwrap_or_default();
    info!(
        "[🌐 GATEWAY]: ✅️ BASIC 命令分发: service={}, uid={}",
        service, auth.uid
    );

    match service.as_str() {
        "app_list" | "app_get" | "app_add" | "app_del" | "app_search" => {
            dispatch_app(&req, start, &api_req, &state, &auth).await
        }
        "binding_list" | "binding_get" | "binding_add" | "binding_del" => {
            dispatch_binding(&req, start, &api_req, &state, &auth).await
        }
        "node_list" | "node_get" | "node_add" | "node_del" | "node_status" => {
            dispatch_node(&req, start, &api_req, &state, &auth).await
        }
        "service" => not_implemented(&req, start, "service CRUD 下层 CASE 尚未接入"),
        _ => AppData::<()>::err(
            4000,
            format!("[🌐 GATEWAY]: ⚠️ Unknown BASIC service: {service}"),
            None,
        )
        .finish(&req, start),
    }
}

////////

async fn dispatch_app(
    req: &HttpRequest,
    start: Instant,
    api_req: &ApiGatewayRequest,
    state: &AppState,
    auth: &AuthContext,
) -> HttpResponse {
    match api_req.service.as_deref().unwrap_or_default() {
        "app_list" => AppApi::api_get_app_list(api_req.clone(), &state.ctx)
            .await
            .finish(req, start),
        "app_get" => {
            let app_id = api_req.params.get("app_id").cloned().unwrap_or_default();
            AppApi::api_get_app(app_id, &state.ctx)
                .await
                .finish(req, start)
        }
        "app_add" => match parse_body_cmd(api_req) {
            Ok(cmd) => AppApi::api_add_app(auth.uid, cmd, &state.ctx)
                .await
                .finish(req, start),
            Err(error) => AppData::<()>::err(4001, error, None).finish(req, start),
        },
        "app_del" => AppApi::api_del_app(auth.uid, api_req.id, &state.ctx)
            .await
            .finish(req, start),
        "app_search" => AppApi::api_search_app(auth.uid, api_req.keyword.clone(), &state.ctx)
            .await
            .finish(req, start),
        _ => not_implemented(req, start, "app CRUD action 未定义"),
    }
}

////////

async fn dispatch_node(
    req: &HttpRequest,
    start: Instant,
    api_req: &ApiGatewayRequest,
    state: &AppState,
    auth: &AuthContext,
) -> HttpResponse {
    match api_req.service.as_deref().unwrap_or_default() {
        "node_list" => NodeApi::api_get_node_list(
            api_req.params.get("app_id").cloned(),
            api_req.limit,
            api_req.offset,
            &state.ctx,
        )
        .await
        .finish(req, start),
        "node_get" => {
            let node_id = api_req.params.get("node_id").cloned().unwrap_or_default();
            NodeApi::api_get_node(node_id.parse().unwrap(), &state.ctx)
                .await
                .finish(req, start)
        }
        "node_add" => not_implemented(req, start, "node create 参数模型仍复用临时 CDN 结构"),
        "node_del" => NodeApi::api_delete_node(auth.uid, api_req.id, &state.ctx)
            .await
            .finish(req, start),
        "node_status" => {
            let status = api_req
                .params
                .get("status")
                .and_then(|value| value.parse().ok())
                .unwrap_or(1);
            NodeApi::api_change_node_status(auth.uid, api_req.id, status, &state.ctx)
                .await
                .finish(req, start)
        }
        _ => not_implemented(req, start, "node CRUD action 未定义"),
    }
}

////////

async fn dispatch_binding(
    req: &HttpRequest,
    start: Instant,
    api_req: &ApiGatewayRequest,
    state: &AppState,
    auth: &AuthContext,
) -> HttpResponse {
    match api_req.service.as_deref().unwrap_or_default() {
        "binding_list" => BindingApi::api_get_binding_list(api_req.clone(), &state.ctx)
            .await
            .finish(req, start),
        "binding_get" => {
            let app_id = api_req.params.get("app_id").cloned().unwrap_or_default();
            BindingApi::api_get_binding(app_id, &state.ctx)
                .await
                .finish(req, start)
        }
        "binding_add" => match parse_body_cmd(api_req) {
            Ok(cmd) => BindingApi::api_add_binding(auth.uid, cmd, &state.ctx)
                .await
                .finish(req, start),
            Err(error) => AppData::<()>::err(4001, error, None).finish(req, start),
        },
        "binding_del" => BindingApi::api_delete_binding(auth.uid, api_req.id, &state.ctx)
            .await
            .finish(req, start),
        _ => not_implemented(req, start, "binding CRUD action 未定义"),
    }
}

////////

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

fn not_implemented(req: &HttpRequest, start: Instant, message: &str) -> HttpResponse {
    AppData::<()>::err(5010, format!("[🌐 GATEWAY]: ⚠️ {message}"), None).finish(req, start)
}

//////// END
