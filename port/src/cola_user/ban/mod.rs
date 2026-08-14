// port/src/user/ban/mod.rs
// ⏩️ 端口 - 🗣 可乐用户 - 封禁 - mod
// 2026/8/5 21:33 Created.

////////

use crate::cola_user::ban::add::UserBanAddPort;
use crate::cola_user::ban::check::UserBanCheckPort;
use crate::cola_user::ban::del::UserBanDelPort;
use crate::cola_user::ban::get::UserBanGetPort;
use crate::cola_user::ban::list::UserBanListPort;
use crate::cola_user::ban::manage::UserBanManagePort;
use crate::cola_user::ban::stat::UserBanStatPort;
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

/// # [BAN PORTS]
/// * `desc`: `🗣 用户 - 用户封禁端口`
#[derive(Clone)]
pub struct UserBanPort {
    pub add: Arc<dyn UserBanAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn UserBanCheckPort + Send + Sync + 'static>,
    pub del: Arc<dyn UserBanDelPort + Send + Sync + 'static>,
    pub get: Arc<dyn UserBanGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn UserBanListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn UserBanManagePort + Send + Sync + 'static>,
    pub stat: Arc<dyn UserBanStatPort + Send + Sync + 'static>,
}

//////// END
