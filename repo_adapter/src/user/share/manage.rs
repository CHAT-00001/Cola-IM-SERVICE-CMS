// repo_adapter/src/user/share/manage.rs  -- 适配器 - USER - 分享 - 管理适配器
// 2026/8/8 12:45 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use cola_data::cola_user::info::config::UserConfigInfo;
use cola_data::cola_user::info::share::UserShareInfo;
use port::cola_user::share::manage::UserShareManagePort;

////////

/// # [MANAGE ADAPTER] - 用户资料分享管理适配器
/// * `DESC`: `COLA USER - Share Manage Adapter`
pub struct ShareManageAdapter;

#[async_trait]
impl UserShareManagePort for ShareManageAdapter {
    //

    ////////

    /// # [ADAPTER] - 管理员列表
    async fn get_admin_list(
        &self,
        user_id: Option<i64>,
        profile_id: Option<i64>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<UserShareInfo>)> {
        todo!()
    }
}

//////// END
