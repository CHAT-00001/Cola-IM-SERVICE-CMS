// repo_adapter/src/market/shop/mod.rs -- 适配器 - MARKET - 店铺
// 2026/8/10 20:00 Updated.

////////

use port::market::shop::ShopPort;
use std::sync::Arc;

////////

pub mod add; // 发布
pub mod appy; // 存活
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [BUILD] - 构建 SHOP Port
/// * `desc`: 店铺端口构造器
pub fn build_shop_port() -> ShopPort {
    ShopPort {
        add: Arc::new(add::ShopAddAdapter),
        appy: Arc::new(appy::ShopAppyAdapter),
        check: Arc::new(check::ShopCheckAdapter),
        del: Arc::new(del::ShopDelAdapter),
        get: Arc::new(get::ShopGetAdapter),
        list: Arc::new(list::ShopListAdapter),
        manage: Arc::new(manage::ShopManageAdapter),
        stat: Arc::new(stat::ShopStatAdapter),
    }
}

//////// END
