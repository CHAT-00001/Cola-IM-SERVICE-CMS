// port/src/user/follow/manage.rs -- 端口 - USER - 关注 - 管理端口
// 2026/8/5 21:58 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::follow::UserFollowInfo;

////////

/// # [MANAGE PORTS]
/// * `desc`: `用户关注管理端口`
#[async_trait]
pub trait UserFollowManagePort: Send + Sync + 'static {
    //

    ////////

    /// # 7. [PORT] - 获取用户的黑名单
    /// * `return` : `user ids`
    async fn admin_list(
        &self,
        user_id: i64, // 目标用户ID
        offset: i64,  // 页数
        limit: i64,   // 数量
    ) -> anyhow::Result<(Vec<UserFollowInfo>)>;
}

//////// END
