// repo_adapter/src/user/role/mod.rs -- 适配器 - USER - 角色 - mod
// 2026/8/8 07:37 Created.

////////

use port::cola_user::role::UserRolePort;
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

/// # [BUILDER] - 构造用户角色端口
/// * `DESC`: `COLA USER - Role Ports.`
pub fn build_user_role_port() -> UserRolePort {
    UserRolePort {
        add: Arc::new(add::RoleAddAdapter),
        check: Arc::new(check::UserRoleCheckAdapter),
        del: Arc::new(del::UserRoleDelAdapter),
        get: Arc::new(get::UserRoleGetAdapter),
        list: Arc::new(list::UserRoleListAdapter),
        manage: Arc::new(manage::UserRoleManageAdapter),
        stat: Arc::new(stat::UserRoleStatAdapter),
    }
}

//////// END
