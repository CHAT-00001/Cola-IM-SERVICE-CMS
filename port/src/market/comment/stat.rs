// port/src/cola_video/comment/stat.rs
// ⏩️ 端口 - ▶ 可乐视频 - 评论 - 统计
// 2026/8/5 00:03 Created.

////////

////////

/// # [SERVICE PORT] - 统计
#[async_trait::async_trait]
pub trait VideoCommentStatPort: Send + Sync {
    //

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
    /// * `desc`: `根据视频ID` - `统计购买数量`
    async fn stat_count_by_video_id(
        &self,
        uid: i64,
        video_id: i64, // 视频 ID
    ) -> anyhow::Result<(u64)>;
}

//////// END
