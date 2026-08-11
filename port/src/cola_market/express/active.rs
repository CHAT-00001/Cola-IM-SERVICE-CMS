// port/src/market/express/active.rs
// ⏩️ 端口  - MARKET - 快递公司 - 活跃
// 2026/8/4 22:11 Created.

////////

/// # [ACTIVE PORTS] - 激活
/// * `desc`: `MARKET - 快递激活端口`
#[async_trait::async_trait]
pub trait ExpressActivePort: Send + Sync {
    //

    ////////

    /// # [PORT] - 设置
    /// * `desc`: `设置默认快递`
    async fn set_default_express(
        &self,
        uid: i64,        // 操作员 ID
        express_id: i64, // 快递 ID
    ) -> anyhow::Result<()>;
}

//////// END
