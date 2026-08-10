// repo_adapter/src/user/share/list.rs  -- 
// 🔌 插头 - 可乐用户 - 分享 - 列表
// 2026/8/8 12:45 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_user::info::black::UserBlackInfo;
use cola_data::cola_user::port::black::list::BlackListPort;

////////

pub struct ShareListAdapter;

#[async_trait]
impl BlackListPort for ShareListAdapter {
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

//////// END