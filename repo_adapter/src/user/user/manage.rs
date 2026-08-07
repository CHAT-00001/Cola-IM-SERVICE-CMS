// repo_adapter/src/user/user/manage.rs
// 🔌 适配器 - 用户 - 用户 - 列表服务
// 2026/8/6 04:19 Created.

////////

use async_trait::async_trait;
use cola_data::user::command::user::update::UpdateUserCommand;
use cola_data::user::info::user::UserInfo;
use cola_data::user::port::user::manage::UserManagePort;

////////

/// # [LIST SERVICE] - 管理
/// * `desc`: `用户管理服务`
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
