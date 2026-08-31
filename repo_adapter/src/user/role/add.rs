// repo_adapter/src/user/role/add.rs
// 🔌 适配器 - USER - 角色 - 发布
// 2026/8/6 04:20 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::user::UserInfo;
use port::cola_user::role::add::UserRoleAddPort;

////////

pub struct RoleAddAdapter;

////////

#[async_trait]
impl UserRoleAddPort for RoleAddAdapter {
    async fn add_black(&self, _uid: i64, _id: i64) -> anyhow::Result<()> {
        todo!()
    }

    async fn del_black(&self, _uid: i64, _id: i64) -> anyhow::Result<()> {
        todo!()
    }

    async fn get_following(&self, _uid: i64) -> anyhow::Result<(UserInfo)> {
        todo!()
    }

    async fn single_del(&self, _uid: i64, _id: i64) -> anyhow::Result<(u16)> {
        todo!()
    }

    async fn batch_del(&self, _uid: i64, _ids: Vec<i64>) -> anyhow::Result<(u16)> {
        todo!()
    }

    async fn check_state(&self, _uid: i64, _user_id: i64) -> anyhow::Result<(bool)> {
        todo!()
    }

    async fn get_list_by_user_id(
        &self,
        _user_id: i64,
        _offset: i64,
        _limit: i64,
    ) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }

    async fn get_here_list(&self, _uid: i64, _user_ids: Vec<i64>) -> anyhow::Result<()> {
        todo!()
    }
}

//////// END
