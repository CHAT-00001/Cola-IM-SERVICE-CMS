// cola_user/port/ban/mod.rs
// 用户 - port - 封禁 - 模块
// 2026/8/5 21:33 Created.

////////

use crate::cola_user::port::add::AddPort;
use crate::cola_user::port::ban::add::BanAddPort;
use crate::cola_user::port::ban::check::BanCheckPort;
use crate::cola_user::port::ban::del::BanDelPort;
use crate::cola_user::port::ban::get::BanGetPort;
use crate::cola_user::port::ban::list::BanListPort;
use crate::cola_user::port::ban::manage::BanManagePort;
use std::sync::Arc;

////////

pub mod add;
pub mod check;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;

////////

/// # [BAN PORTS]
/// * `desc`: `封禁端口`
#[derive(Clone)]
pub struct BanPort {
    pub add: Arc<dyn BanAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn BanCheckPort + Send + Sync + 'static>,
    pub del: Arc<dyn BanDelPort + Send + Sync + 'static>,
    pub get: Arc<dyn BanGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn BanListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn BanManagePort + Send + Sync + 'static>,
}

//////// END
