// repo_adapter/src/user/share/check.rs  -- 
// 🔌 插头 - 可乐用户 - 分享 - 检查
// 2026/8/8 12:45 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_user::port::black::check::BlackCheckPort;

////////

pub struct ShareCheckAdapter;

#[async_trait]
impl BlackCheckPort for ShareCheckAdapter {
    async fn is_blacked(&self, _uid: i64, _id: i64) -> Result<bool> {
        Ok(false)
    }
}

//////// END