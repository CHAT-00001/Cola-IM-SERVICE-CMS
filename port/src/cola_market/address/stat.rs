// port/src/market/address/state.rs
// ⏩️ 端口 - MARKET - 地址簿 - 统计
// 2026/8/4 22:11 Created.

////////

/// # [STAT PORTS] - 统计
/// * `desc`: `地址簿统计端口`
#[async_trait::async_trait]
pub trait AddressStatPort: Send + Sync {
    //

    ////////

    /// # [PORT] - 用户的主动浏览数量
    async fn stat_count_by_user_id(
        &self,
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(u64)>;

    ////////

    /// # [PORT] - 视频的被动浏览数量
    async fn stat_count_by_video_id(
        &self,
        video_id: i64, // 视频 ID
    ) -> anyhow::Result<(u64)>;
}

//////// END
