// repo_adapter/src/cola_user/follow/mod.rs
// 🔌 插头 - 可乐用户 - 关注 - 模块
// 2026/8/6 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::port::follow::add::FollowAddPort;
use cola_data::cola_user::port::follow::check::FollowCheckPort;
use cola_data::cola_user::port::follow::del::FollowDelPort;
use cola_data::cola_user::port::follow::get::FollowGetPort;
use cola_data::cola_user::port::follow::list::FollowListPort;
use cola_data::cola_user::port::follow::manage::FollowManagePort;

pub mod add;

////////

/// # [ADAPTER] - 关注适配器
pub struct FollowAdapter;

#[async_trait]
impl FollowAddPort for FollowAdapter {
    async fn upsert_follow_record(&self, uid: i64, target_id: i64) -> anyhow::Result<()> {
        add::follow(uid, target_id).await
    }
    async fn del_follow(&self, uid: i64, target_id: i64) -> anyhow::Result<()> {
        add::unfollow(uid, target_id).await
    }
    async fn get_following(&self, _uid: i64) -> anyhow::Result<cola_data::cola_user::info::user::UserInfo> {
        Err(anyhow::anyhow!("not implemented"))
    }
    async fn single_del(&self, _uid: i64, _id: i64) -> anyhow::Result<u16> { Ok(0) }
    async fn batch_del(&self, _uid: i64, _ids: Vec<i64>) -> anyhow::Result<u16> { Ok(0) }
    async fn check_state(&self, uid: i64, target_id: i64) -> anyhow::Result<bool> {
        add::check_followed(uid, target_id).await
    }
    async fn get_list_by_user_id(&self, uid: i64, offset: i64, limit: i64) -> anyhow::Result<Vec<i64>> {
        add::get_follow_ids(uid, offset, limit).await
    }
    async fn get_here_list(&self, _uid: i64, _user_ids: Vec<i64>) -> anyhow::Result<()> { Ok(()) }
}

#[async_trait]
impl FollowCheckPort for FollowAdapter {
    async fn is_followed(&self, uid: i64, target_id: i64) -> anyhow::Result<bool> {
        add::check_followed(uid, target_id).await
    }
}

#[async_trait]
impl FollowDelPort for FollowAdapter {
    async fn single_soft_del(&self, uid: i64, target_id: i64) -> anyhow::Result<u16> {
        add::unfollow(uid, target_id).await?;
        Ok(1)
    }
    async fn batch_soft_del(&self, _uid: i64, _ids: Vec<i64>) -> anyhow::Result<u16> { Ok(0) }
}

#[async_trait]
impl FollowGetPort for FollowAdapter {
    async fn get_my_follow_ids(&self, uid: i64, _id: i64, limit: i64, offset: i64) -> anyhow::Result<Vec<i64>> {
        add::get_follow_ids(uid, offset, limit).await
    }
    async fn get_he_follow_ids(&self, _uid: i64, _id: i64, _limit: i64, _offset: i64) -> anyhow::Result<Vec<i64>> {
        Ok(vec![])
    }

    async fn get_follow_me_ids(&self, uid: i64, id: i64, limit: i64, offset: i64) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }

    async fn get_follow_he_ids(&self, uid: i64, id: i64, limit: i64, offset: i64) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }
}

#[async_trait]
impl FollowListPort for FollowAdapter {
    async fn get_config(&self, _user_id: i64) -> anyhow::Result<cola_data::cola_user::info::config::UserConfigInfo> {
        Err(anyhow::anyhow!("not implemented"))
    }
    async fn add_black(&self, _uid: i64, _id: i64) -> anyhow::Result<()> { Ok(()) }
    async fn del_black(&self, _uid: i64, _id: i64) -> anyhow::Result<()> { Ok(()) }
    async fn get_following(&self, _uid: i64) -> anyhow::Result<cola_data::cola_user::info::user::UserInfo> {
        Err(anyhow::anyhow!("not implemented"))
    }
    async fn single_del(&self, _uid: i64, _id: i64) -> anyhow::Result<u16> { Ok(0) }
    async fn batch_del(&self, _uid: i64, _ids: Vec<i64>) -> anyhow::Result<u16> { Ok(0) }
    async fn check_state(&self, _uid: i64, _user_id: i64) -> anyhow::Result<bool> { Ok(false) }
    async fn get_list_by_user_id(&self, _user_id: i64, _offset: i64, _limit: i64) -> anyhow::Result<Vec<i64>> { Ok(vec![]) }
    async fn get_here_list(&self, _uid: i64, _user_ids: Vec<i64>) -> anyhow::Result<()> { Ok(()) }
}

#[async_trait]
impl FollowManagePort for FollowAdapter {
    async fn get_config(&self, _user_id: i64) -> anyhow::Result<cola_data::cola_user::info::config::UserConfigInfo> {
        Err(anyhow::anyhow!("not implemented"))
    }
    async fn add_black(&self, _uid: i64, _id: i64) -> anyhow::Result<()> { Ok(()) }
    async fn del_black(&self, _uid: i64, _id: i64) -> anyhow::Result<()> { Ok(()) }
    async fn get_following(&self, _uid: i64) -> anyhow::Result<cola_data::cola_user::info::user::UserInfo> {
        Err(anyhow::anyhow!("not implemented"))
    }
    async fn single_del(&self, _uid: i64, _id: i64) -> anyhow::Result<u16> { Ok(0) }
    async fn batch_del(&self, _uid: i64, _ids: Vec<i64>) -> anyhow::Result<u16> { Ok(0) }
    async fn check_state(&self, _uid: i64, _user_id: i64) -> anyhow::Result<bool> { Ok(false) }
    async fn get_list_by_user_id(&self, _user_id: i64, _offset: i64, _limit: i64) -> anyhow::Result<Vec<i64>> { Ok(vec![]) }
    async fn get_here_list(&self, _uid: i64, _user_ids: Vec<i64>) -> anyhow::Result<()> { Ok(()) }
}

//////// END