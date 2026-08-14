// /dislike.rs
// 
// 2026/8/9 03:28 Created.

////////


// port/src/cola_dynamic/identity/like.rs
// ⏩️ 端口 - 可乐动态 - 评论 - 列表
// 2026/8/5 00:03 Created.

////////



////////

/// # [LIKE PORTS] - 点赞
/// * `desc`: `可乐动态 - 评论点赞服务端口`
#[async_trait::async_trait]
pub trait VideoDanmakuDislikePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 更新/插入
    /// * `desc`: `用户更新/插入点赞记录`
    async fn upsert_like_record(
        &self,
        uid: i64,        // UID
        comment_id: i64, // 评论 ID
        is_liked: bool,  // 状态
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 检查是否点赞
    /// * `desc`: `用户更新/插入点赞记录`
    async fn check_like_state(
        &self,
        uid: i64,        // UID
        comment_id: i64, // 评论 ID
    ) -> anyhow::Result<()>;
}

//////// END
