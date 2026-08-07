// user/port/friend/mod.rs
// 用户 - port - 朋友 - 模块
// 2026/8/5 21:56 Created.

////////

use crate::user::port::friend::add::FriendAddPort;
use crate::user::port::friend::check::FriendCheckPort;
use crate::user::port::friend::del::FriendDelPort;
use crate::user::port::friend::get::FriendGetPort;
use crate::user::port::friend::list::FriendListPort;
use crate::user::port::friend::manage::FriendManagePort;
use std::sync::Arc;

////////

pub mod add;
pub mod check;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;

////////

/// # [FRIEND PORTS]
/// * `desc`: `用户朋友端口`
#[derive(Clone)]
pub struct FriendPort {
    pub add: Arc<dyn FriendAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn FriendCheckPort + Send + Sync + 'static>,
    pub del: Arc<dyn FriendDelPort + Send + Sync + 'static>,
    pub get: Arc<dyn FriendGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn FriendListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn FriendManagePort + Send + Sync + 'static>,
}

//////// END