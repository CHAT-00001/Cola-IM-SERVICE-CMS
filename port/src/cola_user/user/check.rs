// port/src/cola_user/user/check.rs
// 用户 - port - 用户 - 检查
// 2026/8/5 22:02 Created.

////////

use cola_data::cola_user::info::user::UserInfo;


////////

/// # [CHECK PORTS] - 检查
/// * `desc`: `检查用户状态`
#[async_trait::async_trait]
pub trait UserCheckPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [SERVICE] - 健康
    /// * `desc`: `检查用户健康状态`
    async fn check_health(
        &self,
        id: i64, // 目标用户ID
    ) -> anyhow::Result<(UserInfo)>;

    ////////

    /// # 2. [SERVICE] - 状态
    /// * `desc`: `检查用户机能状态`
    async fn check_state(
        &self,
        id: i64, // 目标用户ID
    ) -> anyhow::Result<(UserInfo)>;
}

//////// END
