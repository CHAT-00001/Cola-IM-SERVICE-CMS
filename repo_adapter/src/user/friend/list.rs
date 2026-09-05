// repo_adapter/src/user/friend/list.rs -- 适配器 - USER - 朋友 - 列表适配器
// 2026/8/10 04:16 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::user::UserInfo;
use port::cola_user::friend::list::FriendListPort;

////////

/// # [ADAPTER] - 朋友适配器
pub struct FriendListAdapter;

#[async_trait]
impl FriendListPort for FriendListAdapter {
    async fn get_friends_by_user_id(
        &self,
        uid: i64,
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(Vec<UserInfo>)> {
        todo!()
    }
}

//////// END
