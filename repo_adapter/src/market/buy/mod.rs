// repo_adapter/src/market/buy/mod.rs -- 适配器 - MARKET - 购买订单
// 2026/8/10 20:00 Updated.

////////

use port::market::buy::GoodsBuyPort;
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

/// # [BUILD] - 构建 BUY Port
/// * `desc`: 购买订单端口构造器

pub fn build_buy_port() -> GoodsBuyPort {
    GoodsBuyPort {
        add: Arc::new(add::GoodsBuyAddAdapter),
        check: Arc::new(check::GoodsBuyCheckAdapter),
        del: Arc::new(del::GoodsBuyDelAdapter),
        get: Arc::new(get::GoodsBuyGetAdapter),
        list: Arc::new(list::GoodsBuyListAdapter),
        manage: Arc::new(manage::GoodsBuyManageAdapter),
        stat: Arc::new(stat::GoodsBuyStatAdapter),
    }
}
//////// END
