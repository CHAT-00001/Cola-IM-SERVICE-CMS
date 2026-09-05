// user/port/friend/get.rs -- 端口 - USER - 朋友 - 获取端口
// 2026/8/5 21:56 Created.

////////

use async_trait::async_trait;

////////

/// # [GET PORTS]
/// * `desc`: `用户朋友发布端口`
#[async_trait]
pub trait FriendGetPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 用户的
    async fn get_friend_ids_by_user_id(
        &self,
        uid: i64,    // UID
        id: i64,     // 目标用户ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END
