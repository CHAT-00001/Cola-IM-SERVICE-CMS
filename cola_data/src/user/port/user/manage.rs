// user/port/user/manage.rs
// 用户 - port - 用户 - 管理
// 2026/8/5 22:02 Created.

////////

use crate::user::command::new::UserCommand;
use crate::user::command::user::update::UpdateUserCommand;
use crate::user::info::user::UserInfo;

////////

/// # [MANAGE PORTS] - 管理
/// * `desc`: `用户管理端口`
#[async_trait::async_trait]
pub trait UserManagePort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 修改用户权限
    async fn change_permission(
        &self,
        uid: i64,               // UID
        cmd: UpdateUserCommand, // 命令
    ) -> anyhow::Result<(UserInfo)>;

    ////////

    /// # 2. [PORT] - 修改用户状态
    async fn change_state(
        &self,
        uid: i64,               // UID
        cmd: UpdateUserCommand, // 命令
    ) -> anyhow::Result<(UserInfo)>;

    ////////

    /// # 3. [PORT] - 修改用户角色
    async fn change_role(
        &self,
        uid: i64,               // UID
        cmd: UpdateUserCommand, // 命令
    ) -> anyhow::Result<(UserInfo)>;

    ////////

    /// # 4. [PORT] - 修改用户类型
    async fn change_type(
        &self,
        uid: i64,               // UID
        cmd: UpdateUserCommand, // 命令
    ) -> anyhow::Result<(UserInfo)>;

    ////////
}

//////// END
