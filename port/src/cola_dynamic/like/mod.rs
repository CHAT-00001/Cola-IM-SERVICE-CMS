// port/src/cola_dynamic/port/like/mod.rs
// ⏩️ 端口 - ⏹ 可乐动态 - port - 点赞 - 模块
// 2026/8/5 15:42 Created.

////////

use crate::cola_dynamic::like::add::LikeAddPort;
use crate::cola_dynamic::like::check::LikeCheckPort;
use crate::cola_dynamic::like::del::LikeDelPort;
use crate::cola_dynamic::like::get::LikeGetPort;
use crate::cola_dynamic::like::list::LikeListPort;
use crate::cola_dynamic::like::manage::LikeManagePort;
use crate::cola_dynamic::like::stat::LikeStatPort;
use std::sync::Arc;

////////

pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [DYNAMIC LIKE PORTS]
/// * `desc`: `⏹ 可乐动态 - 点赞 Ports`
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
