// cola_data/src/video/port/view.rs  -- 浏览服务
// 2026/6/10 07:13

////////

/// # [PORT] - 开始浏览
#[async_trait::async_trait]
pub trait ViewPort: Send + Sync {

    ////////

    /// # [PORT] - 点赞
    async fn view_video(
        &self,
        uid: i64,
        video_id: i64,
        is_liked: bool,
    ) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 浏览完成
    async fn view_done(
        &self,
        uid: i64,
        video_id: i64,
        is_unliked: bool,
    ) -> anyhow::Result<()>;
}