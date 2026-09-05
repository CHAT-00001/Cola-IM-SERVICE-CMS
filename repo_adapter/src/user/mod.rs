// repo_adapter/src/user/mod.rs -- 适配器 - USER - mod
// 2026/8/10 20:00 Updated.

////////

use crate::user::ban::build_user_ban_port;
use crate::user::black::build_user_black_port;
use crate::user::category::build_user_category_port;
use crate::user::follow::build_user_follow_port;
use crate::user::friend::build_user_friend_port;
use crate::user::profile::build_user_profile_port;
use crate::user::role::build_user_role_port;
use crate::user::share::build_user_share_port;
use crate::user::view::build_user_view_port;
use crate::user::vip::build_user_vip_port;
use port::cola_user::ColaUserPort;

////////

pub mod ban; // 封禁
pub mod black; // 适配器 - 黑名单
pub mod category; // 分类
pub mod follow; // 关注
pub mod friend; // 朋友
pub mod profile; // 用户资料 (主内容)
pub mod role; // 角色
pub mod share; // 分享
pub mod view; // 浏览记录
pub mod vip; // 贵宾

////////

/// # [BUILD] - 构建 USER Port
/// * `desc`: 构建用户 Port 聚合体，包含所有用户相关功能
pub fn build_user_port() -> ColaUserPort {
    ColaUserPort {
        ////////
        ban: build_user_ban_port(),
        black: build_user_black_port(),
        category: build_user_category_port(),
        follow: build_user_follow_port(),
        friend: build_user_friend_port(),
        role: build_user_role_port(),
        share: build_user_share_port(),
        profile: build_user_profile_port(),
        view: build_user_view_port(),
        vip: build_user_vip_port(),
        ////////
    }
}

//////// END
