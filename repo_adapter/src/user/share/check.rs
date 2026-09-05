// repo_adapter/src/user/share/check.rs  -- 适配器 - USER - 分享 - 检查适配器
// 2026/8/8 12:45 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::share::check::UserShareCheckPort;

////////

pub struct ShareCheckAdapter;

#[async_trait]
impl UserShareCheckPort for ShareCheckAdapter {
    async fn is_blacked(&self, _uid: i64, _id: i64) -> Result<(bool)> {
        todo!()
    }
}

//////// END
