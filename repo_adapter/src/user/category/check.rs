// repo_adapter/src/user/follow/check.rs
// 🔌 插头 - 可乐用户 - 关注 - 模块
// 2026/8/10 04:17 Created.

////////

use async_trait::async_trait;
use port::cola_user::category::check::UserCategoryCheckPort;

////////

/// # [CHECK ADAPTER] - 检查
/// * `DESC`: `🗣 用户 - 关注记录检查适配器`
pub struct UserCategoryCheckAdapter;

#[async_trait]
impl UserCategoryCheckPort for UserCategoryCheckAdapter {
    async fn is_followed(&self, uid: i64, id: i64) -> anyhow::Result<(bool)> {
        todo!()
    }
}

//////// END
