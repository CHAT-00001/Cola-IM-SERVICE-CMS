// port/src/cola_video/video/stat.rs
// ⏩️ 端口 - 可乐视频 -  视频 - 统计
// 2026/8/5 00:00 Created.

////////

////////

/// # [STAT PORTS] - 统计
/// * `desc`: `视频统计端口`
#[async_trait::async_trait]
pub trait CateStatPort: Send + Sync {
    //

    ////////

    ////////

    /// # [PORT] - 用户的
    /// * `desc`: `根据用户ID` - `统计购买数量`
    async fn stat_count_by_user_id(
        &self,
        uid: i64,
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(u64)>;

    ////////

    /// # [PORT] - 视频的
    /// * `desc`: `统计所有视频数量`
    async fn stat_count(&self) -> anyhow::Result<(u64)>;
}

//////// END
