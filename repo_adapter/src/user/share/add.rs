// repo_adapter/src/user/share/add.rs  -- 
// 🔌 插头 - 可乐用户 - 分享 - 添加/移除
// 2026/8/8 12:45 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_user::port::black::add::BlackAddPort;

////////

pub struct ShareAddAdapter;

#[async_trait]
impl BlackAddPort for ShareAddAdapter {
    async fn add_black(&self, _uid: i64, _target_id: i64) -> Result<()> {
        Ok(())
    }
    async fn del_black(&self, _uid: i64, _id: i64) -> Result<()> {
        Ok(())
    }
}

//////// END