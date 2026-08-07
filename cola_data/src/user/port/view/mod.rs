// user/port/view/mod.rs
// 用户 - port - 浏览 - 模块
// 2026/6/10 07:36

////////
use crate::user::port::view::add::ViewAddPort;
use crate::user::port::view::check::ViewCheckPort;
use crate::user::port::view::del::ViewDelPort;
use crate::user::port::view::get::ViewGetPort;
use crate::user::port::view::list::ViewListPort;
use crate::user::port::view::manage::ViewManagePort;
use std::sync::Arc;

////////

pub mod add;
pub mod check;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;

////////

/// # [VIEW PORTS]
/// * `desc`: `用户浏览端口`
#[derive(Clone)]
pub struct ViewPort {
    pub add: Arc<dyn ViewAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn ViewCheckPort + Send + Sync + 'static>,
    pub del: Arc<dyn ViewDelPort + Send + Sync + 'static>,
    pub get: Arc<dyn ViewGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn ViewListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn ViewManagePort + Send + Sync + 'static>,
}

//////// END
