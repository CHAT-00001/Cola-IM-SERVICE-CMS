// repo_adapter/src/user/share/manage.rs  -- 
// 🔌 插头 - 可乐用户 - 分享 - 管理
// 2026/8/8 12:45 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_user::info::black::UserBlackInfo;
use cola_data::cola_user::port::black::manage::BlackManagePort;

////////

pub struct ShareManageAdapter;

#[async_trait]
impl BlackManagePort for ShareManageAdapter {
    async fn get_black_list(&self, _uid: i64, _target_id: i64, _limit: i64) -> Result<Vec<UserBlackInfo>> {
        Ok(vec![])
    }
}

//////// END