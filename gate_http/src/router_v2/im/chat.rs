// gate_http/src/router_v2/im/chat.rs
// 🔌 IM - Chat - 聊天管理分发器
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

/// # [DISPATCHER] - 聊天管理子路由分发器
pub async fn chat_dispatch(
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
        "chat_add" => handle_chat_add(req, query, state, auth, uid, start).await,
        "chat_del" => handle_chat_del(req, query, state, auth, uid, start).await,
        "chat_list" => handle_chat_list(req, query, state, auth, uid, start).await,
        "chat_manage" => handle_chat_manage(req, query, state, auth, uid, start).await,
        "chat_setting" => handle_chat_setting(req, query, state, auth, uid, start).await,
        "chat_pin" => handle_chat_pin(req, query, state, auth, uid, start).await,
        "chat_sync" => handle_chat_sync(req, query, state, auth, uid, start).await,
        _ => AppData::<()>::err(400, format!("Unknown chat service: {}", service), None)
            .finish(req, start),
    }
}

////////

async fn handle_chat_add(
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

async fn handle_chat_del(
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

async fn handle_chat_list(
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

async fn handle_chat_manage(
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

async fn handle_chat_setting(
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

async fn handle_chat_pin(
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

async fn handle_chat_sync(
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
