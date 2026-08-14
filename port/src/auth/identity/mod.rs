// port/src/auth/identity/mod.rs
// ⏩️ 端口 - AUTH - 身份识别 - 模块
// 2026/8/5 15:11 Created.

////////


use std::sync::Arc;
use crate::auth::identity::add::IdentityAddPort;
use crate::auth::identity::check::IdentityCheckPort;
use crate::auth::identity::del::IdentityDelPort;
use crate::auth::identity::get::IdentityGetPort;
use crate::auth::identity::list::IdentityListPort;
use crate::auth::identity::manage::IdentityManagePort;
use crate::auth::identity::stat::IdentityStatPort;

////////
pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除

pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [AUTH IDENTITY PORT]
/// * `desc`: `AUTH - 身份识别 Ports`
#[derive(Clone)]
pub struct AuthIdentityPort {
    pub add: Arc<dyn IdentityAddPort + Send + Sync + 'static>,        // 发布
    pub check: Arc<dyn IdentityCheckPort + Send + Sync + 'static>,    // 检查
    pub del: Arc<dyn IdentityDelPort + Send + Sync + 'static>,        // 删除
    pub get: Arc<dyn IdentityGetPort + Send + Sync + 'static>,        // 获取
    pub list: Arc<dyn IdentityListPort + Send + Sync + 'static>,      // 列表
    pub manage: Arc<dyn IdentityManagePort + Send + Sync + 'static>,  // 管理
    pub stat: Arc<dyn IdentityStatPort + Send + Sync + 'static>,      // 统计
}

//////// END