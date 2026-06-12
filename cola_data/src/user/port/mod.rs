// port/mod.rs  -- 可乐用户中心 - 服务端口 mod
// 2026/6/10 07:27

////////

pub mod add;
pub mod blacklist;
pub mod config;
pub mod following;
pub mod friend;
pub mod info;
pub mod view;

////////

use std::sync::Arc;
use crate::user::port::add::AddPort;
use crate::user::port::config::ConfigPort;
use crate::user::port::info::InfoPort;
use crate::user::port::view::ViewPort;
use crate::user::port::blacklist::BlacklistPort;
use crate::user::port::following::FollowingPort;
use crate::user::port::friend::FriendPort;

////////

#[derive(Clone)]
pub struct ColaUserPort {
    pub add: Arc<dyn AddPort + Send + Sync + 'static>,
    pub black: Arc<dyn BlacklistPort + Send + Sync + 'static>,
    pub con: Arc<dyn ConfigPort + Send + Sync + 'static>,
    pub following: Arc<dyn FollowingPort + Send + Sync + 'static>,
    pub friend: Arc<dyn FriendPort + Send + Sync + 'static>,
    pub info: Arc<dyn InfoPort + Send + Sync + 'static>,
    pub view: Arc<dyn ViewPort + Send + Sync + 'static>,
    // pub check_port: Arc<dyn UserCheckPermissionPort + Send + Sync + 'static>,
}
