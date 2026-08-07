// cola_data/src/cola_user/port/mod.rs
// 数据中心 - USER - port -  mod
// 2026/6/10 07:27

////////

use crate::cola_three::port::config::ConfigPort;
use crate::cola_user::port::add::AddPort;
use crate::cola_user::port::ban::BanPort;
use crate::cola_user::port::black::BlackPort;
use crate::cola_user::port::category::CategoryPort;
use crate::cola_user::port::follow::FollowPort;
use crate::cola_user::port::friend::FriendPort;
use crate::cola_user::port::home::HomePort;
use crate::cola_user::port::info::InfoPort;
use crate::cola_user::port::role::RolePort;
use crate::cola_user::port::share::SharePort;
use crate::cola_user::port::user::UserPort;
use crate::cola_user::port::view::ViewPort;
use crate::cola_user::port::vip::VipPort;
use std::sync::Arc;

////////
pub mod add;
pub mod auth;
pub mod ban;
pub mod black;
pub mod category;
pub mod config;
pub mod follow;
pub mod friend;
pub mod home;
pub mod info;
pub mod role;
pub mod share;
pub mod user;
pub mod view;
pub mod vip;

////////

/// # [COLA USER SERVICE PORTS] - 用户
/// * `desc`: `可乐用户服务端口`
#[derive(Clone)]
pub struct ColaUserPort {
    pub add: Arc<dyn AddPort + Send + Sync + 'static>,
    pub ban: BanPort,
    pub black: BlackPort,
    pub cate: Arc<dyn CategoryPort + Send + Sync + 'static>,
    pub category: Arc<dyn CategoryPort + Send + Sync + 'static>,
    pub con: Arc<dyn ConfigPort + Send + Sync + 'static>,
    pub follow: FollowPort,
    pub friend: FriendPort,
    pub home: Arc<dyn HomePort + Send + Sync + 'static>,
    pub info: Arc<dyn InfoPort + Send + Sync + 'static>,
    pub role: RolePort,
    pub share: SharePort,
    pub user: UserPort,
    pub view: ViewPort,
    pub vip: VipPort,
}

//////// END
