// repo_adapter/src/user/vip/music.rs -- 适配器 - USER - 贵宾 - mod
// 2026/8/6 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::user::UserInfo;
use port::cola_user::profile::UserProfilePort;
use port::cola_user::vip::UserVipPort;
use port::cola_user::vip::add::VipAddPort;
use port::cola_user::vip::check::VipCheckPort;
use port::cola_user::vip::del::VipDelPort;
use port::cola_user::vip::get::VipGetPort;
use port::cola_user::vip::list::VipListPort;
use port::cola_user::vip::manage::VipManagePort;
use std::sync::Arc;

////////

pub mod add;
pub mod alive;
pub mod check;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;
pub mod stat;

////////

////////

/// # [BUILDER] - 构造用户贵宾端口
/// * `DESC`: `COLA USER - Vip Ports.`
pub fn build_user_vip_port() -> UserVipPort {
    UserVipPort {
        add: Arc::new(add::UserVipAddAdapter),
        check: Arc::new(check::UserVipCheckAdapter),
        del: Arc::new(del::UserVipDelAdapter),
        get: Arc::new(get::UserVipGetAdapter),
        list: Arc::new(list::UserVipListAdapter),
        manage: Arc::new(manage::UserVipManageAdapter),
        stat: Arc::new(stat::UserVipStatAdapter),
    }
}

//////// END
