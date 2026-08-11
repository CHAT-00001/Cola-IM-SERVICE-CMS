// repo_adapter/src/cola_user/follow/get.rs
// 🔌 插头 - 可乐用户 - 关注 - 模块
// 2026/8/10 05:39 Created.

////////

use async_trait::async_trait;
use port::cola_user::follow::get::UserFollowGetPort;

////////

/// # [GET ADAPTER] - 关注适配器
pub struct UserFollowGetAdapter;

#[async_trait]
impl UserFollowGetPort for UserFollowGetAdapter {
    async fn get_he_follow_ids(&self, user_id: i64, limit: i64, offset: i64) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }

    async fn get_follow_he_ids(&self, id: i64, limit: i64, offset: i64) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }
}

//////// END
