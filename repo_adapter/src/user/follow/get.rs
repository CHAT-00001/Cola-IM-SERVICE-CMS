// repo_adapter/src/user/follow/get.rs -- 适配器 - USER - 关注 - 获取适配器
// 2026/8/10 05:39 Created.

////////

use async_trait::async_trait;
use port::cola_user::follow::get::UserFollowGetPort;

////////

/// # [GET ADAPTER] - 关注适配器
pub struct UserFollowGetAdapter;

#[async_trait]
impl UserFollowGetPort for UserFollowGetAdapter {
    async fn get_he_follow_ids(
        &self,
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }

    async fn get_follow_he_ids(
        &self,
        id: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }
}

//////// END
