// port/src/cola_live/stream/check.rs
// 端口 - LIVE - 直播场次 - 检查
// 2026/8/21 09:32 Created.

////////

use async_trait::async_trait;

////////

#[async_trait]
pub trait LiveStreamCheckPort: Send + Sync + 'static {
    async fn can_start(&self, uid: i64, room_id: i64) -> anyhow::Result<()>;
}

//////// END
