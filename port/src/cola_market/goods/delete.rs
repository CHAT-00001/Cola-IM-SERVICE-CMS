// port/src/cola_market/goods/del.rs
// ⏩️ 端口 - MARKET -  商品 - 删除
// 2026/8/5 00:00 Created.

////////

/// # [DELETE PORTS] - 软删除
/// * `desc`: `MARKET - 商品记录删除端口`
#[async_trait::async_trait]
pub trait GoodsDeletePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个删除
    async fn single_delete(
        &self,
        goods_id: i64, // 商品 ID
    ) -> anyhow::Result<(u64)>;

    ////////

    /// # 2. [PORT] - 批量删除
    async fn batch_delete(
        &self,
        goods_ids: Vec<i64>, // 商品 IDs
    ) -> anyhow::Result<(u64)>;

    ////////

    /// # 3. [PORT] - 用户删除时
    async fn delete_videos_by_user_id(
        &self,
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(u64)>;
}

//////// END
