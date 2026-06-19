// /address.rs  -- 地址簿 端口
// 2026/6/18 13:06

////////

use crate::market::command::address::AddressCommand;
use crate::market::info::address::AddressInfo;

////////

/// # [SERVICE PORT] - 地址簿 服务端口
#[async_trait::async_trait]
pub trait AddressPort: Send + Sync {

    ////////

    /// # 1. [PORT] - 保存地址
    async fn save_address(
        &self,
        uid: i64,
        cmd: AddressCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 编辑地址
    async fn update_address(
        &self,
        uid: i64,
        address_id: i64,
        cmd: AddressCommand,
    ) -> anyhow::Result<()>;


    ////////

    /// # 3. [PORT] - 设置默认地址
    async fn setting_default(
        &self,
        uid: i64,
        address_id: i64,
    ) -> anyhow::Result<()>;


    ////////

    /// # 4. [PORT] - 删除地址
    async fn delete_address(
        &self,
        uid: i64,
        address_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # 5. [PORT] - 获取我的地址列表
    async fn get_address_by_user_id(
        &self,
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<AddressInfo>)>;

    ////////

    /// # 6. [PORT] - 获取一个地址
    async fn get_address_by_id(
        &self,
        uid: i64,
        address_id: i64,
    ) -> anyhow::Result<(AddressInfo)>;

    ////////

    /// # 7. [PORT] - 根据用户ID删除所有地址
    /// * `desc` 用户/删除注销时
    async fn delete_address_by_user_id(
        &self,
        uid: i64,
        user_id: i64,
    ) -> anyhow::Result<()>;

}