// cola_user/port/friend/check.rs
// 用户 - port - 朋友 - 检查
// 2026/8/5 21:58 Created.

////////

use async_trait::async_trait;

////////

/// # [CHECK PORTS]
/// * `desc`: `朋友检查端口`
#[async_trait]
pub trait FriendCheckPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 检查
    /// * `desc`: `检查是否已经朋友`
    async fn is_friended(
        &self,
        uid: i64, // 操作者ID
        id: i64,  // 目标用户ID
    ) -> anyhow::Result<(bool)>;
}

//////// END
