// repo_adapter/src/market/express/mod.rs
// 🔌 适配器 - MARKET - 快递
// 2026/8/10 20:00 Updated.

////////

use port::market::express::ExpressPort;
use std::sync::Arc;
////////

// 模块占位符 - 待实现具体操作
pub mod add; // 发布
pub mod alive; // 存活
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [BUILD] - 构建 EXPRESS Port
/// * `desc`: 快递端口构造器
pub fn build_express_port() -> ExpressPort {
    ExpressPort {
        active: Arc::new(alive::ExpressAliveAdapter),
        add: Arc::new(add::ExpressAddAdapter),
        check: Arc::new(check::ExpressCheckAdapter),
        delete: Arc::new(del::ExpressDelAdapter),
        get: Arc::new(get::ExpressGetAdapter),
        list: Arc::new(list::ExpressListAdapter),
        manage: Arc::new(manage::ExpressManageAdapter),
        stat: Arc::new(stat::ExpressStatAdapter),
    }
}

//////// END
