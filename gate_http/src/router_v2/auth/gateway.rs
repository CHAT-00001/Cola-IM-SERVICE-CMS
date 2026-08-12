// http/src/v2/auth/gateway.rs  --  HTTP 验证中心 网关
// 2026/6/18 09:26

////////

use crate::kits::response::IntoApi;
use crate::ping::ping;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use app_config::app_state::AppState;
use cola_auth::api::code::AuthCodeApi;
use cola_auth::api::seesion::add::AuthAddApi;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_auth::command::email::EmailLoginCommand;
use cola_data::cola_auth::command::phone::PhoneLoginCommand;
use cola_data::cola_auth::info::auth::AuthContext;
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

/// # [HELPER] - 从 HttpRequest 提取客户端真实 IP
fn extract_client_ip(req: &HttpRequest) -> String {
    // 优先级: X-Forwarded-For > X-Real-IP > peer_addr
    if let Some(ip) = req
        .headers()
        .get("X-Forwarded-For")
        .and_then(|v| v.to_str().ok())
    {
        return ip.to_string();
    }
    if let Some(ip) = req.headers().get("X-Real-IP").and_then(|v| v.to_str().ok()) {
        return ip.to_string();
    }
    if let Some(addr) = req.peer_addr() {
        return addr.ip().to_string();
    }
    "0.0.0.0".to_string()
}

////////

/// # [ROUTER]
pub fn auth_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("", web::get().to(ping))
            .route("/", web::get().to(root))
            .route("/gateway", web::get().to(auth_gateway))
            .route("/gateway", web::post().to(auth_gateway)),
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
        iam_roles: vec![],
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
        //////// 1xxx HOME

        //////// 2xxx SIGN

        // 2001 手机验证码登录（✔ 改为 cmd）
        "add_phone" => {
            let mut cmd: PhoneLoginCommand = extract_cmd(&gateway_req.body).unwrap_or_default();

            // 从 HttpRequest 提取客户端真实 IP 注入 cmd
            cmd.client_ip = extract_client_ip(&req);

            AuthAddApi::handler_sign_in_by_phone(cmd)
                .await
                .finish(&req, start)
        }

        // 2002 邮箱验证码登录（✔ 改为 cmd）
        "add_email" => {
            let cmd: EmailLoginCommand = extract_cmd(&gateway_req.body).unwrap_or_default();

            AuthAddApi::handler_sign_in_by_email(cmd)
                .await
                .finish(&req, start)
        }

        // 2003 账密验证码登录（✔ 改为 cmd）
        "add_pwd" => {
            let cmd: EmailLoginCommand = extract_cmd(&gateway_req.body).unwrap_or_default();

            AuthAddApi::handler_sign_in_by_email(cmd)
                .await
                .finish(&req, start)
        }

        // 2004 谷歌登录（✔ 改为 cmd）
        "add_google" => {
            let cmd: EmailLoginCommand = extract_cmd(&gateway_req.body).unwrap_or_default();

            AuthAddApi::handler_sign_in_by_email(cmd)
                .await
                .finish(&req, start)
        }

        // 2005 苹果登录（✔ 改为 cmd）
        "add_apple" => {
            let cmd: EmailLoginCommand = extract_cmd(&gateway_req.body).unwrap_or_default();

            AuthAddApi::handler_sign_in_by_email(cmd)
                .await
                .finish(&req, start)
        }

        // 2006 微信登录（✔ 改为 cmd）
        "add_wechat" => {
            let cmd: EmailLoginCommand = extract_cmd(&gateway_req.body).unwrap_or_default();

            AuthAddApi::handler_sign_in_by_email(cmd)
                .await
                .finish(&req, start)
        }

        // 2400 退出登录
        "add_out" => {
            let cmd: PhoneLoginCommand = extract_cmd(&gateway_req.body).unwrap_or_default();

            AuthAddApi::handler_sign_out(cmd).await.finish(&req, start)
        }

        //////// 3xxx CODE

        // 3001 获取短信验证码
        "code_phone" => {
            let phone = url.by.to_string();

            AuthCodeApi::handler_get_sms_code(&phone)
                .await
                .finish(&req, start)
        }

        // 3002 获取邮箱验证码
        "code_email" => {
            let email = url.by.to_string();

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
            2004,
            format!("[🌐 GATEWAY]: ⚠️ Unknown the [🆔 AUTH] service: {}", gateway_req.service),
            None,
        )
        .finish(&req, start),
    }
}

//////// END
