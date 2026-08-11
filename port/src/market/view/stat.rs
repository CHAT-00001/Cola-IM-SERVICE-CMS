// port/src/video/view/state.rs
// ⏩️ 端口 - VIDEO - 浏览 - 统计
// 2026/8/4 22:11 Created.

////////

/// # [STAT PORTS] 浏览 统计
/// * `desc`: `浏览统计端口`
#[async_trait::async_trait]
pub trait GoodsViewStatPort: Send + Sync {
    //

    ////////

    /// # [PORT] - 用户的主动浏览数量
    async fn stat_count_by_user_id(
        &self,
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(u64)>;

    ////////

    /// # [PORT] - 视频的被动浏览数量
    async fn stat_count_by_video_id(
        &self,
        video_id: i64, // 视频 ID
    ) -> anyhow::Result<(u64)>;
}

//////// END
