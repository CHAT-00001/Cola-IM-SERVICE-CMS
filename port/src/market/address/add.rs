// port/src/market/address/add.rs
// ⏩️ 端口 - MARKET - 地址簿 - 发布
// 2026/8/4 22:10 Created.

////////

use cola_data::market::command::address::AddressCommand;
use cola_data::market::info::address::AddressInfo;
////////

/// # [ADD PORTS] - 发布
/// * `desc`: `MARKER - 地址发布端口`
#[async_trait::async_trait]
pub trait AddressAddPort: Send + Sync {
    //

    ////////

    /// # [PORT] - 添加新地址
    async fn add_address(
        &self,
        uid: i64,
        cmd: AddressCommand, // 命令
    ) -> anyhow::Result<(AddressInfo)>;

    ////////

    /// # [PORT] - 更新
    async fn update_address(
        &self,
        uid: i64,
        cmd: AddressCommand, // 命令
    ) -> anyhow::Result<(AddressInfo)>;
}

//////// END
