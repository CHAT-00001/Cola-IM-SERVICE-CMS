// port/src/cola_video/share/mod.rs
// ⏩️ 端口 - ▶ 可乐视频 - 分享 - mod
// 2026/8/5 15:15 Created.

////////

use add::VideoShareAddPort;
use check::VideoShareCheckPort;
use del::VideoShareDelPort;
use get::VideoShareGetPort;
use list::VideoShareListPort;
use manage::VideoShareManagePort;
use stat::VideoShareStatPort;
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

/// # [SHARE PORT]
/// * `desc`: `▶可乐视频 - 视频分享 Ports`
#[derive(Clone)]
pub struct VideoSharePort {
    pub add: Arc<dyn VideoShareAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn VideoShareCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn VideoShareDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn VideoShareGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn VideoShareListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn VideoShareManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn VideoShareStatPort + Send + Sync + 'static>, // 状态
}

//////// END
