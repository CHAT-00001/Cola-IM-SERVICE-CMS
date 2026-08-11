// port/src/market/express/delete.rs
// ⏩️ 端口 - MARKET - 快递公司 - 删除
// 2026/8/4 22:10 Created.

////////

/// # [DELETE PORTS]
/// * `desc`: `MARKET - 管理员删除快递公司记录`
#[async_trait::async_trait]
pub trait ExpressDeletePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个删除
    async fn single_delete(
        &self,
        express_id: i64, // 快递 ID
    ) -> anyhow::Result<(u64)>;

    ////////

    /// # 2. [PORT] - 批量删除
    async fn batch_delete(
        &self,

        express_ids: Vec<i64>, // 快递 IDs
    ) -> anyhow::Result<(u64)>;
}

//////// END
