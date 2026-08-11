// /check.rs
// 
// 2026/8/10 23:28 Created.

////////

use async_trait::async_trait;
use port::cola_user::black::check::UserBlackCheckPort;



pub struct UserRoleCheckAdapter;
#[async_trait]
impl UserBlackCheckPort for UserRoleCheckAdapter {
    async fn is_blacked(&self, _uid: i64, _id: i64) -> anyhow::Result<bool> {
        Ok(false)
    }
}