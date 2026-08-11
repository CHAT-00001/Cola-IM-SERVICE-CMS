// /get.rs
//
// 2026/8/10 23:25 Created.

////////

use async_trait::async_trait;
use port::cola_user::black::get::UserBlackGetPort;

pub struct UserRoleGetAdapter;
#[async_trait]
impl UserBlackGetPort for UserRoleGetAdapter {
    async fn get_black_ids(
        &self,
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }

    async fn get_black_me_ids(
        &self,
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }
}
