// port/src/cola_user/follow/add.rs
// ⏩️ 端口 - 🗣 用户 - 关注 - 发布
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
    async fn set_follow(
        &self,
        uid: i64,        // UID
        user_id: i64,    // 用户 ID
        is_follow: bool, // 是否关注
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
