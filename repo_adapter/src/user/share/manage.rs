// repo_adapter/src/user/share/manage.rs  --
// 🔌 插头 - 可乐用户 - 分享 - 管理
// 2026/8/8 12:45 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_user::info::config::UserConfigInfo;
use port::cola_user::share::manage::UserShareManagePort;

////////

pub struct ShareManageAdapter;

#[async_trait]
impl UserShareManagePort for ShareManageAdapter {
    async fn get_config(&self, user_id: i64) -> Result<(UserConfigInfo)> {
        todo!()
    }
}

//////// END
