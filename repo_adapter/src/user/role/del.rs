// repo_adapter/src/user/role/del.rs
// 🔌 适配器 - USER - 角色 - 删除
// 2026/8/10 23:24 Created.

////////

use async_trait::async_trait;
use port::cola_user::role::del::UserRoleDelPort;

////////

pub struct UserRoleDelAdapter;

#[async_trait]
impl UserRoleDelPort for UserRoleDelAdapter {
    async fn single_soft_del(&self, _uid: i64, _id: i64) -> anyhow::Result<(u16)> {
        todo!()
    }

    async fn batch_soft_del(&self, _uid: i64, _ids: Vec<i64>) -> anyhow::Result<(u16)> {
        todo!()
    }
}

//////// END
