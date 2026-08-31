// repo_adapter/src/user/role/check.rs
// 🔌 适配器 - USER - 角色 - 检查
// 2026/8/10 23:28 Created.

////////

use async_trait::async_trait;
use port::cola_user::role::check::UserRoleCheckPort;

////////

pub struct UserRoleCheckAdapter;

#[async_trait]
impl UserRoleCheckPort for UserRoleCheckAdapter {
    async fn is_blacked(&self, _uid: i64, _id: i64) -> anyhow::Result<(bool)> {
        todo!()
    }
}

//////// END
