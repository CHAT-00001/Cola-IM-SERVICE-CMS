// data/src/cola_video/port/view/mod.rs
// 🗄️ 数据 - ▶ 可乐视频 - port - 浏览 - 模块
// 2026/8/5 14:32 Created.

////////

use crate::cola_video::port::view::active::VideoViewActivePort;
use crate::cola_video::port::view::add::VideoViewAddPort;
use crate::cola_video::port::view::del::VideoViewDelPort;
use crate::cola_video::port::view::get::VideoViewGetPort;
use crate::cola_video::port::view::manage::VideoViewManagePort;
use crate::cola_video::port::view::stat::VideoViewStatPort;
use std::sync::Arc;
use crate::cola_video::port::view::list::VideoViewListPort;

////////
pub mod active; // 活跃
pub mod add; // 发布
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 计数

////////

/// # [VIEW PORTS]
/// * `desc`: `视频浏览服务端口`
#[derive(Clone)]
pub struct VideoViewPort {
    pub active: Arc<dyn VideoViewActivePort + Send + Sync + 'static>,
    pub add: Arc<dyn VideoViewAddPort + Send + Sync + 'static>,
    pub del: Arc<dyn VideoViewDelPort + Send + Sync + 'static>,
    pub get: Arc<dyn VideoViewGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn VideoViewListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn VideoViewManagePort + Send + Sync + 'static>,
    pub stat: Arc<dyn VideoViewStatPort + Send + Sync + 'static>,
}

//////// END
