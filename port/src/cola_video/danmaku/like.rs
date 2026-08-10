// port/src/cola_video/danmaku/like.rs
// ⏩️ 端口 - ▶ 可乐视频 - 弹幕 - 列表
// 2026/8/9 03:28 Created.

////////

////////

/// # [LIKE PORTS] - 点赞
/// * `desc`: `▶ 可乐视频 - 弹幕点赞端口`
#[async_trait::async_trait]
pub trait VideoDanmakuLikePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 更新/插入
    /// * `desc`: `用户更新/插入点赞记录`
    async fn upsert_like(
        &self,
        uid: i64,        // UID
        comment_id: i64, // 评论 ID
        is_liked: bool,  // 状态
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 2. [PORT] - 检查是否点赞
    /// * `desc`: `用户更新/插入点赞记录`
    async fn check_state(
        &self,
        uid: i64,        // UID
        comment_id: i64, // 评论 ID
    ) -> anyhow::Result<()>;
}

//////// END
