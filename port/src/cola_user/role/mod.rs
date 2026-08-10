// port/src/cola_user/role/mod.rs
// ⏩️ 端口 - 🗣 用户 - 角色 - 模块
// 2026/8/5 21:33 Created.

////////

use crate::cola_user::role::add::UserRoleAddPort;
use crate::cola_user::role::check::UserRoleCheckPort;
use crate::cola_user::role::del::UserRoleDelPort;
use crate::cola_user::role::get::UserRoleGetPort;
use crate::cola_user::role::list::UserRoleListPort;
use crate::cola_user::role::manage::UserRoleManagePort;
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

/// # [ROLE PORTS]
/// * `desc`: `用户角色端口`
#[derive(Clone)]
pub struct UserRolePort {
    pub add: Arc<dyn UserRoleAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn UserRoleCheckPort + Send + Sync + 'static>,
    pub del: Arc<dyn UserRoleDelPort + Send + Sync + 'static>,
    pub get: Arc<dyn UserRoleGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn UserRoleListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn UserRoleManagePort + Send + Sync + 'static>,
}

//////// END
