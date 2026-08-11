// repo_adapter/src/cola_user/follow/check.rs
// 🔌 插头 - 可乐用户 - 关注 - 模块
// 2026/8/10 04:17 Created.

////////

use async_trait::async_trait;
use port::cola_user::follow::check::UserFollowCheckPort;

////////

/// # [CHECK ADAPTER] - 检查
/// * `DESC`: `🗣 用户 - 关注记录检查适配器`
pub struct UserFollowCheckAdapter;

#[async_trait]
impl UserFollowCheckPort for UserFollowCheckAdapter {
    //

    ////////

    /// 1. # [ADAPTER] - 是否关注
    async fn is_followed(&self, uid: i64, user_id: i64) -> anyhow::Result<(bool)> {

        let is_followed = true;
        Ok(is_followed)
    }

    ////////

    /// 2. # [ADAPTER] - 检查状态
    async fn check_state(&self, uid: i64, user_id: i64) -> anyhow::Result<(bool)> {
        todo!()
    }
}

//////// END
