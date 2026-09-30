// port/src/basic/node/music.rs -- PORT - BASIC - NODE - mod
// 2026/5/5 10:10 Created.

////////

use crate::basic::node::add::NodeAddPort;
use crate::basic::node::check::NodeCheckPort;
use crate::basic::node::config::NodeConfigPort;
use crate::basic::node::del::NodeDelPort;
use crate::basic::node::get::NodeGetPort;
use crate::basic::node::list::NodeListPort;
use crate::basic::node::manage::NodeManagePort;
use crate::basic::node::stat::NodeStatPort;
use std::sync::Arc;

////////
pub mod add; // 发布
pub mod check; // 检查
pub mod config; // 配置管理
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [BASIC PORT]
/// * `desc`: `BASIC - 边缘节点`
#[derive(Clone)]
pub struct NodePort {
    pub add: Arc<dyn NodeAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn NodeCheckPort + Send + Sync + 'static>, // 检查
    pub config: Arc<dyn NodeConfigPort + Send + Sync + 'static>, // 配置管理
    pub del: Arc<dyn NodeDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn NodeGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn NodeListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn NodeManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn NodeStatPort + Send + Sync + 'static>, // 统计
}

//////// END
