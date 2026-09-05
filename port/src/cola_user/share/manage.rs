// port/src/user/share/manage.rs -- 端口 - USER - 分享 - 管理端口
// 2026/8/5 21:35 Created.

////////

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use cola_data::cola_user::info::config::UserConfigInfo;
use cola_data::cola_user::info::share::UserShareInfo;
////////

/// # [MANAGE PORTS]
/// * `desc`: `COLA USER - Share Manage Ports`
#[async_trait]
pub trait UserShareManagePort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 获取管理员列表
    async fn get_admin_list(
        &self,
        user_id: Option<i64>,              // 用户 ID
        profile_id: Option<i64>,           // 资料 ID
        start_time: Option<DateTime<Utc>>, // 开始时间
        end_time: Option<DateTime<Utc>>,   // 结束时间
        limit: i64,                        // 数量
        offset: i64,                       // 页码
    ) -> anyhow::Result<(Vec<UserShareInfo>)>;

    ////////
}

//////// END
