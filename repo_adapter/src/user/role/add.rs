// repo_adapter/src/user/role/add.rs -- 适配器 - USER - 角色 - 发布适配器
// 2026/8/6 04:20 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::command::role::{UserRoleCreateCmd, UserRoleUpdateCmd};
use cola_data::cola_user::info::role::RoleInfo;
use cola_data::cola_user::info::user::UserInfo;
use port::cola_user::role::add::UserRoleAddPort;

////////

pub struct RoleAddAdapter;

////////

#[async_trait]
impl UserRoleAddPort for RoleAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 创建角色
    async fn create_role(
        &self,
        uid: i64,               // 操作者 ID
        cmd: UserRoleCreateCmd, // 角色创建命令
    ) -> anyhow::Result<(RoleInfo)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 更新角色
    async fn update_role(
        &self,
        uid: i64,               // 操作者 ID
        role_id: i64,           // 角色 ID
        cmd: UserRoleUpdateCmd, // 角色更新命令
    ) -> anyhow::Result<(RoleInfo)> {
        todo!()
    }

    ////////

    /// # 3. [ADAPTER] - 删除角色
    async fn delete_role(&self, uid: i64, id: i64) -> anyhow::Result<()> {
        todo!()
    }

    ////////

    /// # 4. [ADAPTER] - 修改状态
    async fn change_status(&self, uid: i64, status: i16) -> anyhow::Result<(UserInfo)> {
        todo!()
    }

    ////////

    /// # 5. [ADAPTER] - 单个删除
    async fn single_delete(&self, uid: i64, role_id: i64) -> anyhow::Result<(u16)> {
        todo!()
    }

    ////////

    /// # 6. [ADAPTER] - 批量删除
    async fn batch_delete(&self, uid: i64, role_ids: Vec<i64>) -> anyhow::Result<(u16)> {
        todo!()
    }
}

//////// END
