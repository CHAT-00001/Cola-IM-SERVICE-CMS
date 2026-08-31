// gate_http/src/router_v2/cola_three/dispatcher/category.rs  -- 分类/类型 分发器
// 2026/7/27 11:40

//////

use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_three::api::server_type::ServerTypeApi;
use port::cola_three::ColaThreePort;
use serde_json::Value;
//////

/// # [ACTION] - 动作码
pub mod action {
    pub const HOME_LIST: i16 = 1001;
    pub const HOME_DETAIL: i16 = 1002;
    pub const CREATE: i16 = 5001;
    pub const EDIT: i16 = 5002;
    pub const REORDER: i16 = 5003;
}

//////

/// # [DISPATCH] - 转发器
pub async fn category_dispatch(three: &ColaThreePort, req: &ApiGatewayRequest) -> AppData<Value> {
    let action = req.action.unwrap_or(1000);

    // 🚧 动作转发
    match action {
        // 1000-1999 前台 home, 无需登录
        1001 => {
            let inner = ServerTypeApi::list(three.r#type.as_ref()).await;
            AppData {
                code: inner.code,
                message: inner.message,
                error: inner.error,
                duration: inner.duration,
                request_id: inner.request_id,
                at: inner.at,
                log_id: inner.log_id,
                data: inner
                    .data
                    .map(|d| serde_json::to_value(d).unwrap_or_default()),
            }
        }
        1002 => home_detail(req).await,

        // 2000-2999 登录后 feed
        2001 => feed_list(req).await,

        // 4000-4999 状态检查
        4001 => check_status(req).await,

        // 5000-5999 新增/编辑/排序
        5001 => create(req).await,
        5002 => edit(req).await,
        5003 => reorder(req).await,

        // 6000-6999 运营管理
        6001 => admin_list(req).await,
        6002 => toggle_status(req).await,

        // 7000-7999 审计
        7001 => audit_log(req).await,
        7002 => delete(req).await,

        _ => AppData::err(
            400,
            format!("[🚧 DISPATCH]: Unknown dispatch action: {}", action),
            None,
        ),
    }
}

//////// END

async fn home_list(_req: &ApiGatewayRequest) -> AppData<Value> {
    AppData::ok(serde_json::json!({"service":"cola_fs","action":"home_list"}))
}
async fn home_detail(_req: &ApiGatewayRequest) -> AppData<Value> {
    AppData::ok(serde_json::json!({"service":"cola_fs","action":"home_detail"}))
}
async fn feed_list(_req: &ApiGatewayRequest) -> AppData<Value> {
    AppData::ok(serde_json::json!({"service":"cola_fs","action":"feed_list"}))
}
async fn check_status(_req: &ApiGatewayRequest) -> AppData<Value> {
    AppData::ok(serde_json::json!({"service":"cola_fs","action":"check_status"}))
}
async fn create(_req: &ApiGatewayRequest) -> AppData<Value> {
    AppData::ok(serde_json::json!({"service":"cola_fs","action":"create"}))
}
async fn edit(_req: &ApiGatewayRequest) -> AppData<Value> {
    AppData::ok(serde_json::json!({"service":"cola_fs","action":"edit"}))
}
async fn reorder(_req: &ApiGatewayRequest) -> AppData<Value> {
    AppData::ok(serde_json::json!({"service":"cola_fs","action":"reorder"}))
}
async fn admin_list(_req: &ApiGatewayRequest) -> AppData<Value> {
    AppData::ok(serde_json::json!({"service":"cola_fs","action":"admin_list"}))
}
async fn toggle_status(_req: &ApiGatewayRequest) -> AppData<Value> {
    AppData::ok(serde_json::json!({"service":"cola_fs","action":"toggle_status"}))
}
async fn audit_log(_req: &ApiGatewayRequest) -> AppData<Value> {
    AppData::ok(serde_json::json!({"service":"cola_fs","action":"audit_log"}))
}
async fn delete(_req: &ApiGatewayRequest) -> AppData<Value> {
    AppData::ok(serde_json::json!({"service":"cola_fs","action":"delete"}))
}
