// data/src/cola_video/port/collect/mod.rs
// 数据 - ▶ 可乐视频 - PORT - 收藏 - 模块
// 2026/6/10 08:23 Created.

////////

use crate::cola_video::port::collect::manage::CollectManagePort;
use add::CollectAddPort;
use check::CollectCheckPort;
use del::CollectDelPort;
use get::CollectGetPort;
use list::CollectListPort;
use stat::CollectStatPort;
use std::sync::Arc;

////////

pub mod active; // 活跃
pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 软删除
pub mod get; // 获取
pub mod ids; // IDs
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [COLLECT PORTS]
/// * `desc`: `视频收藏 Ports`
#[derive(Clone)]
pub struct VideoCollectPort {
    pub add: Arc<dyn CollectAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn CollectCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn CollectDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn CollectGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn CollectListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn CollectManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn CollectStatPort + Send + Sync + 'static>, // 统计
}

//////// END
