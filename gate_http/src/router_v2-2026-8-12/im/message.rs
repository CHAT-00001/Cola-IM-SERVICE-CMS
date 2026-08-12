// gate_http/src/router_v2/im/message.rs
// 🔌 IM - Message - 消息管理分发器
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

/// # [DISPATCHER] - 消息管理子路由分发器
pub async fn message_dispatch(
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
        "message_send" => handle_message_send(req, query, state, auth, uid, start).await,
        "message_list" => handle_message_list(req, query, state, auth, uid, start).await,
        "message_mark_read" => handle_message_mark_read(req, query, state, auth, uid, start).await,
        "message_delete" => handle_message_delete(req, query, state, auth, uid, start).await,
        "message_recall" => handle_message_recall(req, query, state, auth, uid, start).await,
        "message_edit" => handle_message_edit(req, query, state, auth, uid, start).await,
        "message_search" => handle_message_search(req, query, state, auth, uid, start).await,
        "message_forward" => handle_message_forward(req, query, state, auth, uid, start).await,
        "message_unread_count" => handle_message_unread_count(req, query, state, auth, uid, start).await,
        _ => AppData::<()>::err(400, format!("Unknown message service: {}", service), None)
            .finish(req, start),
    }
}

////////

async fn handle_message_send(
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

async fn handle_message_list(
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

async fn handle_message_mark_read(
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

async fn handle_message_delete(
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

async fn handle_message_recall(
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

async fn handle_message_edit(
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

async fn handle_message_search(
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

async fn handle_message_forward(
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

async fn handle_message_unread_count(
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
