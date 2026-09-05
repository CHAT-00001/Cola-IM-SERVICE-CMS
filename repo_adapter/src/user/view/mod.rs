// repo_adapter/src/user/view/mod.rs -- 适配器 - USER - 浏览 - mod
// 2026/8/6 15:59 Created.

////////

use port::cola_user::profile::UserProfilePort;
use port::cola_user::view::UserViewPort;
use std::sync::Arc;

pub mod add;
pub mod check;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;
pub mod stat;

////////

/// # [BUILDER] - 构造用户浏览端口
/// * `DESC`: `COLA USER - View Ports.`
pub fn build_user_view_port() -> UserViewPort {
    UserViewPort {
        add: Arc::new(add::ViewAddAdapter),
        check: Arc::new(check::UserViewCheckAdapter),
        del: Arc::new(del::UserViewDelAdapter),
        get: Arc::new(get::UserViewGetAdapter),
        list: Arc::new(list::UserViewListAdapter),
        manage: Arc::new(manage::UserViewManageAdapter),
        stat: Arc::new(stat::UserViewStatAdapter),
    }
}

//////// END
