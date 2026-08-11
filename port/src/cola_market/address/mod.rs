// port/src/market/address/mod.rs
// ⏩️ 端口 - MARKET -  地址簿 - mod
// 2026/8/5 14:32 Created.

////////

use crate::cola_market::address::active::AddressActivePort;
use crate::cola_market::address::add::AddressAddPort;
use crate::cola_market::address::delete::AddressDeletePort;
use crate::cola_market::address::get::AddressGetPort;
use crate::cola_market::address::list::AddressListPort;
use crate::cola_market::address::manage::AddressManagePort;
use crate::cola_market::address::stat::AddressStatPort;
use std::sync::Arc;

////////
pub mod active; // 活跃
pub mod add; // 发布
pub mod delete; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 计数

////////

/// # [ADDRESS PORTS]
/// * `desc`: `MARKET - 地址簿端口`
#[derive(Clone)]
pub struct AddressPort {
    pub active: Arc<dyn AddressActivePort + Send + Sync + 'static>,
    pub add: Arc<dyn AddressAddPort + Send + Sync + 'static>,
    pub delete: Arc<dyn AddressDeletePort + Send + Sync + 'static>,
    pub get: Arc<dyn AddressGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn AddressListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn AddressManagePort + Send + Sync + 'static>,
    pub stat: Arc<dyn AddressStatPort + Send + Sync + 'static>,
}

//////// END
