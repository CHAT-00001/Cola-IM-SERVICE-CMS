// cola_video/port/hotlist/mod.rs
// 视频 - port - 上热门 - 模块
// 2026/8/5 15:53 Created.

////////

use crate::cola_video::port::hotlist::add::HotlistAddPort;
use crate::cola_video::port::hotlist::check::VideoHotlistCheckPort;
use crate::cola_video::port::hotlist::del::VideoHotlistDelPort;
use crate::cola_video::port::hotlist::get::VideoHotlistGetPort;
use crate::cola_video::port::hotlist::list::VideoHotlistListPort;
use crate::cola_video::port::hotlist::manage::VideoHotlistManagePort;
use crate::cola_video::port::hotlist::stat::VideoHotlistStatPort;
use std::sync::Arc;

////////

mod active;
mod add;
mod alive;
mod check;
mod del;
mod get;
mod list;
mod manage;
mod stat;

////////

/// # [HOTLIST PORTS]
/// * `desc`: `视频上热门 Ports`
#[derive(Clone)]
pub struct HotlistPort {
    pub add: Arc<dyn HotlistAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn VideoHotlistCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn VideoHotlistDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn VideoHotlistGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn VideoHotlistListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn VideoHotlistManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn VideoHotlistStatPort + Send + Sync + 'static>, // 统计
}

//////// END
