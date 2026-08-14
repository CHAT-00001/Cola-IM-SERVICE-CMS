// port/src/cola_video/comment/dislike.rs
// ⏩️ 端口 - VIDEO - 评论 - 列表
// 2026/8/9 03:30 Created.

////////

////////

/// # [LIKE PORTS] - 不喜欢
/// * `desc`: `VIDEO - 评论不喜欢端口`
#[async_trait::async_trait]
pub trait VideoCommentDisikePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 更新/插入
    /// * `desc`: `用户更新/插入不喜欢记录`
    async fn upsert_dislike(
        &self,
        uid: i64,        // UID
        comment_id: i64, // 评论 ID
        is_disliked: bool,  // 状态
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 2. [PORT] - 检查是否不喜欢
    /// * `desc`: `用户更新/插入不喜欢记录`
    async fn check_state(
        &self,
        uid: i64,        // UID
        comment_id: i64, // 评论 ID
    ) -> anyhow::Result<(bool)>;
}

//////// END
