// port/src/user/role/add.rs -- 端口 - USER - 角色 - 发布
// 2026/8/5 21:33 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::command::role::{UserRoleCreateCmd, UserRoleUpdateCmd};
use cola_data::cola_user::info::role::RoleInfo;
use cola_data::cola_user::info::user::UserInfo;

////////

/// # [ADD PORTS] - 用户角色发布端口
/// * `desc`: `COLA USER - Role Add Ports`
#[async_trait]
pub trait UserRoleAddPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 创建角色
    async fn create_role(
        &self,
        uid: i64,               // UID
        cmd: UserRoleCreateCmd, // 角色创建命令
    ) -> anyhow::Result<(RoleInfo)>;

    ////////

    /// # 2. [PORT] - 更新角色
    async fn update_role(
        &self,
        uid: i64,               // 操作者 ID
        role_id: i64,           // 角色 ID
        cmd: UserRoleUpdateCmd, // 角色更新命令
    ) -> anyhow::Result<(RoleInfo)>;

    ////////

    /// # 3. [PORT] - 移除角色
    async fn delete_role(
        &self,
        uid: i64, // UID
        id: i64,  // 目标用户ID
    ) -> anyhow::Result<()>;

    ////////

    /// # 4. [PORT] - 修改状态
    async fn change_status(
        &self,
        uid: i64,    // 操作者 ID
        status: i16, // 状态码
    ) -> anyhow::Result<(UserInfo)>;

    ////////

    /// # 4. [PORT] - 单个移除
    async fn single_delete(
        &self,
        uid: i64,     // 操作者ID
        role_id: i64, // 角色 ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 5. [PORT] - 批量移除
    /// * `desc`: `管理员批量移除黑名单`
    async fn batch_delete(
        &self,
        uid: i64,           // 操作者ID
        role_ids: Vec<i64>, // 角色 IDs
    ) -> anyhow::Result<(u16)>;
}

//////// END
