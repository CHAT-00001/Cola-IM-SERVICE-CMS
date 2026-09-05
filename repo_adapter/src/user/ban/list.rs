// repo_adapter/src/user/ban/list.rs -- 适配器 - USER - 封禁 - 列表适配器
// 2026/8/7 05:56 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::ban::list::UserBanListPort;

////////

/// # [LIST ADAPTER] - 用户封禁列表适配器
/// * `desc`: `COLA USER - Ban List Adapter`
pub struct BanListAdapter;

#[async_trait]
impl UserBanListPort for BanListAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 发布
    /// * `desc`: `管理员封禁/改权限`
    /// * `warning`: `⚠️ 预设接口`
    async fn get_ban_list(&self, user_id: i64, offset: i64, limit: i64) -> Result<(Vec<i64>)> {
        todo!()
    }
}

//////// END
