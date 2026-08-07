// user/port/role/mod.rs
// 用户 - port - 角色 - 模块
// 2026/8/5 21:33 Created.

////////

use crate::user::port::add::AddPort;
use crate::user::port::black::add::BlackAddPort;
use crate::user::port::black::check::BlackCheckPort;
use crate::user::port::black::del::BlackDelPort;
use crate::user::port::black::get::BlackGetPort;
use crate::user::port::black::list::BlackListPort;
use crate::user::port::black::manage::BlackManagePort;
use std::sync::Arc;

////////
pub mod add;
pub mod check;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;

////////

/// # [ROLE PORTS]
/// * `desc`: `用户角色端口`
#[derive(Clone)]
pub struct RolePort {
    pub add: Arc<dyn BlackAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn BlackCheckPort + Send + Sync + 'static>,
    pub del: Arc<dyn BlackDelPort + Send + Sync + 'static>,
    pub get: Arc<dyn BlackGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn BlackListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn BlackManagePort + Send + Sync + 'static>,
}

//////// END
