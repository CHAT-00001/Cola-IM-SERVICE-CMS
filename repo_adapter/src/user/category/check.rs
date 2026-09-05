// repo_adapter/src/user/category/check.rs -- 适配器 - USER - 分类 - 检查适配器
// 2026/8/10 04:17 Created.

////////

use async_trait::async_trait;
use port::cola_user::category::check::UserCategoryCheckPort;

////////

/// # [CHECK ADAPTER] - 用户分类检查适配器
/// * `DESC`: `COLA USER - Categories Check Adapter`
pub struct UserCategoryCheckAdapter;

#[async_trait]
impl UserCategoryCheckPort for UserCategoryCheckAdapter {
    async fn is_followed(&self, uid: i64, id: i64) -> anyhow::Result<(bool)> {
        todo!()
    }
}

//////// END
