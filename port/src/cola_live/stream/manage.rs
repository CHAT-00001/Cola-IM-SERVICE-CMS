// port/src/cola_live/stream/manage.rs
// 端口 - LIVE - 直播场次 - 管理
// 2026/8/21 09:32 Created.

////////

use async_trait::async_trait;

////////

#[async_trait]
pub trait LiveStreamManagePort: Send + Sync + 'static {
    async fn stop(&self, uid: i64, record_id: i64) -> anyhow::Result<()>;
}

//////// END
