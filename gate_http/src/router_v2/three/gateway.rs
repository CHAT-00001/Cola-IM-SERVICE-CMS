// gate_http/src/router_v2/cola_three/gateway.rs  -- 第三方服务 网关
// 2026/7/27 11:40

////////

use crate::kits::response::IntoApi;
use crate::ping::ping;
use crate::router_v2::three::dispatcher;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use app_config::app_state::AppState;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::request::session::SessionContext;
use std::time::Instant;

////////

/// # [ROUTER] - 第三方服务路由器
pub fn three_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cola_three")
            .route("/", web::get().to(ping))
            .route("/gateway", web::get().to(three_gateway))
            .route("/gateway", web::post().to(three_gateway)),
    );
}

////////

/// # [GATEWAY] - 按 service name 分发到 dispatcher 子模块
async fn three_gateway(
    req: HttpRequest,
    query: web::Query<ApiGatewayRequest>,
    body: web::Bytes,
    state: web::Data<AppState>,
) -> impl Responder {
    let start = Instant::now();
    let three = &state.ctx.three;

    // 1️⃣ 接收请求参数结构体（URL + Body JSON 双重命中，Body 为主）
    let url_req = query.into_inner();
    let mut api_req = if !body.is_empty() {
        match serde_json::from_slice::<ApiGatewayRequest>(&body) {
            // 字段级合并：Body 中有值的字段覆盖 URL，Body 缺省的字段保留 URL
            Ok(body_req) => url_req.merge(body_req),
            Err(_) => url_req,
        }
    } else {
        url_req
    };
    api_req = api_req.build();

    // 2️⃣ 前置身份校验（预留接口）
    let _session = match &api_req.auth {
        Some(auth) if auth.has_token() => {
            // 🚧 后续接入 verify_token
            // auth_service.verify_token(token).await?
            SessionContext::anonymous()
        }
        _ => SessionContext::anonymous(),
    };

    // 3️⃣ 分发
    let service_name = api_req.service.clone().unwrap_or_default();
    match service_name.as_str() {
        "cola_fs" => dispatcher::category::category_dispatch(three, &api_req)
            .await
            .finish(&req, start),
        "sms" => dispatcher::sms::sms_dispatch(three, &api_req)
            .await
            .finish(&req, start),
        "provider" => dispatcher::provider::provider_dispatch(three, &api_req)
            .await
            .finish(&req, start),
        _ => AppData::<()>::err(
            4000,
            format!("[🌐 GATEWAY]: ⚠️ Unknown service: {}", service_name),
            None,
        )
        .finish(&req, start),
    }
}

//////// END
