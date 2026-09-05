// repo_adapter/src/user/friend/get.rs -- 适配器 - USER - 朋友 - 获取适配器
// 2026/8/10 05:39 Created.

////////

use async_trait::async_trait;
use port::cola_user::friend::get::FriendGetPort;

////////

/// # [GET ADAPTER] - 用户朋友获取适配器
pub struct UserFriendGetAdapter;

#[async_trait]
impl FriendGetPort for UserFriendGetAdapter {
    async fn get_friend_ids_by_user_id(
        &self,
        uid: i64,
        id: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }
}

//////// END
