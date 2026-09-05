// repo_adapter/src/user/friend/del.rs -- 适配器 - USER - 朋友 - 删除适配器
// 2026/8/10 04:16 Created.

////////

use async_trait::async_trait;
use port::cola_user::friend::del::FriendDelPort;
////////

/// # [DELETE ADAPTER] - 删除
pub struct UserFriendDelAdapter;

#[async_trait]
impl FriendDelPort for UserFriendDelAdapter {
    async fn single_delete(&self, uid: i64, id: i64) -> anyhow::Result<(u16)> {
        todo!()
    }

    async fn batch_delete(&self, uid: i64, ids: Vec<i64>) -> anyhow::Result<(u16)> {
        todo!()
    }
}

//////// END
