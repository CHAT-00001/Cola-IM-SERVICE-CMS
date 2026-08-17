// src/api.rs
// 2025-11-45 09:40

////////

use app_config::{db_service::DbService, app_state::AppState};
use im::start_ws;
use health::start_health;
use tracing::{info, error};
use app_config::config::config_loader::load_config;
use repo_adapter::build_app_context;
use gate_grpc::start_gateway;
use gate_http::start_api;

////////

/// # [RUN] - 运行
pub async fn run() {

    // 加载配置
    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to load config: {:?}", e);
            return;
        }
    };

    // 初始化数据库
    let db_service = match DbService::new(&config).await {
        Some(db) => db,
        None => {
            error!("Failed to initialize database service");
            return;
        }
    };

    // ⚠️ 2026-03-27 新增 / 2026-05-26 修正
    // 统一调用 lib.rs 暴露出来的初始化标准函数，避免直接对 OnceLock 进行硬编码操作
    app_config::init_global_db(db_service.clone());
    info!("✅ Global database pointer initialized via init_global_db.");

    // 构建 AppContext（注入所有 port trait 的 Adapter 实现）
    let ctx = build_app_context();
    info!("✅ AppContext built with all repository adapters injected.");

    // 实例化 AppState，传入 db_service 和 ctx
    let app_state = AppState::new(db_service, ctx);
    // ----------------------------

    // 启动 API
    let api_handle = {
        let api_config = config.api.clone();
        let app_state = app_state.clone();
        tokio::spawn(async move {
            start_api(&api_config, app_state).await;
        })
    };

    // 启动 WS
    let ws_handle = {
        let ws_config = config.ws.clone();
        let app_state = app_state.clone();
        tokio::spawn(async move {
            start_ws(&ws_config, app_state).await;
        })
    };

    // 启动 Health
    let health_handle = {
        let health_config = config.health.clone();
        let app_state = app_state.clone();
        tokio::spawn(async move {
            start_health(&health_config, app_state).await;
        })
    };

    // 启动 Gateway
    let gateway_handle = {
        let gateway_config = config.gateway.clone();
        let app_state = app_state.clone();
        tokio::spawn(async move {
            start_gateway(&gateway_config, app_state).await;
        })
    };

    info!("😊 😊 😊 😊 ✅️ ✅️ ✅️ ✅️ All services started, waiting for them to run...");
    info!("████████████░░░░░░░░  60% 🟡");
    info!("████████████████████  100% 🟡");

    // 💡 改进方案：使用 tokio::select! 代替 tokio::join!
    // 只要有任何一个底层微服务 handle 崩溃或意外退出，立刻触发 error 日志并结束 run() 退出程序，防止僵尸服务
    tokio::select! {
        res = api_handle => if let Err(e) = res { error!("API service panicked: {:?}", e); },
        res = ws_handle => if let Err(e) = res { error!("WS service panicked: {:?}", e); },
        res = health_handle => if let Err(e) = res { error!("Health service panicked: {:?}", e); },
        res = gateway_handle => if let Err(e) = res { error!("Gateway service panicked: {:?}", e); },
    }

    error!("❌ One of the cola_video services exited unexpectedly. Shutting down application...");
}

//////// END