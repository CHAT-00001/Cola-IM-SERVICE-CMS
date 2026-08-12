// repo_adapter/src/market/goods/del.rs
// 🔌 适配器 - MARKET - 商品 - 逻辑删除
// 2026/8/11 08:32 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::goods::delete::GoodsDeletePort;
use repository::cola_market::pg::goods::delete::GoodsDeleteRepo;

////////

/// # [DELETE ADAPTER] - 逻辑删除
/// `desc`: `MARKET - 商品列表逻辑删除`
pub struct GoodsDeleteAdapter;

#[async_trait]
impl GoodsDeletePort for GoodsDeleteAdapter {
    //

    ////////
    /// # [ADAPTER] - 单个删除
    async fn single_delete(
        &self,
        goods_id: i64 // 商品 ID
    ) -> anyhow::Result<(u64)> {

        // 1. Call REPOSITORY ..
        Ok(GoodsDeleteRepo::delete_by_id(goods_id).await?)
    }

    ////////

    /// # [ADAPTER] - 批量删除
    async fn batch_delete(
        &self,
        goods_ids: Vec<i64>, // 商品 IDs
    ) -> anyhow::Result<(u64)> {
        Ok(GoodsDeleteRepo::delete_by_ids(&goods_ids).await?)
    }

    /////////

    /// # [ADAPTER] - 删除用户的商品
    async fn delete_videos_by_user_id(&self, user_id: i64) -> anyhow::Result<(u64)> {
        Ok(GoodsDeleteRepo::delete_by_user_id(user_id).await?)
    }
}

//////// END
