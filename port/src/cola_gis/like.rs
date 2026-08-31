// /port/follow  --
// 2026/7/7

//////

/// # [PORT] - 点赞
#[async_trait::async_trait]
pub trait LikeRepo: Send + Sync {
    ////////

    /// # [PORT] - 点赞
    async fn like_poi(&self, uid: i64, poi_id: i64, is_liked: bool) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 不喜欢
    async fn unlike_poi(&self, uid: i64, poi_id: i64, is_unliked: bool) -> anyhow::Result<()>;
}
