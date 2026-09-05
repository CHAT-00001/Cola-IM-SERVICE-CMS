// repo_adapter/src/user/follow/mod.rs -- 适配器 - USER - 关注 - mod
// 2026/8/6 10:20 Created.

////////

use port::cola_user::follow::UserFollowPort;
use port::cola_user::follow::add::UserFollowAddPort;
use port::cola_user::friend::UserFriendPort;
use std::sync::Arc;
////////

pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [BUILDER] - 构造用户朋友端口
/// * `DESC`: `COLA USER - Friend Ports.`
pub fn build_user_friend_port() -> UserFriendPort {
    UserFriendPort {
        add: Arc::new(add::FriendAddAdapter),
        check: Arc::new(check::FriendCheckAdapter),
        del: Arc::new(del::UserFriendDelAdapter),
        get: Arc::new(get::UserFriendGetAdapter),
        list: Arc::new(list::FriendListAdapter),
        manage: Arc::new(manage::FriendManageAdapter),
        stat: Arc::new(stat::FriendStatAdapter),
    }
}

//////// END
