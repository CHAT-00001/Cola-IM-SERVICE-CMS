// /express.rs  --
// ⏩️ 端口 - 可乐市场 - 快递 - mod
// 2026/6/18 14:00

////////

// port/goods.rs  --
// 2026/6/18 13:24

////////

use cola_data::cola_market::command::goods::GoodsCommand;
use cola_data::cola_market::info::address::AddressInfo;


////////

/// # [SERVICE PORT] - 商品 服务端口
#[async_trait::async_trait]
pub trait ExpressPort: Send + Sync {

    ////////

    /// # 1. [PORT] - 保存
    async fn save_goods(
        &self,
        uid: i64,
        cmd: GoodsCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 编辑
    async fn update_goods(
        &self,
        uid: i64,
        goods_id: i64,
        cmd: GoodsCommand,
    ) -> anyhow::Result<()>;


    ////////

    /// # 3. [PORT] - 修改
    /// * `desc` 上架/下架
    async fn change_status(
        &self,
        uid: i64,
        goods_id: i64,
    ) -> anyhow::Result<()>;


    ////////

    /// # 4. [PORT] - 删除
    async fn delete_goods(
        &self,
        uid: i64,
        goods_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # 5. [PORT] - 获取我的商品
    async fn get_address_by_user_id(
        &self,
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<AddressInfo>)>;

    ////////

    /// # 6. [PORT] - 浏览一个商品
    async fn view_goods_by_id(
        &self,
        uid: i64,
        goods_id: i64,
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
