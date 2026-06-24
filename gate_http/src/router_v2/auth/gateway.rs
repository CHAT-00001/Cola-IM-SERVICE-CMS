// http/src/v2/auth/gateway.rs  --  HTTP 验证中心 网关
// 2026/6/18 09:26

////////

use crate::kits::response::IntoApi;
use crate::ping::ping;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use app_config::app_state::AppState;
use cola_auth::api::add::AuthAddApi;
use cola_auth::api::code::AuthCodeApi;
use cola_auth::api::session::SessionApi;
use cola_auth::case::add::AuthAddCase;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::command::email::EmailLoginCommand;
use cola_data::auth::command::phone::PhoneLoginCommand;
use cola_data::auth::info::auth::AuthContext;
use cola_user::api::add::AddApi;
use cola_user::api::home::HomeApi;
use serde::Deserialize;
use std::time::Instant;

////////

/// # 网关请求体
struct GatewayRequest {
    auth: AuthContext,
    action: i16,
    service: String,
    query: Option<String>,
    body: web::Bytes,
    path: String,
}

/// # 统一的 Query 提取结构体
#[derive(Deserialize)]
pub struct GatewayQuery {
    pub service: String,
    pub action: Option<i16>,
    pub video_id: Option<i64>,
    pub page: Option<i64>,
    pub qty: Option<i64>,
}

////////
/// # [HELPER] - 从 body 中提取 cmd（新增，最小侵入）
fn extract_cmd<T>(body: &web::Bytes) -> Option<T>
where
    T: serde::de::DeserializeOwned,
{
    let v: serde_json::Value = serde_json::from_slice(body).ok()?;
    v.get("cmd")
        .cloned()
        .and_then(|cmd| serde_json::from_value(cmd).ok())
}

////////

/// # [ROUTER]
pub fn auth_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("", web::get().to(ping))
            .route("/", web::get().to(root))
            .route("/gateway", web::get().to(auth_gateway)),
    );
}

async fn root() -> HttpResponse {
    HttpResponse::Ok().json(vec!["Cole", "VIDEO", "ROUTER"])
}

////////

/// # [GATEWAY]
async fn auth_gateway(
    req: HttpRequest,
    url: web::Query<ApiGatewayRequest>,
    query: web::Query<GatewayQuery>,
    body: web::Bytes,
    state: web::Data<AppState>,
) -> impl Responder {
    let start = Instant::now();

    let uid = match req.extensions().get::<i64>().copied() {
        Some(id) => id,
        None => 1,
    };

    let auth = AuthContext {
        uid,
        access_token: String::new(),
        refresh_token: String::new(),
        device_id: String::new(),
        roles: vec![],
        is_anonymous: false,
    };

    let gateway_req = GatewayRequest {
        auth,
        action: query.action.unwrap_or(0),
        service: query.service.clone(),
        query: Some(req.query_string().to_string()),
        body,
        path: req.path().to_string(),
    };

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

        // 2001 手机验证码登录（✔ 改为 cmd）
        "add.phone" => {
            let cmd: PhoneLoginCommand = extract_cmd(&gateway_req.body).unwrap_or_default();

            AuthAddApi::handler_sign_in_by_phone(cmd)
                .await
                .finish(&req, start)
        }

        // 2002 邮箱验证码登录（✔ 改为 cmd）
        "add.email" => {
            let cmd: EmailLoginCommand = extract_cmd(&gateway_req.body).unwrap_or_default();

            AuthAddApi::handler_sign_in_by_email(cmd)
                .await
                .finish(&req, start)
        }

        // 3001 获取短信验证码
        "code.phone" => {
            let phone = url.params.get("phone").cloned().unwrap_or_default();

            AuthCodeApi::handler_get_sms_code(&phone)
                .await
                .finish(&req, start)
        }

        // 3002 获取邮箱验证码
        "code.email" => {
            let email = url.params.get("email").cloned().unwrap_or_default();

            AuthCodeApi::handler_get_email_code(&email)
                .await
                .finish(&req, start)
        }

        // // 4001 session
        // "session.view" => {
        //     let query = ApiGatewayRequest {
        //         uid: Some(uid),
        //         page: query.page,
        //         qty: query.qty,
        //         ..Default::default()
        //     }.build();
        //
        //     AuthCodeApi::handler_get_email_code(&(query.email))
        //         .await
        //         .finish(&req, start)
        // }
        "sign_in.test" => {
            let data = serde_json::json!({
                "session_id": 123456,
                "user_id": uid,
                "access_token": "这是access token",
                "refresh_token": "这是refresh_token",
                "status": "published"
            });

            AppData::ok(data).finish(&req, start)
        }

        "publish_comment" => {
            let data = serde_json::json!({
                "comment_id": 67890,
                "user_id": uid,
                "video_id": query.video_id.unwrap_or(0),
                "content": "示例评论内容"
            });

            AppData::ok(data).finish(&req, start)
        }

        _ => AppData::<()>::err(
            400,
            format!("Unknown PhalApi service: {}", gateway_req.service),
            None,
        )
        .finish(&req, start),
    }
}
