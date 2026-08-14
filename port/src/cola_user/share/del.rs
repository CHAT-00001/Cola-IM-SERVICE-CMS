// port/src/user/share/del.rs
// ⏩️ 端口 - 🗣 用户 - 分享 - 删除
// 2026/8/5 21:38 Created.

////////

use async_trait::async_trait;

////////

/// # [DEL PORTS]
/// * `desc`: `🗣 用户 - 用户分享记录删除端口`
#[async_trait]
pub trait UserShareDelPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 单个软删除
    async fn single_delete(
        &self,
        uid: i64, // 操作者ID
        id: i64,  // 目录用户ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 2. [PORT] - 批量软删除
    /// * `desc`: `管理员批量移除黑名单`
    async fn batch_delete(
        &self,
        uid: i64,      // 操作者ID
        ids: Vec<i64>, // 目标用户IDs
    ) -> anyhow::Result<(u16)>;
}

//////// END
