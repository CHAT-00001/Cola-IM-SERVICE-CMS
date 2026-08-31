// buy/get.rs
// 视频 - port - 购买 - 获取
// 2026/8/5 00:06 Created.

////////

/// # [GET PORTS] - 获取
/// `desc`: `视频购买获取端口`
#[async_trait::async_trait]
pub trait GoodsBuyGetPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 我的
    /// * `desc`: `用户批量获取购买的视频IDs`
    async fn get_my_bought_ids(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 2. [PORT] - TA
    /// * `desc`: `用户批量获取购买的视频IDs`
    async fn get_he_bought_ids(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END
