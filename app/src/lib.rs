// app/src/lib.rs -- 应用配置 - lib
// 2025-12-10 04:50

////////

use std::sync::{Arc, OnceLock};
pub use crate::db_service::DbService;

////////

pub mod db;
pub mod app_state;
pub mod config;
pub mod db_service;
mod kits;

////////

// 定义全局静态变量
pub static GLOBAL_DB: OnceLock<Arc<DbService>> = OnceLock::new();
/// 初始化全局数据库句柄（在 main.rs 启动时调用一次）
pub fn init_global_db(service: DbService) {
    GLOBAL_DB.set(Arc::new(service)).ok();
}

//////// END
