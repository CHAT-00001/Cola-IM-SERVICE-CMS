// cola_video/port/like.rs
// 视频 - port - 点赞 模块
// 2026/6/10 06:15

////////

pub mod add;
pub mod del;
pub mod get;
mod check;

/// # [PORT] - 点赞
#[async_trait::async_trait]
pub trait LikeRepo: Send + Sync {
    ////////

    /// # [PORT] - 点赞
    async fn like_video(&self, uid: i64, video_id: i64, is_liked: bool) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 不喜欢
    async fn unlike_video(&self, uid: i64, video_id: i64, is_unliked: bool) -> anyhow::Result<()>;
}
