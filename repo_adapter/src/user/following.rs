// repo_adapter/src/user/following.rs
// 2026-06-12

use async_trait::async_trait;
use cola_data::user::info::config::UserConfigInfo;
use cola_data::user::info::user::UserInfo;
use cola_data::user::port::following::FollowingPort;

pub struct UserFollowingPortAdapter;

#[async_trait]
impl FollowingPort for UserFollowingPortAdapter {
    ////////

    /// # [ADAPTER] - 获取配置
    async fn get_config(&self, _user_id: i64) -> anyhow::Result<UserConfigInfo> {
        Err(anyhow::anyhow!("not implemented"))
    }

    /// # [ADAPTER] - 添加关注
    async fn add_following(&self, _uid: i64, _user_id: i64) -> anyhow::Result<()> {
        Ok(())
    }

    ////////

    /// # [ADAPTER] - 获取关注的用户IDs
    async fn get_following_ids(&self, _uid: i64) -> anyhow::Result<Vec<i64>> {
        Ok(vec![])
    }

    ////////

    /// # [ADAPTER] - 获取关注的用户信息
    async fn get_following_infos(&self, _uid: i64) -> anyhow::Result<(Vec<UserInfo>, Option<i64>)> {
        Ok((vec![], None))
    }

    ////////

    /// # [ADAPTER] - 删除关注
    async fn del_following(&self, _uid: i64, _user_id: i64) -> anyhow::Result<()> {
        Ok(())
    }

    ////////

    /// # [ADAPTER] - 批量删除
    async fn batch_del_following(&self, _uid: i64, _user_ids: Vec<i64>) -> anyhow::Result<()> {
        Ok(())
    }
}


/////// END