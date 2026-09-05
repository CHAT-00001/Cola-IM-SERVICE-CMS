// repo_adapter/src/user/user/stat.rs -- 适配器 - USER - 用户资料 - 统计适配器
// 2026/8/10 05:34 Created.

////////

use async_trait::async_trait;
use port::cola_user::profile::stat::UserProfileStatPort;

////////

/// # [STAT ADAPTER] - 用户资料统计适配器
/// * `desc`: `COLA USER - Profile Stat Adapter.`
pub struct UserProfileStatAdapter;
#[async_trait]
impl UserProfileStatPort for UserProfileStatAdapter {
    //

    ////////

    /// # 1. [ADAPTER] -
    async fn stat_count_by_user_id(&self, user_id: i64) -> anyhow::Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] -
    async fn stat_count_by_profile_id(&self, profile_id: i64) -> anyhow::Result<(u64)> {
        todo!()
    }
}

//////// END
