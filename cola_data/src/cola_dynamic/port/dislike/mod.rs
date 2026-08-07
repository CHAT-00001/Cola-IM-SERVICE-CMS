// dislike/mod.rs
// 不喜欢 - 模块
// 2026/8/5 15:53 Created.

////////

use crate::cola_dynamic::port::dislike::add::DislikeAddPort;
use crate::cola_dynamic::port::dislike::del::DelPort;
use crate::cola_dynamic::port::dislike::get::DislikeGetPort;
use crate::cola_dynamic::port::dislike::list::DislikeListPort;
use crate::cola_dynamic::port::dislike::manage::DislikeManagePort;
use crate::cola_dynamic::port::dislike::stat::DislikeStatPort;
use std::sync::Arc;

////////

mod add; // 发布
mod del; // 删除
mod get; // 获取
mod list; // 列表
mod manage; // 管理
mod stat; // 统计

////////

/// # [DISLIKE PORTS]
/// * `desc`: `动态不喜欢 Ports`
#[derive(Clone)]
pub struct DislikePort {
    pub add: Arc<dyn DislikeAddPort + Send + Sync + 'static>, // 发布
    pub del: Arc<dyn DelPort + Send + Sync + 'static>,        // 删除
    pub get: Arc<dyn DislikeGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn DislikeListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn DislikeManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn DislikeStatPort + Send + Sync + 'static>, // 统计
}

//////// END
