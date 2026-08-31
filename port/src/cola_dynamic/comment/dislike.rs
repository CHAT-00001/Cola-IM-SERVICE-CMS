// port/src/cola_dynamic/danmaku/dislike.rs
// ⏩️ 端口 - 可乐动态 - 评论 - 不喜欢
// 2026/8/9 04:26 Created.

////////

////////

/// # [DISLIKE PORTS] - 不喜欢
/// * `desc`: `▶ 可乐动态` - 评论不喜欢端口
#[async_trait::async_trait]
pub trait DynamicCommentDisikePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 更新/插入
    /// * `desc`: `用户更新/插入不喜欢记录`
    async fn upsert_dislike(
        &self,
        uid: i64,          // UID
        comment_id: i64,   // 评论 ID
        is_disliked: bool, // 状态
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
