// src/main.rs  -- 主程序入口
// 2025-12-10 14:23


use tracing_subscriber::{fmt, EnvFilter};
mod app;

#[tokio::main]
async fn main() {
    // 初始化全局日志
    let filter = EnvFilter::new("info,short-live = debug");
    tracing_subscriber::fmt()
        .with_env_filter(filter)

        .init();
    tracing::info!("Hello, world! This should be printed.");

    // 启动应用
    app::run().await;
}
