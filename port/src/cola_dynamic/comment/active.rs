// port/src/cola_dynamic/file/active.rs
// ⏩️ 端口 - 可乐动态 - 评论 - 活跃
// 2026/8/5 00:03 Created.

////////

////////

/// # [SERVICE PORT] - 评论
#[async_trait::async_trait]
pub trait CommentRepo: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 保活
    async fn active_item(&self, uid: i64, video_id: i64, is_liked: bool) -> anyhow::Result<()>;

    ////////
}
