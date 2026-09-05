// port/src/user/vip/manage.rs -- 端口 - USER - 贵宾 - 管理端口
// 2026/8/6 00:35 Created.

////////

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use cola_data::cola_user::info::vip::UserVipInfo;

////////

/// # [MANAGE PORTS]
/// * `desc`: `用户贵宾管理端口`
#[async_trait]
pub trait VipManagePort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 管理员列表
    /// * `return` : `vip record infos`
    async fn get_admin_list(
        &self,
        user_id: Option<i64>,              // 用户 ID
        start_time: Option<DateTime<Utc>>, // 开始时间
        end_time: Option<DateTime<Utc>>,   // 结束时间
        offset: i64,                       // 页数
        limit: i64,                        // 数量
    ) -> anyhow::Result<(Vec<UserVipInfo>)>;
}

//////// END
