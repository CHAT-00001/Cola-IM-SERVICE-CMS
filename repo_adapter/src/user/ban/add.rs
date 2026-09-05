// repo_adapter/src/user/ban/add.rs -- 适配器 - USER - 封禁 - 发布适配器
// 2026/8/7 03:32 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::ban::add::UserBanAddPort;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `用户封禁发布服务`
pub struct BanAddAdapter;

#[async_trait]
impl UserBanAddPort for BanAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 发布
    /// * `desc`: `管理员封禁/改权限`
    /// * `warning`: `风控触发 + 审核人员发布`
    async fn save_banned(&self, uid: i64, id: i64) -> Result<()> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 解封
    /// * `desc`: `解除封禁状态`
    /// * `warning`: `需要 审核人员 + 系统管理员 权限`
    async fn remove_banned(&self, uid: i64, id: i64) -> Result<()> {
        todo!()
    }
}

//////// END
