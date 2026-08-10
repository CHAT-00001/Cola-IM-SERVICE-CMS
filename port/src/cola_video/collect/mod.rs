// port/src/cola_video/collect/mod.rs
// ⏩️ 端口 - ▶ 可乐视频 - 收藏 - 模块
// 2026/6/10 08:23 Created.

////////

use add::VideoCollectAddPort;
use check::VideoCollectCheckPort;
use del::VideoCollectDelPort;
use get::VideoCollectGetPort;
use list::VideoCollectListPort;
use manage::VideoCollectManagePort;
use stat::VideoCollectStatPort;
use std::sync::Arc;

////////

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
/// * `desc`: `▶ 可乐视频 - 视频收藏 Ports`
#[derive(Clone)]
pub struct VideoCollectPort {
    pub add: Arc<dyn VideoCollectAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn VideoCollectCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn VideoCollectDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn VideoCollectGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn VideoCollectListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn VideoCollectManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn VideoCollectStatPort + Send + Sync + 'static>, // 统计
}

//////// END
