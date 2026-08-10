// repo_adapter/src/cola_user/follow/manage.rs
// 🔌 插头 - 可乐用户 - 关注 - 模块
// 2026/8/10 04:16 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::follow::UserFollowInfo;
use port::cola_user::follow::manage::UserFollowManagePort;

////////

/// # [MANAGE ADAPTER] - 管理员
/// * `DESC`: `🗣 用户 - 管理员列表适配器`
pub struct UserFollowManageAdapter;

#[async_trait]
impl UserFollowManagePort for UserFollowManageAdapter {
    //

    ////////

    /// # [ADAPTER] - 管理员列表
    async fn admin_list(
        &self,
        user_id: i64, // 用户 ID
        offset: i64,  // 页码
        limit: i64,   // 数量
    ) -> anyhow::Result<(Vec<UserFollowInfo>)> {
        todo!()
    }
}

//////// END
