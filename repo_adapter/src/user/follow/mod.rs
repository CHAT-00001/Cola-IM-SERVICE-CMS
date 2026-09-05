// repo_adapter/src/user/follow/mod.rs -- 适配器 - USER - 关注 - mod
// 2026/8/6 10:20 Created.

////////

use port::cola_user::follow::UserFollowPort;
use port::cola_user::follow::add::UserFollowAddPort;
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

/// # [BUILDER] - 构造用户关注端口
/// * `DESC`: `COLA USER - Follow Ports.`
pub fn build_user_follow_port() -> UserFollowPort {
    UserFollowPort {
        add: Arc::new(add::FollowAddAdapter),
        check: Arc::new(check::UserFollowCheckAdapter),
        delete: Arc::new(del::UserFollowDelAdapter),
        get: Arc::new(get::UserFollowGetAdapter),
        list: Arc::new(list::UserFollowListAdapter),
        manage: Arc::new(manage::UserFollowManageAdapter),
        stat: Arc::new(stat::UserFollowStatAdapter),
    }
}

//////// END
