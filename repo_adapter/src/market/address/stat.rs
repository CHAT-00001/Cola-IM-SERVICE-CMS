// repo_adapter/src/market/address/stat.rs
// 🔌 适配器 - MARKET - ADDRESS - 统计服务
// 2026/8/6 19:20 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::address::stat::AddressStatPort;

////////

/// # [STAT ADAPTER] - 统计
/// * `desc`: `MARKET - 地址统计适配器`
pub struct AddressStatAdapter;

// 构造实现
#[async_trait]
impl AddressStatPort for AddressStatAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 统计用户的视频数量
    async fn stat_count_by_user_id(&self, user_id: i64) -> Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 统计所有视频数量
    async fn stat_count_by_video_id(&self, video_id: i64) -> Result<(u64)> {
        todo!()
    }
}

//////// END
