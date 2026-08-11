// repo_adapter/src/market/address/get.rs
// 🔌 插头 - MARKET - ADDRESS - 获取
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_market::info::address::AddressInfo;
use port::market::address::get::AddressGetPort;

////////

/// # [GET ADAPTER] - 获取
/// * `desc`: `MARKET - 地址获取适配器`
pub struct AddressGetAdapter;

// 构造实现
#[async_trait]
impl AddressGetPort for AddressGetAdapter {
    /// # [ADAPTER] - 用户的地址
    async fn get_address_user_id(
        &self,
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<AddressInfo>)> {
        todo!()
    }
}

//////// END
