// gate_http/src/router_v2/cola_video/gateway.rs
// HTTP网关 - v2 - VIDEO - 业务网关
// 2026/5/25 06:49
// 2026/8/2 重构

////////

use crate::kits::response::IntoApi;
use crate::ping::ping;
use crate::router_v2::video::dispatcher;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use app_config::app_state::AppState;
use cola_auth::api::seesion::state::SessionStateApi;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_auth::request::session::SessionContext;
use std::time::Instant;

////////

pub fn video_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cola_video")
            .route("/", web::get().to(root))
            .route("", web::get().to(ping))
            .route("/gateway", web::get().to(video_gateway))
            .route("/gateway", web::post().to(video_gateway)),
    );
}

pub async fn root() -> HttpResponse {
    HttpResponse::Ok().json(vec!["Cole", "VIDEO", "ROUTER"])
}

////////

/// # [VIDEO GATEWAY] - 短视频网关
async fn video_gateway(
    req: HttpRequest,
    query: web::Query<ApiGatewayRequest>,
    body: web::Bytes,
    state: web::Data<AppState>,
) -> impl Responder {
    let start = Instant::now();
    let ctx = &state.ctx;

    // 1️⃣ URL + Body JSON 双重命中，Body 为主（对齐 cola_three 网关模式）
    let url_req = query.into_inner();
    let mut api_req = if !body.is_empty() {
        match serde_json::from_slice::<ApiGatewayRequest>(&body) {
            Ok(body_req) => url_req.merge(body_req),
            Err(_) => url_req,
        }
    } else {
        url_req
    };
    api_req = api_req.build();

    // 2️⃣ 把完整原始 Body JSON 注入 api_req.body，供 dispatcher 读取 body.cmd
    api_req.body = serde_json::from_slice(&body).ok();

    // 3️⃣ 前置身份校验：body.cola_auth.access_token → 数据库验证
    let session = match &api_req.auth {
        Some(auth) if auth.has_token() => {
            let result = SessionStateApi::verify_session(auth, &ctx.auth).await;
            if result.code == 0 {
                result.data.unwrap_or_else(|| {
                    tracing::warn!("🌐 [VIDEO GATEWAY]: ⚠️ verify_session 无数据，游客");
                    SessionContext::anonymous()
                })
            } else {
                tracing::warn!(
                    "🌐 [VIDEO GATEWAY]: ⚠️ verify_session 失败: {}",
                    result.message
                );
                SessionContext::anonymous()
            }
        }
        _ => SessionContext::anonymous(),
    };

    tracing::info!(
        "🌐 🌐 🌐 🌐  [VIDEO GATEWAY]: service={:?} action={:?} uid={} is_anonymous={} body.cmd={:?}",
        api_req.service,
        api_req.action,
        session.uid,
        session.is_anonymous,
        api_req.body.as_ref().and_then(|b| b.get("cmd")),
    );
    tracing::info!("🆔: api_req.cola_auth = {:?}", api_req.auth);

    // 4️⃣ 分发
    let service_name = api_req.service.clone().unwrap_or_default();
    match service_name.as_str() {

        // 1. 发布
        "add" => dispatcher::add::add_dispatch(ctx, &session, &api_req)
            .await
            .finish(&req, start),

        // 2. 首页
        "home" => dispatcher::home::home_dispatch(ctx, &session, &api_req)
            .await
            .finish(&req, start),

        // 3. 推荐
        "feed" => dispatcher::feed::feed_dispatch(ctx, &session, &api_req)
            .await
            .finish(&req, start),

        // 4. 评论
        "comment" => dispatcher::comment::comment_dispatch(ctx, &session, &api_req)
            .await
            .finish(&req, start),

        // 5. 弹幕
        "danmaku" => dispatcher::danmaku::danmaku_dispatch(ctx, &session, &api_req)
            .await
            .finish(&req, start),

        // x. 错误阻断
        _ => AppData::<()>::err(
            4000,
            format!("[🌐 GATEWAY]: ⚠️ Unknown service: {}", service_name),
            None,
        )
        .finish(&req, start),
    }
}

//////// END