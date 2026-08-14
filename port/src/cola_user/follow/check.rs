// port/src/user/port/follow/check.rs
// ⏩️ 端口 - 🗣 用户 - 关注 - 检查
// 2026/8/5 21:58 Created.

////////

use async_trait::async_trait;

////////

/// # [CHECK PORTS]
/// * `desc`: `关注检查端口`
#[async_trait]
pub trait UserFollowCheckPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 是否关注
    /// * `desc`: `检查是否已经关注`
    async fn is_followed(
        &self,
        uid: i64,     // 操作者ID
        user_id: i64, // 目标用户ID
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 2. [PORT] - 检查状态
    /// * `desc`: `根据uid + 用户ID` - `检查是否关注`
    async fn check_state(
        &self,
        uid: i64,     // 操作者ID
        user_id: i64, // 目标用户ID
    ) -> anyhow::Result<(bool)>;
}

//////// END
