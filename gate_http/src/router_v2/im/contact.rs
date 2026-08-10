// gate_http/src/router_v2/im/contact.rs
// 🔌 IM - Contact - 联系人管理分发器
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

/// # [DISPATCHER] - 联系人子路由分发器
/// * `desc`: 处理所有联系人相关的业务逻辑
pub async fn contact_dispatch(
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
        // 1001 添加联系人
        "contact_add" => handle_contact_add(req, query, state, auth, uid, start).await,

        // 1002 联系人同步
        "contact_sync" => handle_contact_sync(req, query, state, auth, uid, start).await,

        // 1003 删除联系人
        "contact_del" => handle_contact_del(req, query, state, auth, uid, start).await,

        // 1004 联系人列表
        "contact_list" => handle_contact_list(req, query, state, auth, uid, start).await,

        // 1005 联系人搜索
        "contact_search" => handle_contact_search(req, query, state, auth, uid, start).await,

        // 1006 联系人详情
        "contact_detail" => handle_contact_detail(req, query, state, auth, uid, start).await,

        // 1007 修改联系人
        "contact_update" => handle_contact_update(req, query, state, auth, uid, start).await,

        // 1008 获取黑名单
        "contact_black_list" => handle_contact_black_list(req, query, state, auth, uid, start).await,

        _ => AppData::<()>::err(400, format!("Unknown contact service: {}", service), None)
            .finish(req, start),
    }
}

////////

/// # 1001 - 添加联系人
async fn handle_contact_add(
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

/// # 1002 - 联系人同步
async fn handle_contact_sync(
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

/// # 1003 - 删除联系人
async fn handle_contact_del(
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

/// # 1004 - 联系人列表
async fn handle_contact_list(
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

/// # 1005 - 联系人搜索
async fn handle_contact_search(
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

/// # 1006 - 联系人详情
async fn handle_contact_detail(
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

/// # 1007 - 修改联系人
async fn handle_contact_update(
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

/// # 1008 - 获取黑名单
async fn handle_contact_black_list(
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
