// /del.rs
//
// 2026/8/10 23:24 Created.

////////

use async_trait::async_trait;
use port::cola_user::black::del::UserBlackDelPort;

pub struct UserRoleDelAdapter;

#[async_trait]
impl UserBlackDelPort for UserRoleDelAdapter {
    async fn single_soft_del(&self, _uid: i64, _id: i64) -> anyhow::Result<u16> {
        Ok(0)
    }
    async fn batch_soft_del(&self, _uid: i64, _ids: Vec<i64>) -> anyhow::Result<u16> {
        Ok(0)
    }
}
