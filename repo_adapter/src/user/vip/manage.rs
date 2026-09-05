// repo_adapter/src/user/vip/manage.rs -- 适配器 - USER - 贵宾 - 管理适配器
// 2026/8/6 12:40 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use cola_data::cola_user::command::vip::VipCommand;
use cola_data::cola_user::info::vip::UserVipInfo;
use port::cola_user::view::add::UserViewAddPort;
use port::cola_user::vip::UserVipPort;
use port::cola_user::vip::add::VipAddPort;
use port::cola_user::vip::manage::VipManagePort;
use service::cola_user::vip::add::VipAddService;

////////

/// # [ADD ADAPTER] - 用户贵宾发布适配器
/// * `DESC`: `COLA USER - VIP Add Adapter`
pub struct UserVipManageAdapter;

#[async_trait]
impl VipManagePort for UserVipManageAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 管理员列表
    async fn get_admin_list(
        &self,
        user_id: Option<i64>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<UserVipInfo>)> {
        todo!()
    }
}

//////// END
