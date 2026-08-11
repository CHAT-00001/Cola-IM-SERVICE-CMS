// repo_adapter/src/market/address/check.rs
// 🔌 插头 - MARKET - ADDRESS - 检查
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::address::check::AddressCheckPort;

////////

/// # [CHECK ADAPTER] - 检查
/// * `desc`: `MARKET -  地址检查服务`
pub struct AddressCheckAdapter;

#[async_trait]
impl AddressCheckPort for AddressCheckAdapter {
    async fn check_status(&self, address_id: i64) -> Result<i16> {
        todo!()
    }

    async fn is_owner(&self, user_id: i64, address_id: i64) -> Result<bool> {
        todo!()
    }
}

//////// END
