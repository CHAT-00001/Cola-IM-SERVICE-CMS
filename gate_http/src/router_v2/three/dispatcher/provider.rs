// gate_http/src/router_v2/cola_three/dispatcher/provider.rs  -- 厂商 分发器
// 2026/7/27 12:04

//////

use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use port::cola_three::ColaThreePort;
use serde_json::Value;
//////

/// # [DISPATCH] - 厂商转发器
pub async fn provider_dispatch(_three: &ColaThreePort, req: &ApiGatewayRequest) -> AppData<Value> {
    let action = req.action.unwrap_or(0);
    AppData::ok(serde_json::json!({"service":"provider","action":action}))
}
