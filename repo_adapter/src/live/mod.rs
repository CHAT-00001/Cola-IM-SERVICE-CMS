// repo_adapter/src/live/mod.rs -- 🔌 适配器 - 可乐LIVE - mod
// 2026/8/10 20:00 Updated.

////////

use crate::live::category::LiveCategoryAdapter;
use crate::live::stream::LiveStreamAdapter;
use async_trait::async_trait;
use port::cola_live::ColaLivePort;
use port::cola_live::user::LiveUserPort;
use repository::cola_live::pg::user_repo::LiveUserRepo;
use std::sync::Arc;

// 注：直播 Port 复用 GIS 的 Adapter
use crate::gis;

////////

/// # [BUILD] - 构建 LIVE Port
/// * `desc`: 构建直播 Port 聚合体，复用 GIS 相关功能
pub fn build_live_port() -> ColaLivePort {
    ColaLivePort {
        stream: port::cola_live::stream::LiveStreamPort {
            add: Arc::new(LiveStreamAdapter),
            check: Arc::new(LiveStreamAdapter),
            get: Arc::new(LiveStreamAdapter),
            list: Arc::new(LiveStreamAdapter),
            manage: Arc::new(LiveStreamAdapter),
        },
        category: Arc::new(LiveCategoryAdapter),
        add: Arc::new(gis::add::AddPortAdapter),
        like: Arc::new(gis::like::LikePortAdapter),
        view: Arc::new(gis::view::ViewPortAdapter),
        user: Arc::new(LiveUserInitAdapter),
    }
}

pub mod category;
pub mod stream;

////////

/// # [ADAPTER] - 初始化直播域用户适配器
#[derive(Debug, Default, Clone)]
pub struct LiveUserInitAdapter;

#[async_trait]
impl LiveUserPort for LiveUserInitAdapter {
    /// # 1. [ADAPTER] - 初始化直播域用户
    /// * `desc`: `转发到 LIVE 用户仓储`
    async fn init_live_user(&self, user_id: i64) -> anyhow::Result<()> {
        LiveUserRepo::init_live_user(user_id).await
    }
}

//////// END
