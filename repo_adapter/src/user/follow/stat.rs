// repo_adapter/src/user/follow/stat.rs -- 适配器 - USER - 关注 - 统计适配器
// 2026/8/10 04:17 Created.

////////

use async_trait::async_trait;
use port::cola_user::follow::stat::UserFollowStatPort;

////////

/// # [STAT ADAPTER] - 用户关注统计适配器
/// * `desc`: `COLA USER - Follow Stat Adapter.`
pub struct UserFollowStatAdapter;
#[async_trait]
impl UserFollowStatPort for UserFollowStatAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 用户关注了多少人
    async fn stat_count_by_user_id(&self, user_id: i64) -> anyhow::Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 我被多少人关注了
    async fn stat_count_by_profile_id(&self, profile_id: i64) -> anyhow::Result<(u64)> {
        todo!()
    }
}

//////// END
