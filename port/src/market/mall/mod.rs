// port/src/cola_video/video/mod.rs
// ⏩️ 端口 - ▶ 可乐视频 - 视频 - music
// 2026/8/5 15:57 Created.

////////

use crate::cola_video::video::add::VideoAddPort;
use crate::cola_video::video::check::VideoCheckPort;
use crate::cola_video::video::del::VideoDeletePort;
use crate::cola_video::video::get::VideoGetPort;
use crate::cola_video::video::list::VideoListPort;
use crate::cola_video::video::manage::VideoManagePort;
use crate::cola_video::video::stat::VideoStatPort;
use std::sync::Arc;

////////

pub mod add; // 发布
pub mod check; // 检查
pub mod count; // 计数
pub mod del; // 删除
pub mod feed;
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [VIDEO PORTS] - 视频
/// * `desc`: `▶ 可乐视频 - 视频 Ports`
#[derive(Clone)]
pub struct VideoPort {
    pub add: Arc<dyn VideoAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn VideoCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn VideoDeletePort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn VideoGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn VideoListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn VideoManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn VideoStatPort + Send + Sync + 'static>, // 管理
}

//////// END
