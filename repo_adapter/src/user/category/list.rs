// repo_adapter/src/cola_user/follow/list.rs
// 🔌 插头 - 可乐用户 - 关注 - 模块
// 2026/8/10 04:16 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::follow::UserFollowInfo;
use port::cola_user::follow::list::UserFollowListPort;

////////

/// # [ADAPTER] - 关注适配器
pub struct UserFollowListAdapter;

#[async_trait]
impl UserFollowListPort for UserFollowListAdapter {
    async fn get_follow_infos_by_user_id(
        &self,
        uid: i64,
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<UserFollowInfo>)> {
        todo!()
    }

    async fn get_follow_infos_by_target_id(
        &self,
        uid: i64,
        target_id: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<UserFollowInfo>)> {
        todo!()
    }
}

//////// END
