// /add  -- 购买 服务端口
// 2026/7/7

//////

/// # [PORT] - 购买
#[async_trait::async_trait]
pub trait BuyRepo: Send + Sync {
    ////////

    /// # [PORT] - 保存
    async fn save_buy_record(&self, uid: i64, poi_id: i64) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 删除
    async fn del_buy_record(&self, uid: i64, poi_id: i64) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 获取用户购买的IDs
    async fn get_buy_ids_by_user_id(
        &self,
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<i64>>;
}
