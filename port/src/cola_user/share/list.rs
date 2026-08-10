// port/src/cola_user/share/list.rs
// ⏩️ 端口 - 🗣 用户 - 分享 - 类别
// 2026/8/5 21:36 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::share::ShareInfo;

////////

/// # [LIST PORT]
/// * `desc`: `🗣 用户 - 用户分享记录列表列表端口`
#[async_trait]
pub trait UserShareListPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 用户的
    /// * `desc` : `🗣 USER` - `根据用户ID - 查看TA的分享的记录信息`
    async fn get_share_infos_by_user_id(
        &self,
        user_id: i64, // 目标用户ID
        offset: i64,  // 页数
        limit: i64,   // 数量
    ) -> anyhow::Result<(Vec<ShareInfo>)>;

    ////////

    /// # 1. [PORT] - 资料的
    /// * `desc` : `🗣 USER` - `根据资料ID - 查看被谁分享的记录信息`
    async fn get_share_infos_by_profile_id(
        &self,
        user_id: i64, // 目标用户ID
        offset: i64,  // 页数
        limit: i64,   // 数量
    ) -> anyhow::Result<(Vec<ShareInfo>)>;
}

//////// END
