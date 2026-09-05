// adapter/user/profile/mod.rs -- 适配器 - USER - 用户资料 - mod
// 2026/8/6 04:19 Created.

////////

use port::cola_user::profile::UserProfilePort;
use std::sync::Arc;

////////

pub mod add;
pub mod check;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;
pub mod stat;

////////

/// # [BUILDER] - 构造用户资料端口
/// * `DESC`: `COLA USER - Profile Ports.`
pub fn build_user_profile_port() -> UserProfilePort {
    UserProfilePort {
        add: Arc::new(add::UserAddAdapter),
        check: Arc::new(check::UserCheckAdapter),
        del: Arc::new(del::UserDelAdapter),
        get: Arc::new(get::UserGetAdapter),
        list: Arc::new(list::UserListAdapter),
        manage: Arc::new(manage::UserManageAdapter),
        stat: Arc::new(stat::UserProfileStatAdapter),
    }
}

//////// END
