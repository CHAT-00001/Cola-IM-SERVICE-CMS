// gate_http/src/router_v2/cola_three/dispatcher/sms.rs  -- 短信 分发器
// 2026/7/27

//////

use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use serde_json::Value;
use port::cola_three::ColaThreePort;

//////

/// # [DISPATCH] - 转发器
pub async fn sms_dispatch(
    _three: &ColaThreePort,
    req: &ApiGatewayRequest,
) -> AppData<Value> {
    let action = req.action.unwrap_or(0);
    AppData::ok(serde_json::json!({"service":"sms","action":action}))
}