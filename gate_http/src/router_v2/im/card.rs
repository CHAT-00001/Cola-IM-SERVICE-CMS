// gate_http/src/router_v2/im/card.rs
// 🔌 IM - Card - 用户名片分发器
// 2026/8/8 Created.

////////

use crate::kits::response::IntoApi;
use actix_web::{HttpRequest, Responder, web};
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_auth::info::auth::AuthContext;
use cola_video::api::home::HomeApi;
use app_config::app_state::AppState;
use std::time::Instant;

////////

/// # [DISPATCHER] - 用户名片子路由分发器
/// * `desc`: 处理所有用户名片相关的业务逻辑
pub async fn card_dispatch(
    service: &str,
    req: &HttpRequest,
    query: &super::GatewayQuery,
    body: &web::Bytes,
    state: &web::Data<AppState>,
    auth: AuthContext,
    uid: i64,
    start: Instant,
) -> impl Responder {
    match service {
        // 2001 获取名片
        "card_get" => handle_card_get(req, query, state, auth, uid, start).await,

        // 2002 更新名片
        "card_update" => handle_card_update(req, query, state, auth, uid, start).await,

        // 2003 批量获取名片
        "card_batch" => handle_card_batch(req, query, state, auth, uid, start).await,

        // 2004 搜索名片
        "card_search" => handle_card_search(req, query, state, auth, uid, start).await,

        // 2005 删除名片
        "card_delete" => handle_card_delete(req, query, state, auth, uid, start).await,

        // 2006 名片可见性
        "card_visibility" => handle_card_visibility(req, query, state, auth, uid, start).await,

        _ => AppData::<()>::err(400, format!("Unknown card service: {}", service), None)
            .finish(req, start),
    }
}

////////

/// # 2001 - 获取名片
async fn handle_card_get(
    req: &HttpRequest,
    query: &super::GatewayQuery,
    state: &web::Data<AppState>,
    auth: AuthContext,
    uid: i64,
    start: Instant,
) -> impl Responder {
    let url = ApiGatewayRequest {
        uid: Some(uid),
        page: query.page,
        qty: query.qty,
        ..Default::default()
    }
    .build();

    HomeApi::handler_get_new(auth, url, &state.ctx)
        .await
        .finish(req, start)
}

/// # 2002 - 更新名片
async fn handle_card_update(
    req: &HttpRequest,
    query: &super::GatewayQuery,
    state: &web::Data<AppState>,
    auth: AuthContext,
    uid: i64,
    start: Instant,
) -> impl Responder {
    let url = ApiGatewayRequest {
        uid: Some(uid),
        page: query.page,
        qty: query.qty,
        ..Default::default()
    }
    .build();

    HomeApi::handler_get_new(auth, url, &state.ctx)
        .await
        .finish(req, start)
}

/// # 2003 - 批量获取名片
async fn handle_card_batch(
    req: &HttpRequest,
    query: &super::GatewayQuery,
    state: &web::Data<AppState>,
    auth: AuthContext,
    uid: i64,
    start: Instant,
) -> impl Responder {
    let url = ApiGatewayRequest {
        uid: Some(uid),
        page: query.page,
        qty: query.qty,
        ..Default::default()
    }
    .build();

    HomeApi::handler_get_new(auth, url, &state.ctx)
        .await
        .finish(req, start)
}

/// # 2004 - 搜索名片
async fn handle_card_search(
    req: &HttpRequest,
    query: &super::GatewayQuery,
    state: &web::Data<AppState>,
    auth: AuthContext,
    uid: i64,
    start: Instant,
) -> impl Responder {
    let url = ApiGatewayRequest {
        uid: Some(uid),
        page: query.page,
        qty: query.qty,
        ..Default::default()
    }
    .build();

    HomeApi::handler_get_new(auth, url, &state.ctx)
        .await
        .finish(req, start)
}

/// # 2005 - 删除名片
async fn handle_card_delete(
    req: &HttpRequest,
    query: &super::GatewayQuery,
    state: &web::Data<AppState>,
    auth: AuthContext,
    uid: i64,
    start: Instant,
) -> impl Responder {
    let url = ApiGatewayRequest {
        uid: Some(uid),
        page: query.page,
        qty: query.qty,
        ..Default::default()
    }
    .build();

    HomeApi::handler_get_new(auth, url, &state.ctx)
        .await
        .finish(req, start)
}

/// # 2006 - 名片可见性
async fn handle_card_visibility(
    req: &HttpRequest,
    query: &super::GatewayQuery,
    state: &web::Data<AppState>,
    auth: AuthContext,
    uid: i64,
    start: Instant,
) -> impl Responder {
    let url = ApiGatewayRequest {
        uid: Some(uid),
        page: query.page,
        qty: query.qty,
        ..Default::default()
    }
    .build();

    HomeApi::handler_get_new(auth, url, &state.ctx)
        .await
        .finish(req, start)
}

//////// END
