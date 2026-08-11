// port/src/market/cate/check.rs
// ⏩️ 端口 - MARKET - CATE - 检查
// 2026/8/5 00:00 Created.

////////

////////

/// # [CHECK PORTS] - 管理
/// * `desc`: `商品分类检查端口`
#[async_trait::async_trait]
pub trait CateCheckPort: Send + Sync {
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
}

//////// END
