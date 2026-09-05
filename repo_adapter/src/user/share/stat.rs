// repo_adapter/src/user/share/stat.rs  -- 适配器 - USER - 分享 - 统计适配器
// 2026/8/10 05:34 Created.

////////

use async_trait::async_trait;
use port::cola_user::share::stat::UserShareStatPort;

////////

/// # [STAT ADAPTER] - 计数
/// * `desc`: `🗣 用户 - 分享记录统计适配器`
pub struct UserShareStatAdapter;

#[async_trait]
impl UserShareStatPort for UserShareStatAdapter {
    async fn stat_count_by_user_id(&self, _user_id: i64) -> anyhow::Result<(u64)> {
        todo!()
    }

    async fn stat_count_by_profile_id(&self, _profile_id: i64) -> anyhow::Result<(u64)> {
        todo!()
    }
}

//////// END
