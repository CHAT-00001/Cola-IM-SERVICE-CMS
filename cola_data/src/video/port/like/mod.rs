// video/port/like/mod.rs
// 视频 - port - 点赞 - 模块
// 2026/8/5 15:42 Created.

////////

mod add; // 发布
mod check; // 检查
mod del; // 删除
mod get; // 获取
mod list; // 列表
mod manage; // 管理
mod stat; // 统计

////////

use crate::video::port::danmaku::DanmakuAddPort;
use crate::video::port::like::add::LikeAddPort;
use crate::video::port::like::check::LikeCheckPort;
use crate::video::port::like::del::LikeDelPort;
use crate::video::port::like::get::LikeGetPort;
use crate::video::port::like::list::LikeListPort;
use crate::video::port::like::manage::LikeManagePort;
use crate::video::port::like::stat::LikeStatPort;
use std::sync::Arc;

////////

/// # [LIKE PORTS]
/// * `desc`: `视频点赞 Ports`
#[derive(Clone)]
pub struct LikePort {
    pub add: Arc<dyn LikeAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn LikeCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn LikeDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn LikeGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn LikeListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn LikeManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn LikeStatPort + Send + Sync + 'static>, // 统计
}

//////// END
