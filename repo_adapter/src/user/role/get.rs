// repo_adapter/src/user/role/get.rs
// 🔌 适配器 - USER - 角色 - 获取
// 2026/8/10 23:25 Created.

////////

use async_trait::async_trait;
use port::cola_user::role::get::UserRoleGetPort;

////////

pub struct UserRoleGetAdapter;

#[async_trait]
impl UserRoleGetPort for UserRoleGetAdapter {
    async fn get_my_black_ids(
        &self,
        _uid: i64,
        _id: i64,
        _limit: i64,
        _offset: i64,
    ) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }

    async fn get_he_black_ids(
        &self,
        _uid: i64,
        _id: i64,
        _limit: i64,
        _offset: i64,
    ) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }

    async fn get_black_me_ids(
        &self,
        _uid: i64,
        _id: i64,
        _limit: i64,
        _offset: i64,
    ) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }

    async fn get_black_he_ids(
        &self,
        _uid: i64,
        _id: i64,
        _limit: i64,
        _offset: i64,
    ) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }
}

//////// END
