// port/src/cola_video/comment/like.rs
// ⏩️ 端口 - VIDEO - 评论 - 点赞
// 2026/8/9 03:28 Created.

////////

////////

/// # [LIKE PORTS] - 点赞
/// * `desc`: `VIDEO - 评论点赞端口`
#[async_trait::async_trait]
pub trait VideoCommentLikePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 更新/插入
    /// * `desc`: `用户更新/插入点赞记录`
    async fn upsert_like(
        &self,
        uid: i64,        // UID
        comment_id: i64, // 评论 ID
        state: bool,     // 状态
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 2. [PORT] - 检查是否点赞
    /// * `desc`: `用户更新/插入点赞记录`
    async fn check_state(
        &self,
        uid: i64,        // UID
        comment_id: i64, // 评论 ID
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 3. [PORT] - 查询客户端操作幂等结果
    /// * `desc`: `根据 action_id UUID v4 查询当前用户的评论点赞操作是否已经执行`
    async fn get_operation_result(
        &self,
        uid: i64,        // UID
        action_id: &str, // 业务动作 UUID v4
    ) -> anyhow::Result<Option<bool>> {
        let _ = (uid, action_id);
        Ok(None)
    }

    ////////

    /// # 4. [PORT] - 写入客户端操作幂等结果
    /// * `desc`: `保存 action_id 操作结果, 防止客户端重试重复执行`
    async fn set_operation_result(
        &self,
        uid: i64,        // UID
        action_id: &str, // 业务动作 UUID v4
        state: bool,     // 点赞状态
    ) -> anyhow::Result<()> {
        let _ = (uid, action_id, state);
        Ok(())
    }
}

//////// END
