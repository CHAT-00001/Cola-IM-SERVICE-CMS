// port/src/cola_market/goods/stat.rs
// ⏩️ 端口 - MARKET -  商品 - 统计
// 2026/8/5 00:00 Created.

////////

////////

/// # [STAT PORTS] - 统计
/// * `desc`: `商品统计端口`
#[async_trait::async_trait]
pub trait GoodsStatPort: Send + Sync {
    //

    ////////

    /// # [PORT] - 用户的
    /// * `desc`: `根据用户ID` - `统计购买数量`
    async fn stat_count_by_user_id(
        &self,
        uid: i64,
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(u64)>;

    ////////

    /// # [PORT] - 商品的
    /// * `desc`: `统计所有商品数量`
    async fn stat_count(&self) -> anyhow::Result<(u64)>;
}

//////// END
