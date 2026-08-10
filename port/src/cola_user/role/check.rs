// port/src/cola_user/role/check.rs
// ⏩️ 端口 - 🗣 用户 - 角色 - 检查
// 2026/8/5 21:34 Created.

////////

use async_trait::async_trait;

////////

/// # [CHECK PORTS]
/// * `desc`: `黑名单检查端口`
#[async_trait]
pub trait UserRoleCheckPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 检查
    /// * `desc`: `检查是否已经拉黑`
    async fn is_blacked(
        &self,
        uid: i64, // 操作者ID
        id: i64,  // 目标用户ID
    ) -> anyhow::Result<(bool)>;
}

//////// END
