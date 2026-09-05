// port/src/user/profile/stat.rs -- 端口 - USER - 用户资料 - 统计端口
// 2026/8/9 20:17 Created.

////////

use async_trait::async_trait;

////////

/// # [STAT PORTS]
/// * `desc`: `COLA USER - Profile Stat Ports`
#[async_trait]
pub trait UserProfileStatPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 用户的
    async fn stat_count_by_user_id(
        &self,
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(u64)>;

    ////////

    /// # 2. [PORT] - 主页的
    async fn stat_count_by_profile_id(
        &self,
        profile_id: i64, // 主页 ID
    ) -> anyhow::Result<(u64)>;
}

//////// END
