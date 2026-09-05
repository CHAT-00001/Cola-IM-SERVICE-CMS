// port/src/user/friend/list.rs -- 端口 - USER - 朋友 - 列表端口
// 2026/8/5 21:58 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::user::UserInfo;

////////

/// # [LIST PORT] - 用户朋友列表端口
#[async_trait]
pub trait FriendListPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 用户的朋友
    async fn get_friends_by_user_id(
        &self,
        uid: i64,     // 操作者 ID
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(Vec<UserInfo>)>;

    ////////
}

//////// END
