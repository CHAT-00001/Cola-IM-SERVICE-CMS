// like/check.rs
// 视频 - port - 点赞 - 检查
// 2026/8/5 15:06 Created.

////////

/// # [CHECK SERVICE] - 检查
/// * `desc`: `点赞检查服务端口`
#[async_trait::async_trait]
pub trait LikeCheckPort: Send + Sync {
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
