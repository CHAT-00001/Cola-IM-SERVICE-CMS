// user/port/friend/get.rs
// ⏩️ 端口 - 🗣 用户 - 朋友 - 获取
// 2026/8/5 21:56 Created.

////////

use async_trait::async_trait;

////////

/// # [ADD PORTS]
/// * `desc`: `用户朋友发布端口`
#[async_trait]
pub trait FriendGetPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 我的
    /// * `desc`: `获取我朋友的用户IDs`
    async fn get_my_friend_ids(
        &self,
        uid: i64,    // UID
        id: i64,     // 目标用户ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 2. [PORT] - 她的
    /// * `desc`: `获取TA朋友的用户IDs`
    async fn get_he_friend_ids(
        &self,
        uid: i64,    // UID
        id: i64,     // 目标用户ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 3. [PORT] - 朋友我的
    /// * `desc`: `获取朋友我的用户IDs`
    async fn get_friend_me_ids(
        &self,
        uid: i64,    // UID
        id: i64,     // 目标用户ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 4. [PORT] - 朋友她的
    /// * `desc`: `获取朋友TA的用户IDs`
    async fn get_friend_he_ids(
        &self,
        uid: i64,    // UID
        id: i64,     // 目录用户ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END
