// port/src/user/user/mod.rs
// ⏩️ 端口 - 🗣 可乐用户 - 用户 - 模块
// 2026/8/5 22:02 Created.

////////

use std::sync::Arc;
use crate::cola_user::user::add::UserAddPort;
use crate::cola_user::user::check::UserCheckPort;
use crate::cola_user::user::del::UserDelPort;
use crate::cola_user::user::get::UserGetPort;
use crate::cola_user::user::list::UserListPort;
use crate::cola_user::user::manage::UserManagePort;

////////
pub mod add;
pub mod check;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;
mod stat;
////////

/// # [USER PORTS]
/// * `desc`: `可乐用户 - 用户端口`
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