// repo_adapter/src/user/category/stat.rs -- 适配器 - USER - 分类 - 统计适配器
// 2026/8/10 04:17 Created.

////////

use async_trait::async_trait;
use port::cola_user::category::stat::UserCategoryStatPort;
use port::cola_user::follow::stat::UserFollowStatPort;

////////

/// # [STAT ADAPTER] - 用户分类统计适配器
/// * `desc`: `COLA USER - Categories Adapter.`
pub struct UserCategoryStatAdapter;
#[async_trait]
impl UserCategoryStatPort for UserCategoryStatAdapter {
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
