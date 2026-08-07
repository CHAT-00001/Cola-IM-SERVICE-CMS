// dislike/mod.rs
// 视频 - 不喜欢 - 模块
// 2026/8/5 15:53 Created.

////////

use crate::video::port::dislike::add::DislikeAddPort;
use crate::video::port::dislike::del::DislikeDelPort;
use crate::video::port::dislike::get::VideoDislikeGetService;
use crate::video::port::dislike::list::DislikeListPort;
use crate::video::port::dislike::mana::DislikeManagePort;
use crate::video::port::dislike::stat::VideoDislikeStatPort;
use std::sync::Arc;

////////

mod add; // 发布
mod del; // 删除
mod get; // 获取
mod list; // 列表
mod mana; // 管理
mod stat; // 统计

////////

/// # [DISLIKE PORTS]
/// * `desc`: `视频不喜欢 Ports`
#[derive(Clone)]
pub struct DislikePort {
    pub add: Arc<dyn DislikeAddPort + Send + Sync + 'static>, // 发布
    pub del: Arc<dyn DislikeDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn VideoDislikeGetService + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn DislikeListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn DislikeManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn VideoDislikeStatPort + Send + Sync + 'static>, // 统计
}

//////// END
