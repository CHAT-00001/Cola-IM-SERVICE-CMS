// repo_adapter/src/user/share/get.rs  -- 
// 🔌 插头 - 可乐用户 - 分享 - 获取
// 2026/8/8 12:45 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_user::info::black::UserBlackInfo;
use cola_data::cola_user::port::black::get::BlackGetPort;

////////

pub struct ShareGetAdapter;

#[async_trait]
impl BlackGetPort for ShareGetAdapter {
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

//////// END