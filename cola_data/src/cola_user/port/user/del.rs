// cola_user/port/cola_user/del.rs
// 用户 - port - 用户 - 删除
// 2026/8/5 22:02 Created.

////////

////////

/// # [DEL PORTS] - 删除
/// * `desc`: `用户删除端口`
#[async_trait::async_trait]
pub trait UserDelPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 单个软删除
    async fn single_soft_del(
        &self,
        uid: i64, // 操作者ID
        id: i64,  // 目标ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 2. [PORT] - 批量软删除
    async fn batch_soft_del(
        &self,
        uid: i64,      // 操作者ID
        ids: Vec<i64>, // 目标IDs
    ) -> anyhow::Result<(u16)>;
}

//////// END