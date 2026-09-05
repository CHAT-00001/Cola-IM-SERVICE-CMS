// repo_adapter/src/user/follow/stat.rs -- 适配器 - USER - 朋友 - 统计适配器
// 2026/8/10 04:17 Created.

////////

use async_trait::async_trait;
use port::cola_user::friend::stat::FriendStatPort;

////////

/// # [STAT ADAPTER] - 用户朋友统计适配器
/// * `desc`: `COLA USER - Follow Stat Adapter.`
pub struct FriendStatAdapter;
#[async_trait]
impl FriendStatPort for FriendStatAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 用户有多少个朋友
    async fn stat_count_by_user_id(&self, user_id: i64) -> anyhow::Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 是多少人的朋友
    async fn stat_count_by_profile_id(&self, profile_id: i64) -> anyhow::Result<(u64)> {
        todo!()
    }
}

//////// END
