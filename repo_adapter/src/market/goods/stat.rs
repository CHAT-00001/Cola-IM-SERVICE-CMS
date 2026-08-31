// repo_adapter/src/market/goods/stat.rs
// 🔌 适配器 - MARKET - GOODS - STAT
// 2026/8/11 07:18 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::goods::stat::GoodsStatPort;

////////

/// # [STAT ADAPTER] - 统计
/// `desc`: `MARKET - 商品统计适配器`
pub struct GoodsStatAdapter;

#[async_trait]
impl GoodsStatPort for GoodsStatAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 用户有多少商品
    async fn stat_count_by_user_id(&self, uid: i64, user_id: i64) -> Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 商品总数
    async fn stat_count(&self) -> Result<(u64)> {
        todo!()
    }
}

//////// END
