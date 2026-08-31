// repo_adapter/src/user/ban/list.rs
// 🔌 适配器 - 可乐用户 - 封禁 - 列表服务
// 2026/8/7 05:56 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::ban::list::UserBanListPort;

////////

/// # [LIST SERVICE] - 列表
/// * `desc`: `用户封禁列表服务`
pub struct BanListService;

#[async_trait]
impl UserBanListPort for BanListService {
    //

    ////////

    /// # 1. [SERVICE] - 发布
    /// * `desc`: `管理员封禁/改权限`
    /// * `warning`: `⚠️ 预设接口`
    async fn get_ban_list(&self, user_id: i64, offset: i64, limit: i64) -> Result<(Vec<i64>)> {
        todo!()
    }
}

//////// END
