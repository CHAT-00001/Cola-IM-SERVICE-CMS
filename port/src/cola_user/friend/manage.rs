// user/port/friend/manage.rs -- 端口 - USER - 朋友 - 管理
// 2026/8/5 21:58 Created.

////////

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use cola_data::cola_user::info::config::UserConfigInfo;
use cola_data::cola_user::info::user::UserInfo;
////////

/// # [MANAGE PORTS] - 用户朋友管理端口
#[async_trait]
pub trait FriendManagePort: Send + Sync + 'static {
    //

    ////////

    /// # 7. [PORT] - 管理员列表
    async fn get_admin_list(
        &self,

        user_id: Option<i64>,              // 用户 ID
        to_user_id: Option<i64>,           // 被添加的用户 ID
        start_time: Option<DateTime<Utc>>, // 开始时间
        end_time: Option<DateTime<Utc>>,   // 结束时间
        offset: i64,                       // 页数
        limit: i64,                        // 数量
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END
