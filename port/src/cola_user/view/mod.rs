// port/src/user/view/music.rs
// ⏩️ 端口 - 🗣 可乐用户 -  浏览 - 模块
// 2026/6/10 07:36

////////

use add::UserViewAddPort;
use check::UserViewCheckPort;
use del::UserViewDelPort;
use get::UserViewGetPort;
use list::UserViewListPort;
use manage::UserViewManagePort;
use stat::UserViewStatPort;
use std::sync::Arc;

////////

pub mod add;
pub mod check;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;
pub mod stat;

////////

/// # [VIEW PORTS]
/// * `desc`: `🗣 用户 - 用户浏览端口`
#[derive(Clone)]
pub struct UserViewPort {
    pub add: Arc<dyn UserViewAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn UserViewCheckPort + Send + Sync + 'static>,
    pub del: Arc<dyn UserViewDelPort + Send + Sync + 'static>,
    pub get: Arc<dyn UserViewGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn UserViewListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn UserViewManagePort + Send + Sync + 'static>,
    pub stat: Arc<dyn UserViewStatPort + Send + Sync + 'static>,
}

//////// END
