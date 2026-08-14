// user/followt/list.rs
// ⏩️ 端口 - 🗣 用户 - 关注 - 列表
// 2026/8/5 21:58 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::config::UserConfigInfo;
use cola_data::cola_user::info::follow::UserFollowInfo;
use cola_data::cola_user::info::user::UserInfo;
////////

/// # [LIST PORT]
/// * `desc`: `用户关注列表端口`
#[async_trait]
pub trait UserFollowListPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 自己关注的
    /// * `return` : `user ids`
    async fn get_follow_infos_by_user_id(
        &self,
        uid: i64,     // 操作者
        user_id: i64, // 用户 ID
        offset: i64,  // 页数
        limit: i64,   // 数量
    ) -> anyhow::Result<(Vec<UserFollowInfo>)>;

    ////////

    /// # 2. [PORT] - 被关注的
    /// * `return` : `user ids`
    async fn get_follow_infos_by_target_id(
        &self,
        uid: i64,       // 操作者
        target_id: i64, // 目标 ID
        offset: i64,    // 页数
        limit: i64,     // 数量
    ) -> anyhow::Result<(Vec<UserFollowInfo>)>;
}

//////// END
