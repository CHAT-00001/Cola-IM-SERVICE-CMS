// repo_adapter/src/user/share/del.rs  -- 
// 🔌 插头 - 可乐用户 - 分享 - 删除
// 2026/8/8 12:45 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_user::port::black::del::BlackDelPort;

////////

pub struct ShareDelAdapter;

#[async_trait]
impl BlackDelPort for ShareDelAdapter {
    async fn single_soft_del(&self, _uid: i64, _id: i64) -> Result<u16> {
        Ok(0)
    }
    async fn batch_soft_del(&self, _uid: i64, _ids: Vec<i64>) -> Result<u16> {
        Ok(0)
    }
}

//////// END