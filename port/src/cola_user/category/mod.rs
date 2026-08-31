// user/port/category/mod.rs
// ⏩️ 端口 - 🗣 可乐用户 - 分类 - music
// 2026/8/10 04:02 Created.

////////

use add::UserCategoryAddPort;
use check::UserCategoryCheckPort;
use del::UserCategoryDeletePort;
use get::UserCategoryGetPort;
use list::UserCategoryListPort;
use manage::UserCategoryManagePort;
use std::sync::Arc;

////////

pub mod add; // 添加
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage;
mod stat;
// 管理

////////

/// # [CATEGORY PORTS]
/// * `desc`: `🗣 用户 - 用户分类端口`
#[derive(Clone)]
pub struct UserCategoryPort {
    pub add: Arc<dyn UserCategoryAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn UserCategoryCheckPort + Send + Sync + 'static>,
    pub delete: Arc<dyn UserCategoryDeletePort + Send + Sync + 'static>,
    pub get: Arc<dyn UserCategoryGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn UserCategoryListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn UserCategoryManagePort + Send + Sync + 'static>,
}

//////// END
