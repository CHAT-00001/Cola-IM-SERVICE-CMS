// port/src/user/view/list.rs
// ⏩️ 端口 - USER - 浏览 - 列表
// 2026/8/6 00:44 Created.

////////

use async_trait::async_trait;

////////

/// # [LIST PORT]
/// * `desc`: `用户浏览列表端口`
#[async_trait]
pub trait UserViewListPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 用户主动看的
    /// * `desc` : `根据用户ID` - `批量获取用户浏览记录信息`
    async fn get_view_infos_by_user_id(
        &self,
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页数
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 1. [PORT] - 资料被动看的
    /// * `desc` : `根据资料ID` - `批量获取资料被浏览记录信息`
    async fn get_view_infos_by_profile_id(
        &self,
        profile_id: i64, // 资料 ID
        limit: i64,      // 数量
        offset: i64,     // 页数
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END
