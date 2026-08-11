// /add.rs
// 
// 2026/8/6 04:20 Created.

////////

use async_trait::async_trait;
use port::cola_user::black::add::UserBlackAddPort;

pub struct RoleAddAdapter;

////////

#[async_trait]
impl UserBlackAddPort for RoleAddAdapter {
    async fn add_black(&self, _uid: i64, _target_id: i64) -> anyhow::Result<(), > {
        Ok(())
    }
    async fn del_black(&self, _uid: i64, _id: i64) -> anyhow::Result<(), > {
        Ok(())
    }
}