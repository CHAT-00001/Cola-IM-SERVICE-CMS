// port/src/market/order/add.rs
// 端口 - MARKET - 购买 - 订单发布
// 2026/8/11 05:17 Created.

////////

////////

/// # [PORT] - 商品购买
#[async_trait::async_trait]
pub trait GoodsBuyAddPort: Send + Sync {
    ////////

    /// # [PORT] - 添加订单
    async fn add_order(&self, uid: i64, goods_id: i64) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 删除订单
    async fn del_order(&self, uid: i64, goods_id: i64) -> anyhow::Result<()>;

    ////////

}
