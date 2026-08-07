// video/port/video/mod.rs
// 视频 - port - 视频 - 模块
// 2026/8/5 15:57 Created.

////////

use std::sync::Arc;
use crate::video::port::video::add::{VideoAddPort};
use crate::video::port::video::check::VideoCheckPort;
use crate::video::port::video::del::VideoDelPort;
use crate::video::port::video::get::VideoGetPort;
use crate::video::port::video::list::VideoListPort;
use crate::video::port::video::manage::VideoManagePort;
use crate::video::port::video::stat::VideoStatPortt;

////////

pub mod active; // 活跃
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


/// # [BUY PORTS]
/// * `desc`: `视频购买 Ports`
#[derive(Clone)]
pub struct VideoPort {
    pub add: Arc<dyn VideoAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn VideoCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn VideoDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn VideoGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn VideoListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn VideoManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn VideoStatPortt + Send + Sync + 'static>, // 管理
}

//////// END
