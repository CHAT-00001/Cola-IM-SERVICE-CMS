// repo_adapter/src/user/following.rs
// 2026-06-12

use async_trait::async_trait;
use cola_data::user::port::following::FollowingPort;
use cola_data::user::info::config::UserConfigInfo;
use cola_data::user::info::user::UserInfo;

pub struct UserFollowingPortAdapter;

#[async_trait]
impl FollowingPort for UserFollowingPortAdapter {
    async fn get_config(&self, _user_id: i64) -> anyhow::Result<UserConfigInfo> {
        Err(anyhow::anyhow!("not implemented"))
    }
    async fn add_following(&self, _uid: i64, _user_id: i64) -> anyhow::Result<()> {
        Ok(())
    }
    async fn get_following_ids(&self, _uid: i64) -> anyhow::Result<Vec<i64>> {
        Ok(vec![])
    }
    async fn get_following_infos(&self, _uid: i64) -> anyhow::Result<(Vec<UserInfo>, Option<i64>)> {
        Ok((vec![], None))
    }
    async fn del_following(&self, _uid: i64, _user_id: i64) -> anyhow::Result<()> {
        Ok(())
    }
    async fn batch_del_following(&self, _uid: i64, _user_ids: Vec<i64>) -> anyhow::Result<()> {
        Ok(())
    }
}