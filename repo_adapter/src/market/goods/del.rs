// repo_adapter/src/cola_market/goods/del.rs
// 🔌 适配器 - MARKET - 商品 - 逻辑删除
// 2026/8/11 08:32 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_market::goods::delete::GoodsDeletePort;
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
    async fn single_delete(&self, goods_id: i64) -> Result<(u64)> {
        GoodsDeleteRepo::delete_by_id(goods_id).await?;
        Ok((u64))
    }

    ////////

    /// # [ADAPTER] - 批量删除
    async fn batch_delete(&self, goods_ids: Vec<i64>) -> Result<(u64)> {
        GoodsDeleteRepo::delete_by_ids(goods_id).await?;
    }

    /////////

    /// # [ADAPTER] - 删除用户的商品
    async fn delete_videos_by_user_id(&self, user_id: i64) -> Result<(u64)> {
        GoodsDeleteRepo::delete_by_user_id(goods_id).await?;
        todo!()
    }
}


//////// END