// repo_adapter/src/basic/bonding/mod.rs -- ADAPTER - BASIC - bonding - mod
// 2026/8/8 Created.

////////

use port::basic::bonding::BondingPort;
use std::sync::Arc;

////////

pub mod add; // 发布
pub mod alive; // 存活
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [BUILD] - 构建 IDENTITY Port
/// * `desc`: 应用绑定
pub fn build_basic_bonding_port() -> BondingPort {
    BondingPort {
        add: Arc::new(add::FileAddAdapter),
        check: Arc::new(check::FileCheckAdapter),
        del: Arc::new(del::FileDelAdapter),
        get: Arc::new(get::FileGetAdapter),
        list: Arc::new(list::FileListAdapter),
        manage: Arc::new(manage::FileManageAdapter),
        stat: Arc::new(stat::FileStatAdapter),
    }
}

//////// END
