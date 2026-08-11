// port/src/video/buy/check.rs
// ⏩️ 端口 - 视频 - 购买 - 检查
// 2026/8/5 00:06 Created.

////////

/// # [CHECK PORTS] - 检查
/// * `desc`: `检查购买状态端口`
#[async_trait::async_trait]
pub trait GoodsBuyCheckPort: Send + Sync {
    //

    ////////

    /// # [PORT] - 是否购买
    /// * `desc`: `检查是否已经购买`
    async fn is_bought(
        &self,
        user_id: i64,  // 用户 ID
        video_id: i64, // 视频 ID
    ) -> anyhow::Result<(bool)>;
}

//////// END
