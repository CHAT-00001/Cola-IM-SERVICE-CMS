// repo_adapter/src/user/role/mod.rs  -- 🔌 插头 - 可乐用户 - 角色 - 模块
// 2026/8/8 07:37 Created.

////////

mod stat;

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_user::info::black::UserBlackInfo;
use cola_data::cola_user::port::black::add::BlackAddPort;
use cola_data::cola_user::port::black::check::BlackCheckPort;
use cola_data::cola_user::port::black::del::BlackDelPort;
use cola_data::cola_user::port::black::get::BlackGetPort;
use cola_data::cola_user::port::black::list::BlackListPort;
use cola_data::cola_user::port::black::manage::BlackManagePort;

////////

/// # [ROLE ADAPTER] - 角色适配器 (桩实现)
pub struct RoleAdapter;

////////

#[async_trait]
impl BlackAddPort for RoleAdapter {
    async fn add_black(&self, _uid: i64, _target_id: i64) -> Result<()> {
        Ok(())
    }
    async fn del_black(&self, _uid: i64, _id: i64) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
impl BlackCheckPort for RoleAdapter {
    async fn is_blacked(&self, _uid: i64, _id: i64) -> Result<bool> {
        Ok(false)
    }
}

#[async_trait]
impl BlackDelPort for RoleAdapter {
    async fn single_soft_del(&self, _uid: i64, _id: i64) -> Result<u16> {
        Ok(0)
    }
    async fn batch_soft_del(&self, _uid: i64, _ids: Vec<i64>) -> Result<u16> {
        Ok(0)
    }
}

#[async_trait]
impl BlackGetPort for RoleAdapter {
    async fn get_my_black_ids(&self, _uid: i64, _id: i64, _limit: i64, _offset: i64) -> Result<Vec<i64>> {
        Ok(vec![])
    }
    async fn get_he_black_ids(&self, _uid: i64, _id: i64, _limit: i64, _offset: i64) -> Result<Vec<i64>> {
        Ok(vec![])
    }
    async fn get_black_me_ids(&self, _uid: i64, _id: i64, _limit: i64, _offset: i64) -> Result<Vec<i64>> {
        Ok(vec![])
    }
    async fn get_black_he_ids(&self, _uid: i64, _id: i64, _limit: i64, _offset: i64) -> Result<Vec<i64>> {
        Ok(vec![])
    }
}

#[async_trait]
impl BlackListPort for RoleAdapter {
    async fn get_black_list(
        &self,
        _actor_id: Option<i64>,
        _target_id: Option<i64>,
        _start_time: Option<i64>,
        _end_time: Option<i64>,
        _limit: i64,
        _offset: i64,
    ) -> Result<(i64, Vec<UserBlackInfo>)> {
        Ok((0, vec![]))
    }
}

#[async_trait]
impl BlackManagePort for RoleAdapter {
    async fn get_black_list(&self, _uid: i64, _target_id: i64, _limit: i64) -> Result<Vec<UserBlackInfo>> {
        Ok(vec![])
    }
}

//////// END