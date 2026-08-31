// /stat.rs
//
// 2026/8/10 05:34 Created.

////////

////////

use async_trait::async_trait;
use port::cola_user::black::stat::UserBlackStatPort;

////////

/// # [STAT ADAPTER] - 计数
/// * `desc`: `🗣 用户 - 关注记录统计适配器`
pub struct UserBlackStatAdapter;
#[async_trait]
impl UserBlackStatPort for UserBlackStatAdapter {
    async fn stat_count_by_user_id(&self, user_id: i64) -> anyhow::Result<(u64)> {
        todo!()
    }

    async fn stat_count_by_profile_id(&self, profile_id: i64) -> anyhow::Result<(u64)> {
        todo!()
    }
}
