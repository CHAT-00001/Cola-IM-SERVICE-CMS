// dislike/add.rs
// 视频 - port - 不喜欢 - 检查
// 2026/8/5 15:59 Created.

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `不喜欢发布服务端口`
#[async_trait::async_trait]
pub trait DislikeAddPort: Send + Sync {
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
