// repo_adapter/src/user/ban/check.rs
// 🔌 适配器 - 可乐用户 - 封禁 - 检查服务
// 2026/8/7 05:56 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::user::port::ban::check::BanCheckPort;

////////

/// # [CHECK SERVICE] - 检查
/// * `desc`: `检查是否被封禁`
pub struct BanCheckService;

#[async_trait]
impl BanCheckPort for BanCheckService {
    //

    ////////

    /// # 1. [SERVICE] - 检查
    /// * `desc`: `是否被封禁`
    /// * `warning`: `风控触发 + 审核人员发布`
    async fn is_baned(&self, uid: i64, id: i64) -> Result<(bool)> {
        todo!()
    }
}

//////// END
