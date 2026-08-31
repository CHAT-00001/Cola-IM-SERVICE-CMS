// repo_adapter/src/user/share/del.rs
// 🔌 插头 - 可乐用户 - 分享 - 删除
// 2026/8/8 12:45 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::share::del::UserShareDelPort;
////////

pub struct ShareDelAdapter;

#[async_trait]
impl UserShareDelPort for ShareDelAdapter {
    async fn single_delete(&self, _uid: i64, _id: i64) -> Result<(u16)> {
        todo!()
    }

    async fn batch_delete(&self, _uid: i64, _ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }
}

//////// END
