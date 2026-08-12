// port/src/cola_user/mod.rs
// ⏩️ 端口 - USER - mod
// 2026/6/10 07:27

////////

use category::UserCategoryPort;

use crate::cola_user::ban::UserBanPort;
use crate::cola_user::black::UserBlackPort;
use crate::cola_user::follow::UserFollowPort;
use crate::cola_user::friend::UserFriendPort;
use crate::cola_user::role::UserRolePort;
use crate::cola_user::share::UserSharePort;
use crate::cola_user::view::UserViewPort;
use crate::cola_user::vip::UserVipPort;
use std::sync::Arc;
use user::UserPort;

////////

pub mod auth; // 验证
pub mod ban; // 封装
pub mod black; // 黑名单
pub mod category; // 分类
pub mod follow; // 关注
pub mod friend; // 朋友
pub mod role; // 角色
pub mod share; // 分享
pub mod user; // 用户(主内容)
pub mod view; // 浏览
pub mod vip; // 贵宾

////////

/// # [COLA USER SERVICE PORTS] - 用户
/// * `desc`: `🗣 用户 - 端口`
#[derive(Clone)]
pub struct ColaUserPort {
    pub ban: UserBanPort,           // 封禁
    pub black: UserBlackPort,       // 黑名单
    pub category: UserCategoryPort, // 分类
    pub follow: UserFollowPort,     // 关注
    pub friend: UserFriendPort,     // 朋友
    pub role: UserRolePort,         // 角色
    pub share: UserSharePort,       // 分享
    pub user: UserPort,             // 用户(主内容)
    pub view: UserViewPort,         // 访客浏览
    pub vip: UserVipPort,           // 贵宾
}

//////// END
