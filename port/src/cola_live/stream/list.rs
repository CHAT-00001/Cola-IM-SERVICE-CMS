// port/src/cola_live/stream/list.rs
// 端口 - LIVE - 直播场次 - 前台列表
// 2026/8/21 09:32 Created.

////////

use async_trait::async_trait;
use cola_data::cola_live::info::record::LiveRecordInfo;

////////

#[async_trait]
pub trait LiveStreamListPort: Send + Sync + 'static {
    async fn newest(&self, limit: i64, offset: i64) -> anyhow::Result<Vec<LiveRecordInfo>>;
    async fn category(
        &self,
        category_id: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<LiveRecordInfo>>;
    async fn hot(&self, limit: i64, offset: i64) -> anyhow::Result<Vec<LiveRecordInfo>>;
}

//////// END
