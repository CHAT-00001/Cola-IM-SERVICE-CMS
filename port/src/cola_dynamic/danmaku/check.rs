// port/src/cola_video/danmaku/check.rs
// ⏩️ 端口 - ▶ 可乐动态 - 弹幕 - 检查
// 2026/8/9 04:04 Created.

////////

////////

/// # [CHECK PORTS] - 检查
/// * `desc`: `▶ 可乐动态 - 弹幕检查端口`
#[async_trait::async_trait]
pub trait DynamicDanmakuCheckPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 健康
    /// * `desc`: `检查目标健康`
    async fn check_health(
        &self,
        uid: i64,        // UID
        danmaku_id: i64, // 弹幕 ID
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 2. [PORT] - 状态
    /// * `desc`: `检查目标状态`
    async fn check_state(
        &self,
        uid: i64,        // UID
        danmaku_id: i64, // 弹幕 ID
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 3. [PORT] - 是否所有者
    /// * `desc`: `检查目标状态`
    async fn is_owner(
        &self,
        uid: i64,        // UID
        danmaku_id: i64, // 弹幕 ID
    ) -> anyhow::Result<(bool)>;

}

//////// END
