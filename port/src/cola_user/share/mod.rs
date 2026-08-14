// port/src/user/share/mod.rs
// ⏩️ 端口 - 🗣 用户 - 分享 - 模块
// 2026/8/5 21:33 Created.

////////

use add::UserShareAddPort;
use check::UserShareCheckPort;
use del::UserShareDelPort;
use get::UserShareGetPort;
use list::UserShareListPort;
use manage::UserShareManagePort;
use stat::UserShareStatPort;
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

/// # [USER SHARE PORTS]
/// * `desc`: `🗣 用户 - 用户主页分享端口`
#[derive(Clone)]
pub struct UserSharePort {
    pub add: Arc<dyn UserShareAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn UserShareCheckPort + Send + Sync + 'static>,
    pub del: Arc<dyn UserShareDelPort + Send + Sync + 'static>,
    pub get: Arc<dyn UserShareGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn UserShareListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn UserShareManagePort + Send + Sync + 'static>,
    pub stat: Arc<dyn UserShareStatPort + Send + Sync + 'static>,
}

//////// END
