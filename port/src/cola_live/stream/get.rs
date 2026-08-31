// port/src/cola_live/stream/get.rs
// 端口 - LIVE - 直播场次 - 获取
// 2026/8/21 09:32 Created.

////////

use async_trait::async_trait;
use cola_data::cola_live::info::record::LiveRecordInfo;

////////

#[async_trait]
pub trait LiveStreamGetPort: Send + Sync + 'static {
    async fn current(&self, uid: i64, room_id: i64) -> anyhow::Result<Option<LiveRecordInfo>>;
}

//////// END
