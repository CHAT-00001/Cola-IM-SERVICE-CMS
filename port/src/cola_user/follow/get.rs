// cola_user/port/follow/get.rs
// ⏩️ 端口 - 🗣 用户 - 关注 - 获取
// 2026/8/5 21:56 Created.

////////

use async_trait::async_trait;

////////

/// # [GET PORTS]
/// * `desc`: `用户关注获取端口`
#[async_trait]
pub trait UserFollowGetPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 用户主动关注的
    /// * `desc`: `获取我关注的用户IDs`
    async fn get_he_follow_ids(
        &self,
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 2. [PORT] - 用户被动关注的
    /// * `desc`: `获取TA关注的用户IDs`
    async fn get_follow_he_ids(
        &self,
        id: i64,     // 用户 ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END
