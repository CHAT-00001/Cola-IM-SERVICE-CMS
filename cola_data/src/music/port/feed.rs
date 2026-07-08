// cola_data/src/music/port/feed.rs  -- 数据中心 - MUSIC - port - feed流
// 2026/7/7 13:18

////////

use crate::music::info::music::MusicInfo;

////////

/// # [SERVICE PORT] - Feed 服务
#[async_trait::async_trait]
pub trait FeedPort: Send + Sync {
    ////////

    /// # 1. [PORT] - 关注
    async fn feed_follow(&self, uid: i64, limit: i64, offset: i64) -> anyhow::Result<(Vec<MusicInfo>)>;

    ////////

    /// # 2. [PORT] - 朋友
    async fn feed_friend(&self, uid: i64, limit: i64, offset: i64) -> anyhow::Result<(Vec<MusicInfo>)>;

    ////////

    /// # 3. [PORT] - 推荐
    async fn feed_recommend(&self, uid: i64, limit: i64, offset: i64) -> anyhow::Result<(Vec<MusicInfo>)>;

    ////////

    /// # 4. [PORT] - 看过的
    async fn feed_visited(&self, uid: i64, limit: i64, offset: i64) -> anyhow::Result<(Vec<MusicInfo>)>;

    ////////

    /// # 5. [PORT] - 点赞过的
    async fn feed_liked(&self, uid: i64, limit: i64, offset: i64) -> anyhow::Result<(Vec<MusicInfo>)>;

    ////////

    /// # 6. [PORT] - 收藏过的
    async fn feed_collected(&self, uid: i64, limit: i64, offset: i64) -> anyhow::Result<(Vec<MusicInfo>)>;
}

//////// END
