// port/src/user/ban/check.rs
// ⏩️ 端口 - 🗣 可乐用户 - 封禁 - 检查
// 2026/8/5 21:34 Created.

////////

use async_trait::async_trait;

////////

/// # [CHECK PORTS]
/// * `desc`: `封禁检查端口`
#[async_trait]
pub trait UserBanCheckPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 检查
    /// * `desc`: `检查是否已经拉黑`
    async fn is_baned(
        &self,
        uid: i64, // 操作者ID
        id: i64,  // 目标用户ID
    ) -> anyhow::Result<(bool)>;
}

//////// END
