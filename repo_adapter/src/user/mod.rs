// repo_adapter/src/user/mod.rs -- 适配器 - USER - mod
// 2026/8/10 20:00 Updated.

////////

use port::cola_user::ColaUserPort;
use port::cola_user::ban::UserBanPort;
use port::cola_user::black::UserBlackPort;
use port::cola_user::category::UserCategoryPort;
use port::cola_user::follow::UserFollowPort;
use port::cola_user::friend::UserFriendPort;
use port::cola_user::role::UserRolePort;
use port::cola_user::share::UserSharePort;
use port::cola_user::user::UserPort;
use port::cola_user::view::UserViewPort;
use port::cola_user::vip::UserVipPort;
use std::sync::Arc;

////////

pub mod ban; // 封禁
pub mod black; // 适配器 - 黑名单
pub mod category; // 分类
pub mod follow; // 关注
pub mod friend; // 朋友 (备用)
pub mod profile; // 资料
pub mod role; // 角色
pub mod share; // 分享
pub mod user; // 用户 (主内容)
pub mod view; // 浏览记录
pub mod vip; // 贵宾

////////

/// # [BUILD] - 构建 USER Port
/// * `desc`: 构建用户 Port 聚合体，包含所有用户相关功能
pub fn build_user_port() -> ColaUserPort {
    ColaUserPort {
        // BAN - 封禁
        ban: UserBanPort {
            add: Arc::new(ban::add::BanAddService),
            check: Arc::new(ban::check::BanCheckService),
            del: Arc::new(ban::del::BanDelService),
            get: Arc::new(ban::get::BanGetService),
            list: Arc::new(ban::list::BanListService),
            manage: Arc::new(ban::manage::BanManageService),
            stat: Arc::new(ban::stat::UserBankStatAdapter),
        },
        // BLACK - 黑名单
        black: UserBlackPort {
            add: Arc::new(black::add::BlackAddAdapter),
            check: Arc::new(black::check::BlackCheckAdapter),
            del: Arc::new(black::del::BlackDelAdapter),
            get: Arc::new(black::get::BlackGetAdapter),
            list: Arc::new(black::list::BlackListAdapter),
            manage: Arc::new(black::manage::BlackManageAdapter),
            stat: Arc::new(black::stat::UserBlackStatAdapter),
        },
        // CATEGORY - 分类
        category: UserCategoryPort {
            add: Arc::new(category::add::CategoryAddAdapter),
            check: Arc::new(category::check::UserCategoryCheckAdapter),
            delete: Arc::new(category::del::CategoryDeleteAdapter),
            get: Arc::new(category::get::CategoryGetAdapter),
            list: Arc::new(category::list::CategoryListAdapter),
            manage: Arc::new(category::manage::CategoryManageAdapter),
        },
        // FOLLOW - 关注
        follow: UserFollowPort {
            add: Arc::new(follow::add::FollowAddAdapter),
            check: Arc::new(follow::check::UserFollowCheckAdapter),
            delete: Arc::new(follow::del::UserFollowDelAdapter),
            get: Arc::new(follow::get::UserFollowGetAdapter),
            list: Arc::new(follow::list::UserFollowListAdapter),
            manage: Arc::new(follow::manage::UserFollowManageAdapter),
            stat: Arc::new(follow::stat::UserFollowStatAdapter),
        },
        // FRIEND - 朋友
        friend: UserFriendPort {
            add: Arc::new(friend::FriendAdapter),
            check: Arc::new(friend::FriendAdapter),
            del: Arc::new(friend::FriendAdapter),
            get: Arc::new(friend::FriendAdapter),
            list: Arc::new(friend::FriendAdapter),
            manage: Arc::new(friend::FriendAdapter),
        },
        // ROLE - 角色
        role: UserRolePort {
            add: Arc::new(role::add::RoleAddAdapter),
            check: Arc::new(role::check::UserRoleCheckAdapter),
            del: Arc::new(role::del::UserRoleDelAdapter),
            get: Arc::new(role::get::UserRoleGetAdapter),
            list: Arc::new(role::list::UserRoleListAdapter),
            manage: Arc::new(role::manage::UserRoleManageAdapter),
        },
        // SHARE - 分享
        share: UserSharePort {
            add: Arc::new(share::add::UserShareAddAdapter),
            check: Arc::new(share::check::ShareCheckAdapter),
            del: Arc::new(share::del::ShareDelAdapter),
            get: Arc::new(share::get::ShareGetAdapter),
            list: Arc::new(share::list::ShareListAdapter),
            manage: Arc::new(share::manage::ShareManageAdapter),
            stat: Arc::new(share::stat::UserShareStatAdapter),
        },
        // USER - 用户
        user: UserPort {
            add: Arc::new(user::add::UserAddAdapter),
            check: Arc::new(user::check::UserCheckAdapter),
            del: Arc::new(user::del::UserDelAdapter),
            get: Arc::new(user::get::UserGetAdapter),
            list: Arc::new(user::list::UserListAdapter),
            manage: Arc::new(user::manage::UserManageAdapter),
        },
        // VIEW - 浏览
        view: UserViewPort {
            add: Arc::new(view::add::ViewAddService),
            check: Arc::new(view::check::ViewCheckService),
            del: Arc::new(view::del::ViewDelService),
            get: Arc::new(view::get::ViewGetService),
            list: Arc::new(view::list::ViewListService),
            manage: Arc::new(view::manage::ViewManageService),
            stat: Arc::new(view::stat::UserViewStatAdapter),
        },
        // VIP - 贵宾
        vip: UserVipPort {
            add: Arc::new(vip::VipAdapter),
            check: Arc::new(vip::VipAdapter),
            del: Arc::new(vip::VipAdapter),
            get: Arc::new(vip::VipAdapter),
            list: Arc::new(vip::VipAdapter),
            manage: Arc::new(vip::VipAdapter),
        },
    }
}

//////// END
