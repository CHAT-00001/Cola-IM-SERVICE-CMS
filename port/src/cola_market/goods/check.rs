// port/src/cola_market/goods/check.rs
// ⏩️ 端口 - MARKET - 商品 - 检查
// 2026/8/5 00:00 Created.

////////

////////

/// # [CHECK PORTS] - 检查
/// * `desc`: `商品状态检查端口`
#[async_trait::async_trait]
pub trait GoodsCheckPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 健康
    /// * `desc`: `检查商品健康`
    async fn check_health(
        &self,
        goods_id: i64, // 商品 ID
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 2. [PORT] - 状态
    /// * `desc`: `检查商品状态`
    async fn check_state(
        &self,
        goods_id: i64, // 商品 ID
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 3. [PORT] - 归属
    /// * `desc`: `▶ 可乐商品 - 检查商品归属`
    async fn is_owner(
        &self,
        uid: i64,      // UID
        goods_id: i64, // 商品 ID
    ) -> anyhow::Result<(bool)>;
}

//////// END
