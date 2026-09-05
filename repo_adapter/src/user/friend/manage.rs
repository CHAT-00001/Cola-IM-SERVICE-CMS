// repo_adapter/src/user/follow/manage.rs -- 适配器 - USER - 关注 - 管理适配器
// 2026/8/10 04:16 Created.

////////

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use port::cola_user::friend::manage::FriendManagePort;

////////

/// # [MANAGE ADAPTER] - 用户朋友管理适配器
pub struct FriendManageAdapter;

#[async_trait]
impl FriendManagePort for FriendManageAdapter {
    //

    ////////

    /// # [ADAPTER] - 管理员列表
    async fn get_admin_list(
        &self,
        user_id: Option<i64>,
        to_user_id: Option<i64>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }
}

//////// END
