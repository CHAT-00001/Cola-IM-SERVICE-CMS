// repo_adapter/src/user/share/add.rs  --
// 🔌 插头 - 可乐用户 - 分享 - 添加/移除
// 2026/8/8 12:45 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::share::add::UserShareAddPort;
////////

pub struct UserShareAddAdapter;

#[async_trait]
impl UserShareAddPort for UserShareAddAdapter {
    async fn add_share(&self, uid: i64, id: i64) -> Result<()> {
        todo!()
    }

    async fn del_share(&self, uid: i64, id: i64) -> Result<()> {
        todo!()
    }
}

//////// END
