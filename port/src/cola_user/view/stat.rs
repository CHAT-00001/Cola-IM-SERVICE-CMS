// port/src/user/view/stat.rs
// ⏩️ 端口 - 🗣 用户 - 浏览 - 统计
// 2026/8/9 20:17 Created.

////////

use async_trait::async_trait;

////////

/// # [STAT PORTS]
/// * `desc`: `🗣 用户 - 用户浏览记录统计端口`
#[async_trait]
pub trait UserViewStatPort: Send + Sync + 'static {
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

    ////////
}
