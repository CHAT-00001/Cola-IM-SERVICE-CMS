// port/src/user/profile/mod.rs -- 端口 - USER - 用户资料 - mod
// 2026/8/5 22:02 Created.

////////

use crate::cola_user::profile::add::UserAddPort;
use crate::cola_user::profile::check::UserCheckPort;
use crate::cola_user::profile::del::UserDelPort;
use crate::cola_user::profile::get::UserGetPort;
use crate::cola_user::profile::list::UserListPort;
use crate::cola_user::profile::manage::UserManagePort;
use crate::cola_user::profile::stat::UserProfileStatPort;
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

/// # [USER PROFILE PORTS]
/// * `desc`: `COLA USER - Profile Ports.`
#[derive(Clone)]
pub struct UserProfilePort {
    pub add: Arc<dyn UserAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn UserCheckPort + Send + Sync + 'static>,
    pub del: Arc<dyn UserDelPort + Send + Sync + 'static>,
    pub get: Arc<dyn UserGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn UserListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn UserManagePort + Send + Sync + 'static>,
    pub stat: Arc<dyn UserProfileStatPort + Send + Sync + 'static>,
}

//////// END
