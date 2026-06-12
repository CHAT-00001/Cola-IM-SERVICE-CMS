// repo_adapter/src/user/friend.rs
// 2026-06-12

use async_trait::async_trait;
use cola_data::user::port::friend::FriendPort;
use cola_data::user::info::config::UserConfigInfo;
use cola_data::user::info::user::UserInfo;

pub struct UserFriendPortAdapter;

#[async_trait]
impl FriendPort for UserFriendPortAdapter {
    async fn get_config(&self, _user_id: i64) -> anyhow::Result<UserConfigInfo> {
        Err(anyhow::anyhow!("not implemented"))
    }
    async fn add_friend(&self, _uid: i64, _user_id: i64) -> anyhow::Result<()> {
        Ok(())
    }
    async fn get_friend(&self, _uid: i64) -> anyhow::Result<UserInfo> {
        Err(anyhow::anyhow!("not implemented"))
    }
    async fn del_friend(&self, _uid: i64, _user_id: i64) -> anyhow::Result<()> {
        Ok(())
    }
    async fn batch_del_friend(&self, _uid: i64, _user_ids: Vec<i64>) -> anyhow::Result<()> {
        Ok(())
    }
}