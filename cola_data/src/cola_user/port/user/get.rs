// /cola_user/port/cola_user/get.rs
// 用户 - port - 用户 - 获取
// 2026/8/5 22:02 Created.

////////

use crate::cola_user::info::user::UserInfo;

////////

/// # [GET PORTS]
/// `desc`: `获取用户资料`
#[async_trait::async_trait]
pub trait UserGetPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 单个获取
    /// * `desc`: `单个获取用户资料`
    async fn single_get_info(
        &self,
        id: i64, // 目标用户ID
    ) -> anyhow::Result<(UserInfo)>;

    ////////

    /// # [PORT] - 批量获取
    /// * `desc`: `批量获取用户资料列表`
    async fn batch_get_info(
        &self,
        ids: Vec<i64>, // 用户IDs
    ) -> anyhow::Result<(Vec<UserInfo>)>;
}

//////// END
