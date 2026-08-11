// repo_adapter/src/market/shop/stat.rs
// 🔌 适配器 - MARKET - SHOP - 统计
// 2026/8/6 19:20 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::shop::stat::ShopStatPort;

////////

/// # [STAT SERVICE] - 统计
/// * `desc`: `MARKET - 商店统计适配器`
pub struct ShopStatAdapter;

// 构造实现
#[async_trait]
impl ShopStatPort for ShopStatAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 统计用户的视频数量
    async fn stat_count_by_user_id(&self, uid: i64, user_id: i64) -> Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 统计所有视频数量
    async fn stat_count(&self) -> Result<(u64)> {
        todo!()
    }
}

//////// END
