// port/src/user/category/mod.rs -- 端口 - USER - 分类 - mod
// 2026/8/10 04:02 Created.

////////

use add::UserCategoryAddPort;
use check::UserCategoryCheckPort;
use del::UserCategoryDeletePort;
use get::UserCategoryGetPort;
use list::UserCategoryListPort;
use manage::UserCategoryManagePort;
use stat::UserCategoryStatPort;
use std::sync::Arc;

////////

pub mod add; // 添加
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [CATEGORY PORTS]
/// * `desc`: `COLA USER - Categories Ports.`
#[derive(Clone)]
pub struct UserCategoryPort {
    pub add: Arc<dyn UserCategoryAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn UserCategoryCheckPort + Send + Sync + 'static>,
    pub delete: Arc<dyn UserCategoryDeletePort + Send + Sync + 'static>,
    pub get: Arc<dyn UserCategoryGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn UserCategoryListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn UserCategoryManagePort + Send + Sync + 'static>,
    pub stat: Arc<dyn UserCategoryStatPort + Send + Sync + 'static>,
}

//////// END
