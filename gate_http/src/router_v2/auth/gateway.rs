// gate_http/src/v2/auth/gateway.rs  --  HTTP网关 -  验证中心 - 业务网关
// 2026/6/18 09:26

////////

use crate::kits::response::IntoApi;
use crate::ping::ping;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use app_config::app_state::AppState;
use cola_auth::api::code::AuthCodeApi;
use cola_auth::api::seesion::add::SessionAddApi;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::command::email::EmailLoginCommand;
use cola_data::auth::command::phone::PhoneLoginCommand;
use std::time::Instant;

////////

/// # [HELPER] - 合并 URL 参数与 Body 命令
/// * `desc`: Body 的 cmd 字段优先，URL 的 by 仅作为命令字段缺省值
fn extract_cmd_with_by<T>(request: &ApiGatewayRequest, field: &str) -> Option<T>
where
    T: serde::de::DeserializeOwned,
{
    let mut cmd = request
        .body
        .as_ref()
        .and_then(|body| body.get("cmd"))
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    if let Some(cmd_object) = cmd.as_object_mut() {
        if !cmd_object.contains_key(field) {
            cmd_object.insert(field.to_string(), serde_json::json!(request.by));
        }
    }
    serde_json::from_value(cmd).ok()
}

////////

/// # [HELPER] - 从 Body cmd 或通用请求字段提取字符串参数
/// * `desc`: Body 的 `cmd.field` 优先，缺省时回退到 URL/body 顶层的 `by`
fn extract_cmd_string(request: &ApiGatewayRequest, field: &str) -> String {
    request
        .body
        .as_ref()
        .and_then(|body| body.get("cmd"))
        .and_then(|cmd| cmd.get(field))
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| request.by.clone())
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
    body: web::Bytes,
    state: web::Data<AppState>,
) -> impl Responder {
    let start = Instant::now();

    let uid = match req.extensions().get::<i64>().copied() {
        Some(id) => id,
        None => 1,
    };

    let url_request = ApiGatewayRequest {
        uid: Some(uid),
        ..url.into_inner()
    };
    let mut body_request = serde_json::from_slice::<ApiGatewayRequest>(&body).unwrap_or_default();
    body_request.body = serde_json::from_slice(&body).ok();
    let request = url_request.merge(body_request).build();
    let service = request.service.as_deref().unwrap_or_default();

    match service {
        //////// 1xxx HOME

        //////// 2xxx SIGN

        // 2001 手机验证码登录（✔ 改为 cmd）
        "add_phone" => {
            let mut cmd: PhoneLoginCommand =
                extract_cmd_with_by(&request, "phone_no").unwrap_or_default();

            // 从 HttpRequest 提取客户端真实 IP 注入 cmd
            cmd.client_ip = extract_client_ip(&req);

            SessionAddApi::handler_sign_in_by_phone(cmd, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 2002 邮箱验证码登录（✔ 改为 cmd）
        "add_email" => {
            let cmd: EmailLoginCommand = extract_cmd_with_by(&request, "email").unwrap_or_default();

            SessionAddApi::handler_sign_in_by_email(cmd)
                .await
                .finish(&req, start)
        }

        // 2003 账密验证码登录（✔ 改为 cmd）
        "add_pwd" => {
            let cmd: EmailLoginCommand = extract_cmd_with_by(&request, "email").unwrap_or_default();

            SessionAddApi::handler_sign_in_by_email(cmd)
                .await
                .finish(&req, start)
        }

        // 2004 谷歌登录（✔ 改为 cmd）
        "add_google" => {
            let cmd: EmailLoginCommand = extract_cmd_with_by(&request, "email").unwrap_or_default();

            SessionAddApi::handler_sign_in_by_email(cmd)
                .await
                .finish(&req, start)
        }

        // 2005 苹果登录（✔ 改为 cmd）
        "add_apple" => {
            let cmd: EmailLoginCommand = extract_cmd_with_by(&request, "email").unwrap_or_default();

            SessionAddApi::handler_sign_in_by_email(cmd)
                .await
                .finish(&req, start)
        }

        // 2006 微信登录（✔ 改为 cmd）
        "add_wechat" => {
            let cmd: EmailLoginCommand = extract_cmd_with_by(&request, "email").unwrap_or_default();

            SessionAddApi::handler_sign_in_by_email(cmd)
                .await
                .finish(&req, start)
        }

        // 2400 退出登录
        "add_out" => {
            let cmd: PhoneLoginCommand =
                extract_cmd_with_by(&request, "phone_no").unwrap_or_default();

            SessionAddApi::handler_sign_out(cmd)
                .await
                .finish(&req, start)
        }

        //////// 3xxx CODE

        // 3001 获取短信验证码
        "code_phone" => {
            let phone = extract_cmd_string(&request, "phone_no");

            AuthCodeApi::handler_get_sms_code(&phone)
                .await
                .finish(&req, start)
        }

        // 3002 获取邮箱验证码
        "code_email" => {
            let email = extract_cmd_string(&request, "email");

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
                "user_id": request.uid.unwrap_or(uid),
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
                "video_id": request.video_id,
                "content": "示例评论内容"
            });

            AppData::ok(data).finish(&req, start)
        }

        _ => AppData::<()>::err(
            2004,
            format!(
                "[🌐 GATEWAY]: ⚠️ Unknown the [🆔 AUTH] service: {}",
                service
            ),
            None,
        )
        .finish(&req, start),
    }
}

//////// END
