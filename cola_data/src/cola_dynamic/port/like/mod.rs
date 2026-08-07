// cola_dynamic/port/like/mod.rs
// 动态 - port - 点赞 - 模块
// 2026/8/5 15:42 Created.

////////

use crate::cola_dynamic::port::like::add::LikeAddPort;
use crate::cola_dynamic::port::like::check::LikeCheckPort;
use crate::cola_dynamic::port::like::del::LikeDelPort;
use crate::cola_dynamic::port::like::get::LikeGetPort;
use crate::cola_dynamic::port::like::list::LikeListPort;
use crate::cola_dynamic::port::like::manage::LikeManagePort;
use crate::cola_dynamic::port::like::stat::LikeStatPort;
use std::sync::Arc;
////////

mod add; // 发布
mod check; // 检查
mod del; // 删除
mod get; // 获取
mod list; // 列表
mod manage; // 管理
mod stat; // 统计

////////

/// # [LIKE PORTS]
/// * `desc`: `动态点赞 Ports`
#[derive(Clone)]
pub struct DynamicLikePort {
    pub add: Arc<dyn LikeAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn LikeCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn LikeDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn LikeGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn LikeListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn LikeManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn LikeStatPort + Send + Sync + 'static>, // 统计
}

//////// END
