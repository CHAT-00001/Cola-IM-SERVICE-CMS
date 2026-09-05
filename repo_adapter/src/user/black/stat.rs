// repo_adapter/src/user/black/stat.rs -- 适配器 - USER - 黑名单 - 统计适配器
// 2026/8/10 05:27 Created.

////////

use async_trait::async_trait;
use port::cola_user::black::stat::UserBlackStatPort;

////////

/// # [STAT ADAPTER] - 用户黑名单统计适配器
/// * `desc`: `COLA USER - Black Stat Adapter.`
pub struct UserBlackStatAdapter;
#[async_trait]
impl UserBlackStatPort for UserBlackStatAdapter {
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
