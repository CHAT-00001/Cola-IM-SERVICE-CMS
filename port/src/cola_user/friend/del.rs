// user/port/friend/del.rs -- 端口 - USER - 朋友 - 删除端口
// 2026/8/5 21:58 Created.

////////

use async_trait::async_trait;

////////

/// # [ADD PORTS]
/// * `desc`: `用户朋友删除端口`
#[async_trait]
pub trait FriendDelPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 单个删除
    async fn single_delete(
        &self,
        uid: i64, // 操作者ID
        id: i64,  // 目录用户ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 2. [PORT] - 批量删除
    /// * `desc`: `管理员批量移除黑名单`
    async fn batch_delete(
        &self,
        uid: i64,      // 操作者ID
        ids: Vec<i64>, // 目标用户IDs
    ) -> anyhow::Result<(u16)>;
}

//////// END
