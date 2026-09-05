// port/src/user/friend/music.rs -- 端口 - USER - 朋友 - mod
// 2026/8/5 21:56 Created.

////////

use crate::cola_user::friend::add::FriendAddPort;
use crate::cola_user::friend::check::FriendCheckPort;
use crate::cola_user::friend::del::FriendDelPort;
use crate::cola_user::friend::get::FriendGetPort;
use crate::cola_user::friend::list::FriendListPort;
use crate::cola_user::friend::manage::FriendManagePort;
use crate::cola_user::friend::stat::FriendStatPort;
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

/// # [FRIEND PORTS]
/// * `desc`: `COLA USER - Friend Ports.`
#[derive(Clone)]
pub struct UserFriendPort {
    pub add: Arc<dyn FriendAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn FriendCheckPort + Send + Sync + 'static>,
    pub del: Arc<dyn FriendDelPort + Send + Sync + 'static>,
    pub get: Arc<dyn FriendGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn FriendListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn FriendManagePort + Send + Sync + 'static>,
    pub stat: Arc<dyn FriendStatPort + Send + Sync + 'static>,
}

//////// END
