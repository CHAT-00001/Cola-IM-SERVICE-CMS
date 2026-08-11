// port/src/market/express/get.rs
// ⏩️ 端口 - MARKET - 快递公司 -  获取
// 2026/8/4 22:10 Created.

////////

////////

/// # [GET  PORTS]
/// * `desc`: `MARKET - 快递公司获取端口`
#[async_trait::async_trait]
pub trait ExpressGetPort: Send + Sync {
    //

    ////////

    /// # [PORT] - 获取用户ID获取快递IDs
    async fn get_express_ids_by_user_id(
        &self,
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END
