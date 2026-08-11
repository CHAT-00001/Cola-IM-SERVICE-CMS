// repo_adapter/src/market/shop/del.rs
// 🔌 插头 - MARKET - 商品 - 删除服务
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::shop::del::ShopDeletePort;

////////

/// # [DELETE ADAPTER] - 删除
/// * `desc`: `🔌 视频删除服务`
pub struct ShopDelAdapter;

#[async_trait]
impl ShopDeletePort for ShopDelAdapter {
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
