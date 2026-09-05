// repo_adapter/src/user/ban/check.rs -- 适配器 - USER - 封禁 - 检查适配器
// 2026/8/7 05:56 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::ban::check::UserBanCheckPort;
////////

/// # [CHECK ADAPTER] - 用户封禁检查适配器
pub struct BanCheckAdapter;

#[async_trait]
impl UserBanCheckPort for BanCheckAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 检查
    /// * `desc`: `是否被封禁`
    /// * `warning`: `风控触发 + 审核人员发布`
    async fn is_baned(&self, uid: i64, id: i64) -> Result<(bool)> {
        todo!()
    }
}

//////// END
