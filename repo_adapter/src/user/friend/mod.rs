// repo_adapter/src/user/friend/mod.rs
// 🔌 插头 - 可乐用户 - 朋友 - 模块
// 2026/8/6 10:14 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::config::UserConfigInfo;
use cola_data::cola_user::info::user::UserInfo;
use port::cola_user::friend::add::FriendAddPort;
use port::cola_user::friend::check::FriendCheckPort;
use port::cola_user::friend::del::FriendDelPort;
use port::cola_user::friend::get::FriendGetPort;
use port::cola_user::friend::list::FriendListPort;
use port::cola_user::friend::manage::FriendManagePort;

////////
pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [ADAPTER] - 朋友统一适配器
pub struct FriendAdapter;

////////

#[async_trait]
impl FriendAddPort for FriendAdapter {
    async fn upsert_friend_record(&self, uid: i64, id: i64) -> anyhow::Result<()> {
        add::upsert_friend(uid, id).await
    }
    async fn del_friend(&self, uid: i64, id: i64) -> anyhow::Result<()> {
        add::del_friend(uid, id).await
    }
    async fn get_friending(&self, uid: i64) -> anyhow::Result<UserInfo> {
        get::get_friending(uid).await
    }
    async fn single_del(&self, uid: i64, id: i64) -> anyhow::Result<u16> {
        del::single_del(uid, id).await
    }
    async fn batch_del(&self, uid: i64, ids: Vec<i64>) -> anyhow::Result<u16> {
        del::batch_del(uid, ids).await
    }
    async fn check_state(&self, uid: i64, user_id: i64) -> anyhow::Result<bool> {
        check::check_state(uid, user_id).await
    }
    async fn get_list_by_user_id(
        &self,
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<i64>> {
        get::get_list(user_id, offset, limit).await
    }
    async fn get_here_list(&self, uid: i64, user_ids: Vec<i64>) -> anyhow::Result<()> {
        get::get_here_list(uid, user_ids).await
    }
}

////////

#[async_trait]
impl FriendCheckPort for FriendAdapter {
    async fn is_friended(&self, uid: i64, id: i64) -> anyhow::Result<bool> {
        check::is_friended(uid, id).await
    }
}

////////

#[async_trait]
impl FriendDelPort for FriendAdapter {
    async fn single_soft_del(&self, uid: i64, id: i64) -> anyhow::Result<u16> {
        del::single_del(uid, id).await
    }
    async fn batch_soft_del(&self, uid: i64, ids: Vec<i64>) -> anyhow::Result<u16> {
        del::batch_del(uid, ids).await
    }
}

////////

#[async_trait]
impl FriendGetPort for FriendAdapter {
    async fn get_my_friend_ids(
        &self,
        uid: i64,
        id: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<i64>> {
        get::get_my_friend_ids(uid, id, limit, offset).await
    }
    async fn get_he_friend_ids(
        &self,
        uid: i64,
        id: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<i64>> {
        get::get_he_friend_ids(uid, id, limit, offset).await
    }
    async fn get_friend_me_ids(
        &self,
        uid: i64,
        id: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<i64>> {
        get::get_friend_me_ids(uid, id, limit, offset).await
    }
    async fn get_friend_he_ids(
        &self,
        uid: i64,
        id: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<i64>> {
        get::get_friend_he_ids(uid, id, limit, offset).await
    }
}

////////

#[async_trait]
impl FriendListPort for FriendAdapter {
    async fn get_config(&self, user_id: i64) -> anyhow::Result<UserConfigInfo> {
        list::get_config(user_id).await
    }
    async fn add_black(&self, uid: i64, id: i64) -> anyhow::Result<()> {
        add::upsert_friend(uid, id).await
    }
    async fn del_black(&self, uid: i64, id: i64) -> anyhow::Result<()> {
        add::del_friend(uid, id).await
    }
    async fn get_friending(&self, uid: i64) -> anyhow::Result<UserInfo> {
        get::get_friending(uid).await
    }
    async fn single_del(&self, uid: i64, id: i64) -> anyhow::Result<u16> {
        del::single_del(uid, id).await
    }
    async fn batch_del(&self, uid: i64, ids: Vec<i64>) -> anyhow::Result<u16> {
        del::batch_del(uid, ids).await
    }
    async fn check_state(&self, uid: i64, user_id: i64) -> anyhow::Result<bool> {
        check::check_state(uid, user_id).await
    }
    async fn get_list_by_user_id(
        &self,
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<i64>> {
        get::get_list(user_id, offset, limit).await
    }
    async fn get_here_list(&self, uid: i64, user_ids: Vec<i64>) -> anyhow::Result<()> {
        get::get_here_list(uid, user_ids).await
    }
}

////////

#[async_trait]
impl FriendManagePort for FriendAdapter {
    async fn get_config(&self, user_id: i64) -> anyhow::Result<UserConfigInfo> {
        list::get_config(user_id).await
    }
    async fn add_black(&self, uid: i64, id: i64) -> anyhow::Result<()> {
        add::upsert_friend(uid, id).await
    }
    async fn del_black(&self, uid: i64, id: i64) -> anyhow::Result<()> {
        add::del_friend(uid, id).await
    }
    async fn get_friending(&self, uid: i64) -> anyhow::Result<UserInfo> {
        get::get_friending(uid).await
    }
    async fn single_del(&self, uid: i64, id: i64) -> anyhow::Result<u16> {
        manage::single_del(uid, id).await
    }
    async fn batch_del(&self, uid: i64, ids: Vec<i64>) -> anyhow::Result<u16> {
        manage::batch_del(uid, ids).await
    }
    async fn check_state(&self, uid: i64, user_id: i64) -> anyhow::Result<bool> {
        check::check_state(uid, user_id).await
    }
    async fn get_list_by_user_id(
        &self,
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<i64>> {
        get::get_list(user_id, offset, limit).await
    }
    async fn get_here_list(&self, uid: i64, user_ids: Vec<i64>) -> anyhow::Result<()> {
        get::get_here_list(uid, user_ids).await
    }
}

//////// END
