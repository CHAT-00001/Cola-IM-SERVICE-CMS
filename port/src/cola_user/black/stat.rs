// port/src/cola_user/black/stat.rs
// ⏩️ 端口 - 🗣 用户 - 黑名单 - 统计
// 2026/8/10 04:30 Created.

////////


use async_trait::async_trait;

////////

/// # [STAT PORTS]
/// * `desc`: `🗣 用户 - 用户黑名单记录统计端口`
#[async_trait]
pub trait UserBlackStatPort: Send + Sync + 'static {
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