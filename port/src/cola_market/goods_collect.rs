// port/goods_collect.rs
// ⏩️ 端口 - 可乐市场 - 商品 - 收藏
// 2026/6/18 13:41

////////

use cola_data::cola_market::info::address::AddressInfo;

////////

/// # [SERVICE PORT] - 商品 收藏
#[async_trait::async_trait]
pub trait GoodsCollectPort: Send + Sync {

    ////////

    /// # 1. [PORT] - 保存
    async fn save_collect_record(
        &self,
        uid: i64,
        goods_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 删除
    async fn delete_collect_record(
        &self,
        uid: i64,
        goods_id: i64,
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

    /// # 4. [PORT] - 删除一条记录
    async fn delete_collect(
        &self,
        uid: i64,
        goods_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # 5. [PORT] - 获取我浏览过的商品IDs
    async fn get_collect_ids_by_user_id(
        &self,
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 6. [PORT] - 获取商品被多少人看过
    async fn get_collect_record_by_goods_id(
        &self,
        uid: i64,
        goods_id: i64,
    ) -> anyhow::Result<(AddressInfo)>;

    ////////

    /// # 7. [PORT] - 根据用户ID删除所有记录
    /// * `desc` 用户/删除注销时
    async fn delete_collect_by_user_id(
        &self,
        uid: i64,
        user_id: i64,
    ) -> anyhow::Result<()>;

}