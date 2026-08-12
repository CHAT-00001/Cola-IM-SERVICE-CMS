// port/src/cola_video/dislike/mod.rs
// ⏩️ 端口 - ▶ 可乐视频 - 不喜欢 - 模块
// 2026/8/5 15:53 Created.

////////

use crate::cola_video::dislike::add::VideoDislikeAddPort;
use crate::cola_video::dislike::del::VideoDislikeDelPort;
use crate::cola_video::dislike::get::VideoDislikeGetPort;
use crate::cola_video::dislike::list::VideoDislikeListPort;
use crate::cola_video::dislike::manage::VideoDislikeManagePort;
use crate::cola_video::dislike::stat::VideoDislikeStatPort;
use std::sync::Arc;

////////

pub mod add; // 发布
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [DISLIKE PORTS]
/// * `desc`: `▶ 可乐视频 - 视频不喜欢 Ports`
#[derive(Clone)]
pub struct VideoDislikePort {
    pub add: Arc<dyn VideoDislikeAddPort + Send + Sync + 'static>, // 发布
    pub del: Arc<dyn VideoDislikeDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn VideoDislikeGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn VideoDislikeListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn VideoDislikeManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn VideoDislikeStatPort + Send + Sync + 'static>, // 统计
}

//////// END
