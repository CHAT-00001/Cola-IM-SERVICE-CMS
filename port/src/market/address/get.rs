// port/src/market/address/get.rs
// ⏩️ 端口 - MARKET - 地址簿 - 获取
// 2026/8/4 22:10 Created.

////////

use cola_data::cola_market::info::address::AddressInfo;

////////

/// # [GET  PORTS]
/// * `desc`: `MARKET - 地址簿获取端口`
#[async_trait::async_trait]
pub trait AddressGetPort: Send + Sync {
    //

    ////////

    /// # [PORT] - 获取用户的地址信息
    async fn get_address_user_id(
        &self,
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<AddressInfo>)>;
}

//////// END
