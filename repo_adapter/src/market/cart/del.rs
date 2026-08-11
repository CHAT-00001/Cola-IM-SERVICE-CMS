// repo_adapter/src/market/cart/del.rs
// 🔌 插头 - MARKET - CART - 删除
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::cart::del::CartDeletePort;

////////

/// # [DELETE ADAPTER] - 删除
/// * `desc`: `购物车删除适配器`
pub struct CartDelAdapter;

#[async_trait]
impl CartDeletePort for CartDelAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 单个删除
    async fn single_delete(&self, video_id: i64) -> Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 批量删除
    async fn batch_delete(&self, video_ids: Vec<i64>) -> Result<(u64)> {
        todo!()
    }

    ////////

    /// # 3. [ADAPTER] - 用户删除时
    async fn delete_videos_by_user_id(&self, user_id: i64) -> Result<(u64)> {
        todo!()
    }
}

//////// END
