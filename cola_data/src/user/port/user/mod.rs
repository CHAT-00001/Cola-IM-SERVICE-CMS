// user/port/user/mod.rs
// 用户 - port - 用户 - 模块
// 2026/8/5 22:02 Created.

////////

use crate::user::port::user::add::UserAddPort;
use crate::user::port::user::check::UserCheckPort;
use crate::user::port::user::del::UserDelPort;
use crate::user::port::user::get::UserGetPort;
use crate::user::port::user::list::UserListPort;
use crate::user::port::user::manage::UserManagePort;
use std::sync::Arc;

////////
pub mod add;
pub mod check;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;

////////

/// # [USER PORTS]
/// * `desc`: `黑名单端口`
#[derive(Clone)]
pub struct UserPort {
    pub add: Arc<dyn UserAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn UserCheckPort + Send + Sync + 'static>,
    pub del: Arc<dyn UserDelPort + Send + Sync + 'static>,
    pub get: Arc<dyn UserGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn UserListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn UserManagePort + Send + Sync + 'static>,
}

//////// END