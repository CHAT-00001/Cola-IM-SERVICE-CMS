// repo_adapter/src/user/blacklist.rs
// 2026-06-12

use async_trait::async_trait;
use cola_data::user::port::blacklist::BlacklistPort;
use cola_data::user::info::config::UserConfigInfo;
use cola_data::user::info::user::UserInfo;

pub struct UserBlacklistPortAdapter;

#[async_trait]
impl BlacklistPort for UserBlacklistPortAdapter {
    async fn get_config(&self, _user_id: i64) -> anyhow::Result<UserConfigInfo> {
        Err(anyhow::anyhow!("not implemented"))
    }
    async fn add_following(&self, _uid: i64, _user_id: i64) -> anyhow::Result<()> {
        Ok(())
    }
    async fn get_following(&self, _uid: i64) -> anyhow::Result<UserInfo> {
        Err(anyhow::anyhow!("not implemented"))
    }
    async fn del_following(&self, _uid: i64, _user_id: i64) -> anyhow::Result<()> {
        Ok(())
    }
    async fn batch_del_following(&self, _uid: i64, _user_ids: Vec<i64>) -> anyhow::Result<()> {
        Ok(())
    }
}