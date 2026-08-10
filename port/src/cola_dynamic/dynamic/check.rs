// port/src/cola_dynamic/dynamic/check.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 动态 - 检查
// 2026/8/5 00:00 Created.

////////

////////

/// # [CHECK PORTS] - 检查
/// * `desc`: `⏹ 可乐动态 - 动态检查端口`
#[async_trait::async_trait]
pub trait CheckPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 健康
    /// * `desc`: `检查视频健康`
    async fn check_health(
        &self,
        dynamic_id: i64, // 动态 ID
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 2. [PORT] - 状态
    /// * `desc`: `检查视频状态`
    async fn check_state(
        &self,
        dynamic_id: i64, // 动态 ID
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 3. [PORT] - 是我的
    /// * `desc`: `⏹ 可乐动态 - 检查视频归属`
    async fn is_owner(
        &self,
        uid: i64,        // 操作者 ID
        dynamic_id: i64, // 动态 ID
    ) -> anyhow::Result<(bool)>;
}

//////// END
