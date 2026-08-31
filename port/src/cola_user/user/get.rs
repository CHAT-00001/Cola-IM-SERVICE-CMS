// port/src/user/user/get.rs
// 用户 - ⏩️ 端口 - USER - 用户 - 获取
// 2026/8/5 22:02 Created.

////////

////////

use cola_data::cola_user::info::user::UserInfo;

/// # [GET PORTS]
/// `desc`: `获取用户资料`
#[async_trait::async_trait]
pub trait UserGetPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 单个获取
    async fn single_get_info(
        &self,
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(UserInfo)>;

    ////////

    /// # 2. [PORT] - 批量获取
    async fn batch_get_infos(
        &self,
        user_ids: Vec<i64>, // 用户 IDs
    ) -> anyhow::Result<(Vec<UserInfo>)>;
}

//////// END
