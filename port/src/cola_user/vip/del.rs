// port/src/user/vip/del.rs
// 用户 - port - 贵宾 - 删除
// 2026/8/6 00:39 Created.

////////

use async_trait::async_trait;

////////

/// # [ADD PORTS]
/// * `desc`: `用户贵宾删除端口`
#[async_trait]
pub trait VipDelPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 单个软删除
    async fn single_soft_del(
        &self,
        uid: i64, // 操作者ID
        id: i64,  // 目录用户ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 2. [PORT] - 批量软删除
    /// * `desc`: `管理员批量移除贵宾`
    async fn batch_soft_del(
        &self,
        uid: i64,      // 操作者ID
        ids: Vec<i64>, // 目标用户IDs
    ) -> anyhow::Result<(u16)>;
    async fn single_del(&self, uid: i64, id: i64) -> anyhow::Result<u16>;
    async fn batch_del(&self, uid: i64, ids: Vec<i64>) -> anyhow::Result<u16>;
}

//////// END
