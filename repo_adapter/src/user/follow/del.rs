// repo_adapter/src/user/follow/del.rs
// 🔌 插头 - 可乐用户 - 关注 - 模块
// 2026/8/10 04:16 Created.

////////

use async_trait::async_trait;
use port::cola_user::follow::del::UserFollowDelPort;

////////

/// # [DELETE ADAPTER] - 删除
pub struct UserFollowDelAdapter;

#[async_trait]
impl UserFollowDelPort for UserFollowDelAdapter {
    async fn single_delete(&self, uid: i64, id: i64) -> anyhow::Result<(u16)> {
        todo!()
    }

    async fn batch_delete(&self, uid: i64, ids: Vec<i64>) -> anyhow::Result<(u16)> {
        todo!()
    }
}


//////// END
