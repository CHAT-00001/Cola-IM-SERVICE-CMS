// port/src/user/follow/add.rs
// ⏩️ 端口 - USER - 关注 - 发布
// 2026/8/5 21:58 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::user::UserInfo;

////////

/// # [ADD PORTS]
/// * `desc`: `用户关注发布端口`
#[async_trait]
pub trait UserFollowAddPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 添加
    /// * `desc`: `用户关注/取关`
    async fn add_follow(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 移除
    async fn del_follow(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<()>;
}

//////// END
