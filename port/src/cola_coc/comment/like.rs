// port/src/cola_video/comment/like.rs
// ⏩️ 端口 - VIDEO - 评论 - 点赞
// 2026/8/9 03:28 Created.

////////

////////

/// # [LIKE PORTS] - 点赞
/// * `desc`: `VIDEO - 评论点赞端口`
#[async_trait::async_trait]
pub trait VideoCommentLikePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 更新/插入
    /// * `desc`: `用户更新/插入点赞记录`
    async fn upsert_like(
        &self,
        uid: i64,        // UID
        comment_id: i64, // 评论 ID
        state: bool,     // 状态
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 2. [PORT] - 检查是否点赞
    /// * `desc`: `用户更新/插入点赞记录`
    async fn check_state(
        &self,
        uid: i64,        // UID
        comment_id: i64, // 评论 ID
    ) -> anyhow::Result<(bool)>;
}

//////// END
