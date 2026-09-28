// port/src/cola_video/video/check.rs
// ⏩️ 端口 - 可乐视频 -  视频 - 检查
// 2026/8/5 00:00 Created.

////////

////////

/// # [PORT] - 视频评论访问状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoCommentAccess {
    NotFound,
    Forbidden,
    Allowed,
}

/// # [CHECK PORTS] - 管理
/// * `desc`: `视频检查端口`
#[async_trait::async_trait]
pub trait VideoCheckPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 健康
    /// * `desc`: `检查视频健康`
    async fn check_health(
        &self,
        video_id: i64, // 视频 ID
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 2. [PORT] - 状态
    /// * `desc`: `检查视频状态`
    async fn check_state(
        &self,
        video_id: i64, // 视频 ID
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 3. [PORT] - 归属
    /// * `desc`: `▶ 可乐视频 - 检查视频归属`
    async fn is_owner(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频 ID
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 4. [PORT] - 检查视频评论访问状态
    /// * `desc`: `区分视频不存在、禁止评论和允许评论`
    async fn check_comment_access(
        &self,
        video_id: i64, // 视频 ID
    ) -> anyhow::Result<VideoCommentAccess> {
        let _ = video_id;
        Ok(VideoCommentAccess::NotFound)
    }
}

//////// END
