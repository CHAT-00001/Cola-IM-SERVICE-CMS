// repo_adapter/src/market/address/active.rs
// 🔌 适配器 - 可乐MARKET - 地址簿 - 激活
// 2026/8/11 12:45 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::address::active::AddressActivePort;

////////

/// # [ACTIVE ADAPTER] - 激活
/// * `desc`: `MARKET ADDRESS 适配器`
pub struct AddressActiveAdapter;

#[async_trait]
impl AddressActivePort for AddressActiveAdapter {
    async fn set_default_address(&self, address_id: i64) -> Result<()> {
        todo!()
    }
    //

    ////////
}

//////// END
