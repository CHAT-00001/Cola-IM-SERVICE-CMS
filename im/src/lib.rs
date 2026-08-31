use app_config::app_state::AppState;
use app_config::config::Ws;
use redis::Client as RedisClient;
use tracing::info;

pub async fn start_ws(ws_config: &Ws, app_state: AppState) {
    // 从 AppState 获取 Redis client
    let redis_conn = &app_state.db.redis_conn;

    info!(
        "WS service listening on {}:{}",
        ws_config.host, ws_config.port
    );

    // TODO: 这里启动 WebSocket server 并使用 redis_client
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
