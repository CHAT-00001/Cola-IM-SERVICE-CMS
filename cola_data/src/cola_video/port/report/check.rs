// cola_video/port/comment/check.rs
// 评论 - port - 评论 - 检查
// 2026/8/5 00:00 Created.

////////

////////

/// # [CHECK PORTS] - 检查
/// * `desc`: `举报检查服务端口`
#[async_trait::async_trait]
pub trait VideoReportCheckPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 健康
    /// * `desc`: `检查目标健康`
    async fn health(
        &self,
        uid: i64,        // UID
        comment_id: i64, // 评论ID
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 状态
    /// * `desc`: `检查目标状态`
    async fn state(
        &self,
        uid: i64,        // UID
        comment_id: i64, // 弹幕ID
    ) -> anyhow::Result<()>;
}

//////// END
