// app/src/app_state.rs -- 应用配置 - 应用状态
// 2026-04-16 07:18

////////

use crate::db_service::DbService;
use cola_data::app::ctx::AppContext;
use std::sync::Arc;

////////

/// AppState 用来在各服务间共享数据库连接和 Repository 实例
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DbService>, // 数据库连接池
    pub ctx: AppContext,    // 应用上下文
}

impl AppState {
    /// 🌟 完善 new 方法：接收具体的 Repo 实现并进行注入
    pub fn new(db: DbService, ctx: AppContext) -> Self {
        Self {
            db: Arc::new(db),
            ctx,
        }
    }
}

//////// END
