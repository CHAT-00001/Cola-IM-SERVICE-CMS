// repo_adapter/src/market/address/mod.rs
// 🔌 适配器 - MARKET - 地址
// 2026/8/10 20:00 Updated.

////////

use crate::stub;
use port::market::address::AddressPort;
use std::sync::Arc;

////////

pub mod active; // 激活
pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [BUILD] - 构建 ADDRESS Port
/// * `desc`: 地址端口构造器
pub fn build_address_port() -> AddressPort {
    AddressPort {
        active: Arc::new(active::AddressActiveAdapter),
        add: Arc::new(add::AddressAddAdapter),
        check: Arc::new(check::AddressCheckAdapter),
        delete: Arc::new(del::AddressDelAdapter),
        get: Arc::new(get::AddressGetAdapter),
        list: Arc::new(list::AddressListAdapter),
        manage: Arc::new(manage::AddressManageAdapter),
        stat: Arc::new(stat::AddressStatAdapter),
    }
}

//////// END
