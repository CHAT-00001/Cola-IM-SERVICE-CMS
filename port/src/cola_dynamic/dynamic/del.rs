// port/src/cola_dynamic/dynamic/del.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 动态 - 软删除
// 2026/8/5 00:00 Created.

////////


////////

/// # [DELETE PORTS] - 删除
/// * `desc`: `动态删除端口`
#[async_trait::async_trait]
pub trait DelPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个软删除
    /// * `desc`: `用户批量删除视频`
    async fn single_soft_del(
        &self,
        dynamic_id: i64, // 动态 ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 2. [PORT] - 批量软删除
    /// * `desc`: `用户批量删除视频`
    async fn batch_soft_del(
        &self,
        dynamic_ids: Vec<i64>, // 动态 IDs
    ) -> anyhow::Result<(u16)>;
}

//////// END
