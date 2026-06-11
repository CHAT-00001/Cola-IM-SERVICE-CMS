// api/src/app_state.rs
// 2026-04-16 07:18

use crate::db_service::DbService;
use std::sync::Arc;

/// AppState 用来在各服务间共享数据库连接和 Repository 实例
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DbService>,
}

impl AppState {
    /// 🌟 完善 new 方法：接收具体的 Repo 实现并进行注入
    pub fn new(
        db: DbService,

    ) -> Self {
        Self {
            db: Arc::new(db),
        }
    }
}