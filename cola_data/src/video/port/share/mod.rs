// video/port/share/mod.rs
// 视频 - port - 分享 - 模块
// 2026/8/5 15:15 Created.

////////

use crate::video::port::share::add::VideoShareAddPort;
use crate::video::port::share::check::VideoShareCheckPort;
use crate::video::port::share::del::VideoShareDelPort;
use crate::video::port::share::get::VideoShareGetPort;
use crate::video::port::share::list::VideoShareListPort;
use crate::video::port::share::manage::VideoShareManagePort;
use crate::video::port::share::stat::VideoShareStatPort;
use std::sync::Arc;

////////
mod add;
mod check;
mod del;
mod get;
mod list;
mod manage;
mod stat;

////////

/// # [SHARE PORT]
/// * `desc`: `分享 Ports`
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
