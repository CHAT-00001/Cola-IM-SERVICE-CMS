// port/src/market/express/stat.rs
// ⏩️ 端口 - MARKET - 快递公司 - 统计
// 2026/8/4 22:11 Created.

////////

/// # [STAT PORTS] - 统计
/// * `desc`: `物流公司统计端口`
#[async_trait::async_trait]
pub trait ExpressStatPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 快递公司总数量
    async fn stat_count(&self) -> anyhow::Result<(u64)>;

    ////////

    /// # 2. [PORT] - 根据状态码获取数量
    async fn stat_count_by_status_code(
        &self,
        status_code: i16, // 状态码
    ) -> anyhow::Result<(u64)>;
}

//////// END
