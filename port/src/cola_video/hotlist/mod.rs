// port/src/cola_video/hotlist/mod.rs
// ⏩️ 端口 - ▶ 可乐视频 - 上热门 - 模块
// 2026/8/5 15:53 Created.

////////

use crate::cola_video::hotlist::add::VideoHotlistAddPort;
use crate::cola_video::hotlist::check::VideoHotlistCheckPort;
use crate::cola_video::hotlist::del::VideoHotlistDelPort;
use crate::cola_video::hotlist::get::VideoHotlistGetPort;
use crate::cola_video::hotlist::list::VideoHotlistListPort;
use crate::cola_video::hotlist::manage::VideoHotlistManagePort;
use crate::cola_video::hotlist::stat::VideoHotlistStatPort;
use std::sync::Arc;

////////

pub mod add;
pub mod alive;
pub mod check;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;
pub mod stat;

////////

/// # [HOTLIST PORTS]
/// * `desc`: `▶ 可乐视频 - 视频上热门 Ports`
#[derive(Clone)]
pub struct VideoHotlistPort {
    pub add: Arc<dyn VideoHotlistAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn VideoHotlistCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn VideoHotlistDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn VideoHotlistGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn VideoHotlistListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn VideoHotlistManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn VideoHotlistStatPort + Send + Sync + 'static>, // 统计
}

//////// END
