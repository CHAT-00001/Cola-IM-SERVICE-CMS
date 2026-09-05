// repo_adapter/src/user/view/stat.rs -- 适配器 - USER - 浏览记录 - 统计适配器
// 2026/8/10 05:34 Created.

////////

use async_trait::async_trait;
use port::cola_user::view::stat::UserViewStatPort;
////////

/// # [STAT ADAPTER] - 用户主页浏览记录统计适配器
/// * `desc`: `COLA USER - View Record Stat Adapter`
pub struct UserViewStatAdapter;
#[async_trait]
impl UserViewStatPort for UserViewStatAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 用户总数
    async fn stat_count_by_user_id(&self, user_id: i64) -> anyhow::Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 用户总数
    async fn stat_count_by_profile_id(&self, profile_id: i64) -> anyhow::Result<(u64)> {
        todo!()
    }
}

//////// END
