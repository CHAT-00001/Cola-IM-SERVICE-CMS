// repo_adapter/src/user/vip/music.rs
// 🔌 插头 - 可乐用户 - 贵宾 - 模块
// 2026/8/6 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::user::UserInfo;
use port::cola_user::vip::add::VipAddPort;
use port::cola_user::vip::check::VipCheckPort;
use port::cola_user::vip::del::VipDelPort;
use port::cola_user::vip::get::VipGetPort;
use port::cola_user::vip::list::VipListPort;
use port::cola_user::vip::manage::VipManagePort;

////////

pub mod add;
pub mod alive;
pub mod check;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;
pub mod stat;

////////

/// # [VIP ADAPTER] - VIP 统一适配器
pub struct VipAdapter;

#[async_trait]
impl VipAddPort for VipAdapter {
    async fn add_black(&self, uid: i64, target_id: i64) -> anyhow::Result<()> {
        add::add_vip(uid, target_id).await
    }
    async fn del_black(&self, uid: i64, target_id: i64) -> anyhow::Result<()> {
        add::cancel_vip(uid, target_id).await
    }
    async fn get_following(&self, _uid: i64) -> anyhow::Result<UserInfo> {
        Err(anyhow::anyhow!("not implemented"))
    }
    async fn single_del(&self, _uid: i64, _id: i64) -> anyhow::Result<u16> {
        Ok(0)
    }
    async fn batch_del(&self, _uid: i64, _ids: Vec<i64>) -> anyhow::Result<u16> {
        Ok(0)
    }
    async fn check_state(&self, uid: i64, user_id: i64) -> anyhow::Result<bool> {
        check::check_state(uid, user_id).await
    }
    async fn get_list_by_user_id(
        &self,
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<i64>> {
        get::get_vip_ids(uid, offset, limit).await
    }
    async fn get_here_list(&self, _uid: i64, _user_ids: Vec<i64>) -> anyhow::Result<()> {
        Ok(())
    }
}

#[async_trait]
impl VipCheckPort for VipAdapter {
    async fn is_viper(&self, uid: i64, user_id: i64) -> anyhow::Result<bool> {
        check::check_state(uid, user_id).await
    }

    async fn is_vip(&self, uid: i64, user_id: i64) -> anyhow::Result<bool> {
        todo!()
    }
}

#[async_trait]
impl VipDelPort for VipAdapter {
    async fn single_soft_del(&self, _uid: i64, _id: i64) -> anyhow::Result<u16> {
        Ok(0)
    }
    async fn batch_soft_del(&self, _uid: i64, _ids: Vec<i64>) -> anyhow::Result<u16> {
        Ok(0)
    }

    async fn single_del(&self, uid: i64, id: i64) -> anyhow::Result<u16> {
        todo!()
    }

    async fn batch_del(&self, uid: i64, ids: Vec<i64>) -> anyhow::Result<u16> {
        todo!()
    }
}

#[async_trait]
impl VipGetPort for VipAdapter {
    async fn get_my_black_ids(
        &self,
        uid: i64,
        _id: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<i64>> {
        get::get_vip_ids(uid, offset, limit).await
    }
    async fn get_he_black_ids(
        &self,
        _uid: i64,
        _id: i64,
        _limit: i64,
        _offset: i64,
    ) -> anyhow::Result<Vec<i64>> {
        Ok(vec![])
    }
    async fn get_black_me_ids(
        &self,
        _uid: i64,
        _id: i64,
        _limit: i64,
        _offset: i64,
    ) -> anyhow::Result<Vec<i64>> {
        Ok(vec![])
    }
    async fn get_black_he_ids(
        &self,
        _uid: i64,
        _id: i64,
        _limit: i64,
        _offset: i64,
    ) -> anyhow::Result<Vec<i64>> {
        Ok(vec![])
    }

    async fn get_my_vip(&self, uid: i64, offset: i64, limit: i64) -> anyhow::Result<Vec<i64>> {
        todo!()
    }
}

#[async_trait]
impl VipListPort for VipAdapter {
    async fn get_config(
        &self,
        _user_id: i64,
    ) -> anyhow::Result<cola_data::cola_user::info::config::UserConfigInfo> {
        Err(anyhow::anyhow!("not implemented"))
    }
    async fn add_black(&self, _uid: i64, _id: i64) -> anyhow::Result<()> {
        Ok(())
    }
    async fn del_black(&self, _uid: i64, _id: i64) -> anyhow::Result<()> {
        Ok(())
    }
    async fn get_following(&self, _uid: i64) -> anyhow::Result<UserInfo> {
        Err(anyhow::anyhow!("not implemented"))
    }
    async fn single_del(&self, _uid: i64, _id: i64) -> anyhow::Result<u16> {
        Ok(0)
    }
    async fn batch_del(&self, _uid: i64, _ids: Vec<i64>) -> anyhow::Result<u16> {
        Ok(0)
    }
    async fn check_state(&self, _uid: i64, _user_id: i64) -> anyhow::Result<bool> {
        Ok(false)
    }
    async fn get_list_by_user_id(
        &self,
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<i64>> {
        get::get_vip_ids(uid, offset, limit).await
    }
    async fn get_here_list(&self, _uid: i64, _user_ids: Vec<i64>) -> anyhow::Result<()> {
        Ok(())
    }
}

#[async_trait]
impl VipManagePort for VipAdapter {
    async fn get_config(
        &self,
        _user_id: i64,
    ) -> anyhow::Result<cola_data::cola_user::info::config::UserConfigInfo> {
        Err(anyhow::anyhow!("not implemented"))
    }
    async fn add_black(&self, _uid: i64, _id: i64) -> anyhow::Result<()> {
        Ok(())
    }
    async fn del_black(&self, _uid: i64, _id: i64) -> anyhow::Result<()> {
        Ok(())
    }
    async fn get_following(&self, _uid: i64) -> anyhow::Result<UserInfo> {
        Err(anyhow::anyhow!("not implemented"))
    }
    async fn single_del(&self, _uid: i64, _id: i64) -> anyhow::Result<u16> {
        Ok(0)
    }
    async fn batch_del(&self, _uid: i64, _ids: Vec<i64>) -> anyhow::Result<u16> {
        Ok(0)
    }
    async fn check_state(&self, _uid: i64, _user_id: i64) -> anyhow::Result<bool> {
        Ok(false)
    }
    async fn get_list_by_user_id(
        &self,
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<i64>> {
        get::get_vip_ids(uid, offset, limit).await
    }
    async fn get_here_list(&self, _uid: i64, _user_ids: Vec<i64>) -> anyhow::Result<()> {
        Ok(())
    }
}

//////// END
