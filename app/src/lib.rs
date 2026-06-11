// api/src/lib.rs

use std::sync::{Arc, OnceLock};
pub use crate::db_service::DbService;

pub mod db;
pub mod app_state;
pub mod config_loader;
pub mod config;
pub mod db_service;

pub mod pg;
pub mod redis;

pub mod mongodb;
mod kits;
mod ctx;

// 定义全局静态变量
pub static GLOBAL_DB: OnceLock<Arc<DbService>> = OnceLock::new();
/// 初始化全局数据库句柄（在 main.rs 启动时调用一次）
pub fn init_global_db(service: DbService) {
    GLOBAL_DB.set(Arc::new(service)).ok();
}
