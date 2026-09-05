// repo_adapter/src/user/follow/music.rs -- 适配器 - USER - 分类 - mod
// 2026/8/6 14:00 Created.

////////

use port::cola_user::category::UserCategoryPort;
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

/// # [BUILDER] - 构造用户分类端口
/// * `DESC`: `COLA USER - Categories Ports.`
pub fn build_user_category_port() -> UserCategoryPort {
    UserCategoryPort {
        add: Arc::new(add::CategoryAddAdapter),
        check: Arc::new(check::UserCategoryCheckAdapter),
        delete: Arc::new(del::CategoryDeleteAdapter),
        get: Arc::new(get::CategoryGetAdapter),
        list: Arc::new(list::CategoryListAdapter),
        manage: Arc::new(manage::CategoryManageAdapter),
        stat: Arc::new(stat::UserCategoryStatAdapter),
    }
}

//////// END
