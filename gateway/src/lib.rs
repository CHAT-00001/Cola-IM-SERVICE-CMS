// gateway/src/lib.rs -- 网关库 - lib
// 2025-12-10 14:20

////////

use tracing::info;
use app_config::app_state::AppState;
use app_config::config::Gateway; // 从 app crate 引入配置类型

pub mod auth;
mod music;

////////


/// 启动 Gateway 服务
pub async fn start_gateway(config: &Gateway,app_state: AppState) {
    info!("Gateway service listening on {}:{}", config.host, config.port);

    // TODO: 这里写实际的监听逻辑，例如 TCP 服务器
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
