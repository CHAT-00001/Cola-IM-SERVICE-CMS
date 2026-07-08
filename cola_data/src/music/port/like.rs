// cola_data/src/music/port/like.rs  -- 数据中心 - MUSIC - port - 点赞
// 2026/6/10 06:15

////////

/// # [SERVICE PORT] - 音乐 点赞 服务端口
#[async_trait::async_trait]
pub trait LikeRepo: Send + Sync {
    ////////

    /// # 1. [PORT] - 点赞
    async fn add_like(&self, uid: i64, music_id: i64, status: i16) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 不喜欢
    async fn add_unlike(&self, uid: i64, music_id: i64, status: i16) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - 获取点赞的IDs
    async fn get_like_ids(&self, uid: i64, limit: i64, offset: i64) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 4. [PORT] - 获取不喜欢的IDs
    async fn get_unlike_ids(&self, uid: i64, limit: i64, offset: i64) -> anyhow::Result<(Vec<i64>)>;
}
