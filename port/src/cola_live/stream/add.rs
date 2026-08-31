// port/src/cola_live/stream/add.rs
// 端口 - LIVE - 直播场次 - 发布
// 2026/8/21 09:32 Created.

////////

use async_trait::async_trait;
use cola_data::cola_live::command::stream::record::LiveRecordCommand;
use cola_data::cola_live::info::record::LiveRecordInfo;

////////

#[async_trait]
pub trait LiveStreamAddPort: Send + Sync + 'static {
    async fn start(&self, uid: i64, command: LiveRecordCommand) -> anyhow::Result<LiveRecordInfo>;
}

//////// END
