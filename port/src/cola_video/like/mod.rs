// port/srs/cola_video/like/mod.rs
// ⏩️ 端口 - ▶ 可乐视频 - 点赞 - 模块
// 2026/8/5 15:42 Created.

////////

use add::VideoLikeAddPort;
use check::VideoLikeCheckPort;
use del::VideoLikeDelPort;
use get::VideoLikeGetPort;
use list::VideoLikeListPort;
use manage::VideoLikeManagePort;
use stat::VideoLikeStatPort;
use std::sync::Arc;

////////
pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [VIDEO LIKE PORTS]
/// * `desc`: `▶ 视频 - 视频点赞端口 Ports`
#[derive(Clone)]
pub struct VideoLikePort {
    pub add: Arc<dyn VideoLikeAddPort + Send + Sync + 'static>,       // 发布
    pub check: Arc<dyn VideoLikeCheckPort + Send + Sync + 'static>,   // 检查
    pub del: Arc<dyn VideoLikeDelPort + Send + Sync + 'static>,       // 删除
    pub get: Arc<dyn VideoLikeGetPort + Send + Sync + 'static>,       // 获取
    pub list: Arc<dyn VideoLikeListPort + Send + Sync + 'static>,     // 列表
    pub manage: Arc<dyn VideoLikeManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn VideoLikeStatPort + Send + Sync + 'static>,     // 统计
}

//////// END
