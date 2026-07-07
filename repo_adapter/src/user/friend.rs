// repo_adapter/src/user/friend.rs
// 2026-06-12

use async_trait::async_trait;
use cola_data::user::info::config::UserConfigInfo;
use cola_data::user::info::user::UserInfo;
use cola_data::user::port::friend::FriendPort;

pub struct UserFriendPortAdapter;

#[async_trait]
impl FriendPort for UserFriendPortAdapter {
    ////////

    /// # [ADAPTER] - 获取配置
    async fn get_config(&self, _user_id: i64) -> anyhow::Result<UserConfigInfo> {
        Err(anyhow::anyhow!("not implemented"))
    }

    ////////

    /// # [ADAPTER] - 添加朋友
    async fn add_friend(&self, _uid: i64, _user_id: i64) -> anyhow::Result<()> {
        Ok(())
    }

    ////////

    /// # [ADAPTER] - 查看朋友
    async fn get_friend(&self, _uid: i64) -> anyhow::Result<UserInfo> {
        Err(anyhow::anyhow!("not implemented"))
    }

    ////////

    /// # [ADAPTER] - 单个删除
    async fn del_friend(&self, _uid: i64, _user_id: i64) -> anyhow::Result<()> {
        Ok(())
    }

    ////////

    /// # [ADAPTER] - 批量删除
    async fn batch_del_friend(&self, _uid: i64, _user_ids: Vec<i64>) -> anyhow::Result<()> {
        Ok(())
    }
}
