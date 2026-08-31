// port/src/user/black/mod.rs
// ⏩️ 端口 - 🗣 可乐用户 - 黑名单 - music
// 2026/8/5 21:33 Created.

////////

use crate::cola_user::black::add::UserBlackAddPort;
use crate::cola_user::black::check::UserBlackCheckPort;
use crate::cola_user::black::del::UserBlackDelPort;
use crate::cola_user::black::get::UserBlackGetPort;
use crate::cola_user::black::list::UserBlackListPort;
use crate::cola_user::black::manage::UserBlackManagePort;
use crate::cola_user::black::stat::UserBlackStatPort;
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

/// # [BLACK PORTS]
/// * `desc`: `🗣 用户 - 黑名单端口`
#[derive(Clone)]
pub struct UserBlackPort {
    pub add: Arc<dyn UserBlackAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn UserBlackCheckPort + Send + Sync + 'static>,
    pub del: Arc<dyn UserBlackDelPort + Send + Sync + 'static>,
    pub get: Arc<dyn UserBlackGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn UserBlackListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn UserBlackManagePort + Send + Sync + 'static>,
    pub stat: Arc<dyn UserBlackStatPort + Send + Sync + 'static>,
}

//////// END
