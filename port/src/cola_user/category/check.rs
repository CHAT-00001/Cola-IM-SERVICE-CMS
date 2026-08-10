// port/src/cola_user/category/check.rs
// 用户 - port - 关注 - 检查
// 2026/8/5 21:58 Created.

////////

use async_trait::async_trait;

////////

/// # [CHECK PORTS]
/// * `desc`: `关注检查端口`
#[async_trait]
pub trait UserCategoryCheckPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 检查
    /// * `desc`: `检查是否已经关注`
    async fn is_followed(
        &self,
        uid: i64, // 操作者ID
        id: i64,  // 目标用户ID
    ) -> anyhow::Result<(bool)>;
}

//////// END
