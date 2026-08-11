// /list.rs
// 
// 2026/8/10 23:27 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::black::UserBlackInfo;
use port::cola_user::black::list::UserBlackListPort;


pub struct UserRoleListAdapter;
#[async_trait]
impl UserBlackListPort for UserRoleListAdapter {
    async fn get_black_list(
        &self,
        _actor_id: Option<i64>,
        _target_id: Option<i64>,
        _start_time: Option<i64>,
        _end_time: Option<i64>,
        _limit: i64,
        _offset: i64,
    ) -> anyhow::Result<(i64, Vec<UserBlackInfo>)> {
        Ok((0, vec![]))
    }
}