// user/port/follow/mod.rs
// 用户 - port - 关注 - 模块
// 2026/8/5 21:56 Created.

////////

use crate::user::port::follow::add::FollowAddPort;
use crate::user::port::follow::check::FollowCheckPort;
use crate::user::port::follow::del::FollowDelPort;
use crate::user::port::follow::get::FollowGetPort;
use crate::user::port::follow::list::FollowListPort;
use crate::user::port::follow::manage::FollowManagePort;
use std::sync::Arc;
use async_trait::async_trait;
////////

pub mod add;
pub mod check;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;

////////

/// # [FOLLOW PORTS]
/// * `desc`: `用户关注端口`
#[derive(Clone)]
pub struct FollowPort {
    pub add: Arc<dyn FollowAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn FollowCheckPort + Send + Sync + 'static>,
    pub del: Arc<dyn FollowDelPort + Send + Sync + 'static>,
    pub get: Arc<dyn FollowGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn FollowListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn FollowManagePort + Send + Sync + 'static>,
}

//////// END
