// port/src/user/share/list.rs -- 端口 - USER - 分享 - 列表端口
// 2026/8/5 21:36 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::share::UserShareInfo;

////////

/// # [LIST PORT]
/// * `desc`: `COLA USER - Share List Ports`
#[async_trait]
pub trait UserShareListPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 用户的
    /// * `desc` : `根据用户ID` - `查看TA的分享的记录信息`
    async fn get_share_infos_by_user_id(
        &self,
        user_id: i64, // 用户 ID
        offset: i64,  // 页数
        limit: i64,   // 数量
    ) -> anyhow::Result<(Vec<UserShareInfo>)>;

    ////////

    /// # 1. [PORT] - 资料的
    /// * `desc` : `根据资料ID` - `查看被谁分享的记录信息`
    async fn get_share_infos_by_profile_id(
        &self,
        profile_id: i64, // 资料 ID
        offset: i64,     // 页数
        limit: i64,      // 数量
    ) -> anyhow::Result<(Vec<UserShareInfo>)>;
}

//////// END
