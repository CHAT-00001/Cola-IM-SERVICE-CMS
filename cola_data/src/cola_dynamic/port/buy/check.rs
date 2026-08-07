// buy/check.rs
// 视频 - port - 购买 - 检查
// 2026/8/5 00:06 Created.

////////

/// # [CHECK PORTS] - 检查
/// * `desc`: `检查购买状态端口`
#[async_trait::async_trait]
pub trait BuyCheckPort: Send + Sync {
    ////////

    /// # [PORT] - 保存
    async fn save_buy_record(
        &self,
        uid: i64,
        video_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 删除
    async fn del_buy_record(
        &self,
        uid: i64,
        video_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 获取用户收藏的IDs
    async fn get_buy_ids_by_user_id(
        &self,
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END