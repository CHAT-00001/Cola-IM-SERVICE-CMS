// repo_adapter/src/user/user/manage.rs -- 适配器 - USER - 用户资料 - 管理适配器
// 2026/8/6 04:19 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::command::user::update::UpdateUserCommand;
use cola_data::cola_user::info::user::UserInfo;
use port::cola_user::profile::manage::UserManagePort;

////////

/// # [LIST SERVICE] - 用户资料管理适配器
/// * `desc`: `COLA USER - Profile Manage Adapter.`
pub struct UserManageAdapter;

// 构造实现
#[async_trait]
impl UserManagePort for UserManageAdapter {
    //

    ////////

    /// # 1. [SERVICE] - 最新
    /// * `desc`: `保存新用户记录`
    async fn change_permission(
        &self,
        uid: i64,
        cmd: UpdateUserCommand,
    ) -> anyhow::Result<(UserInfo)> {
        todo!()
    }

    async fn change_state(&self, uid: i64, cmd: UpdateUserCommand) -> anyhow::Result<(UserInfo)> {
        todo!()
    }

    async fn change_role(&self, uid: i64, cmd: UpdateUserCommand) -> anyhow::Result<(UserInfo)> {
        todo!()
    }

    async fn change_type(&self, uid: i64, cmd: UpdateUserCommand) -> anyhow::Result<(UserInfo)> {
        todo!()
    }
}

//////// END
