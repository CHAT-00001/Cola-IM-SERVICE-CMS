// repo_adapter/src/video/port/like/add.rs
// 🔌 适配器 - ▶ 视频 - 点赞 - 发布
// 2026/8/5 00:02 Created.

////////

////////

/// # [ADD PORTS] - 点赞
/// * `desc`: `视频点赞发布端口`
#[async_trait::async_trait]
pub trait VideoLikeAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 发布点赞
    async fn send_like(
        &self,
        uid: i64,       // 操作者ID
        video_id: i64,  // 视频 ID
        is_liked: bool, // 是否点赞
    ) -> anyhow::Result<()>;
}

//////// END
