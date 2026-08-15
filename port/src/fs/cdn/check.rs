// port/src/fs/cdn/check.rs
// ⏩️ 端口 - FS - CDN - 检查
// 2026/8/5 00:00 Created.

////////

////////

/// # [CHECK PORTS] - 检查
/// * `desc`: `▶ 可乐视频 - 评论检查端口`
#[async_trait::async_trait]
pub trait CdnCheckPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 健康
    /// * `desc`: `检查目标健康`
    async fn check_health(
        &self,
        uid: i64,        // UID
        comment_id: i64, // 评论 ID
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 2. [PORT] - 状态
    /// * `desc`: `检查目标状态`
    async fn check_state(
        &self,
        uid: i64,        // UID
        comment_id: i64, // 评论 ID
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 3. [PORT] - 是否所有者
    /// * `desc`: `根据用户ID + 评论ID` - `检查是否属于作者`
    async fn is_owner(
        &self,
        uid: i64,        // UID
        user_id: i64,    // 用户 ID
        comment_id: i64, // 评论 ID
    ) -> anyhow::Result<(bool)>;
}

//////// END
