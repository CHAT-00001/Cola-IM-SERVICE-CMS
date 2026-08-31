// port/src/video/port/share/check.rs
// ⏩️ 端口 - ▶ 视频 - 视频 - 检查
// 2026/8/5 00:00 Created.

////////

////////

/// # [CHECK PORTS] - 检查
/// * `desc`: `收藏检查端口`
#[async_trait::async_trait]
pub trait VideoShareCheckPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 健康
    /// * `desc`: `检查视频状态`
    async fn health(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频ID
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 状态
    /// * `desc`: `检查目标状态`
    async fn state(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频ID
    ) -> anyhow::Result<()>;
}

//////// END
