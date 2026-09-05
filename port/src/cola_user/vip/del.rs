// port/src/user/vip/del.rs -- 端口 - USER - 贵宾 - 删除端口
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

    /// # 1. [PORT] - 单个删除
    async fn single_delete(
        &self,
        uid: i64, // 操作者ID
        id: i64,  // 目录用户ID
    ) -> anyhow::Result<(u64)>;

    ////////

    /// # 2. [PORT] - 批量删除
    /// * `desc`: `管理员批量移除贵宾`
    async fn batch_delete(
        &self,
        uid: i64,      // 操作者 ID
        ids: Vec<i64>, // 目标 IDs
    ) -> anyhow::Result<(u64)>;
}

//////// END
