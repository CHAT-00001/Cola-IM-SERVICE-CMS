// collect/mod.rs
// 视频 - port - 收藏 - 模块
// 2026/6/10 08:23 Created.

////////

use add::CollectAddPort;
use check::CollectCheckPort;
use del::CollectDelPort;
use get::CollectGetPort;
use list::CollectListPort;
use manage::ManagePort;
use stat::CollectStatPort;
use std::sync::Arc;

////////

mod active;
mod add;
mod check;
mod del;
mod get;
mod ids;
mod list;
mod manage;
mod stat;

////////

/// # [COLLECT PORTS]
/// * `desc`: `视频收藏 Ports`
#[derive(Clone)]
pub struct CollectPort {
    pub add: Arc<dyn CollectAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn CollectCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn CollectDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn CollectGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn CollectListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn ManagePort + Send + Sync + 'static>,  // 管理
    pub stat: Arc<dyn CollectStatPort + Send + Sync + 'static>, // 统计
}

//////// END
