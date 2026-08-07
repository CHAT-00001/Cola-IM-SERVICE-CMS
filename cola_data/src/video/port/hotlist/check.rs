// video/port/hotlist/check.rs
// 视频 - port - 上热门 - 检查
// 2026/8/5 19:25 Created.

////////

/// # [CHECK PORTS] - 检查
/// * `desc`: `视频上热门检查端口`
#[async_trait::async_trait]
pub trait VideoHotlistCheckPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 健康
    /// * `desc`: `检查目标健康`
    async fn health(
        &self,
        uid: i64,        // UID
        collect_id: i64, // 收藏ID
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 状态
    /// * `desc`: `检查目标状态`
    async fn state(
        &self,
        uid: i64,        // UID
        collect_id: i64, // 收藏ID
    ) -> anyhow::Result<()>;
}

//////// END
