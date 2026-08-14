// cola_video/port/comment/del.rs
// ⏩️ 端口 - VIDEO - 评论 - 删除
// 2026/8/5 00:03 Created.

////////

////////

/// # [DEL PORTS] - 删除
/// `desc`: `VIDEO - 评论删除端口`
#[async_trait::async_trait]
pub trait VideoCommentDelPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个删除
    async fn single_delete(
        &self,
        id: i64, // 目标 ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 2. [PORT] - 批量删除
    async fn batch_delete(
        &self,
        ids: Vec<i64>, // 目标 IDs
    ) -> anyhow::Result<(u16)>;
}

//////// END
