// repo_adapter/src/aaaa/node/mod.rs -- 适配器 - BASIC - NODE - mod
// 2026/5/8 14:10 Created.

////////

use port::basic::node::NodePort;
use std::sync::Arc;

////////

pub mod add; // 发布
pub mod alive; // 存活
pub mod check; // 检查
pub mod config; // 配置管理
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [BUILD] - 构建 NODE Port
/// * `desc`: `COLA BASIC NODE Service Adapter`
pub fn build_basic_node_port() -> NodePort {
    NodePort {
        add: Arc::new(add::NodeAddAdapter),          // 发布
        check: Arc::new(check::NodeCheckAdapter),    // 检查
        config: Arc::new(config::NodeConfigAdapter), // 配置
        del: Arc::new(del::NodeDelAdapter),          // 逻辑删除
        get: Arc::new(get::NodeGetAdapter),          // 获取
        list: Arc::new(list::NodeListAdapter),       // 列表
        manage: Arc::new(manage::NodeManageAdapter), // 管理
        stat: Arc::new(stat::NodeStatAdapter),       // 统计
    }
}

//////// END
