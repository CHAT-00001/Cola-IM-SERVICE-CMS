// repo_adapter/src/user/vip/check.rs -- 适配器 - USER - 贵宾 - 检查适配器
// 2026/8/6 14:20 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::vip::check::VipCheckPort;

////////

/// # [CHECK ADAPTER] - 用户贵宾检查适配器
/// * `DESC`: `COLA USER - VIP Add Adapter`
pub struct UserVipCheckAdapter;

#[async_trait]
impl VipCheckPort for UserVipCheckAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 是贵宾
    async fn is_viper(&self, uid: i64, id: i64) -> Result<(bool)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 取消 VIP
    async fn is_vip(&self, uid: i64, user_id: i64) -> Result<bool> {
        todo!()
    }
}

//////// END
