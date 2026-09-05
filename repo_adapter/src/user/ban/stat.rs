// repo_adapter/src/user/ban/stat.rs -- 适配器 - USER - 封禁 - 统计适配器
// 2026/8/10 05:29 Created.

////////

use async_trait::async_trait;
use port::cola_user::ban::stat::UserBanStatPort;

////////

/// # [STAT ADAPTER] - 用户封禁统计适配器
/// * `desc`: `COLA USER - Ban Stat Adapter`
pub struct UserBankStatAdapter;
#[async_trait]
impl UserBanStatPort for UserBankStatAdapter {
    async fn stat_count_by_user_id(&self, user_id: i64) -> anyhow::Result<(u64)> {
        todo!()
    }

    async fn stat_count_by_profile_id(&self, profile_id: i64) -> anyhow::Result<(u64)> {
        todo!()
    }
}

//////// END