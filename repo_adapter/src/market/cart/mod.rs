// repo_adapter/src/market/cart/mod.rs
// 🔌 适配器 - MARKET - 购物车
// 2026/8/10 20:00 Updated.

////////

use port::market::cart::CartPort;
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

/// # [BUILD] - 构建 CART Port
/// * `desc`: 购物车端口构造器
pub fn build_cart_port() -> CartPort {
    CartPort {
        add: Arc::new(add::CartAddAdapter),
        check: Arc::new(check::CartCheckAdapter),
        del: Arc::new(del::CartDelAdapter),
        get: Arc::new(get::CartGetAdapter),
        list: Arc::new(list::CartListAdapter),
        manage: Arc::new(manage::CartManageAdapter),
        stat: Arc::new(stat::CartStatAdapter),
    }
}

//////// END
