// gate_http/src/router_v2/gateway_dispatcher.rs
// 🌐 命令式网关分发器 - 通用框架
// 2026/8/12 20:00 Created.

////////

use actix_web::{HttpRequest, HttpResponse};
use app_config::app_state::AppState;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_auth::request::session::SessionContext;
use std::time::Instant;

////////

/// # [GATEWAY DISPATCHER TRAIT] - 命令式网关分发器
/// * `desc`: 定义网关分发器的统一接口
#[async_trait::async_trait]
pub trait GatewayDispatcher: Send + Sync {
    /// 执行分发逻辑
    async fn dispatch(
        ctx: &AppState,
        session: &SessionContext,
        api_req: &ApiGatewayRequest,
    ) -> AppData<serde_json::Value>;
}

////////

/// # [COMMAND HANDLER] - 命令处理器
/// * `desc`: 根据 name 参数路由到具体的 API 处理函数
pub struct CommandHandler;

impl CommandHandler {
    /// 通用分发入口
    pub async fn dispatch_by_name(
        ctx: &AppState,
        session: &SessionContext,
        api_req: &ApiGatewayRequest,
        handlers: &[(&str, Box<dyn Fn() -> std::pin::Pin<std::boxed::Box<dyn std::future::Future<Output = AppData<serde_json::Value>> + Send>> + Send + Sync>)],
    ) -> AppData<serde_json::Value> {
        let action_name = api_req.action.clone().unwrap_or_default();

        for (name, handler) in handlers {
            if name == &action_name.as_str() {
                return handler().await;
            }
        }

        AppData::<serde_json::Value>::err(
            4001,
            format!("[🚧 DISPATCH]: Unknown action: {}", action_name),
            None,
        )
    }
}

////////

/// # [RESPONSE WRAPPER] - 响应包装器
/// * `desc`: 为 AppData 添加协议信息和时间统计
pub trait ResponseWrapper {
    fn finish(self, req: &HttpRequest, start: Instant) -> HttpResponse;
}

impl<T: serde::ser::Serialize> ResponseWrapper for AppData<T> {
    fn finish(self, req: &HttpRequest, start: Instant) -> HttpResponse {
        let elapsed = start.elapsed();
        let mut resp = HttpResponse::Ok().json(&self);

        // 添加协议头
        resp.headers_mut().insert(
            "X-Cost-Ms",
            format!("{}", elapsed.as_millis())
                .parse()
                .unwrap_or_else(|_| "0".parse().unwrap()),
        );

        resp
    }
}

//////// END
