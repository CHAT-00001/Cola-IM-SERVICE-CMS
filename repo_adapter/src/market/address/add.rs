// repo_adapter/src/market/address.rs
// 🔌 适配器 - 可乐MARKET - 地址簿
// 2026/6/18 14:02

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_market::command::address::AddressCommand;
use cola_data::cola_market::info::address::AddressInfo;
use port::market::address::add::AddressAddPort;

////////

/// # [ADDRESS ADAPTER] - 地址簿
/// * `desc`: `MARKET ADDRESS 适配器`
pub struct AddressAddAdapter;

#[async_trait]
impl AddressAddPort for AddressAddAdapter {
    async fn add_address(&self, uid: i64, cmd: AddressCommand) -> Result<(AddressInfo)> {
        todo!()
    }

    async fn update_address(&self, uid: i64, cmd: AddressCommand) -> Result<(AddressInfo)> {
        todo!()
    }
    //

    ////////
}

//////// END
