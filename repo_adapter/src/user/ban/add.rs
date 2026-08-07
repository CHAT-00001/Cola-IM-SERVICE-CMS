// repo_adapter/src/user/ban/add.rs
// 🔌 适配器 - 可乐用户 - 封禁 - 发布服务
// 2026/8/7 03:32 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::user::port::ban::add::BanAddPort;

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `用户封禁发布服务`
pub struct BanAddService;

#[async_trait]
impl BanAddPort for BanAddService {
    //

    ////////

    /// # 1. [SERVICE] - 发布
    /// * `desc`: `管理员封禁/改权限`
    /// * `warning`: `风控触发 + 审核人员发布`
    async fn save_banned(&self, uid: i64, id: i64) -> Result<()> {
        todo!()
    }

    ////////

    /// # 2. [SERVICE] - 解封
    /// * `desc`: `解除封禁状态`
    /// * `warning`: `需要 审核人员 + 系统管理员 权限`
    async fn remove_banned(&self, uid: i64, id: i64) -> Result<()> {
        todo!()
    }
}

//////// END