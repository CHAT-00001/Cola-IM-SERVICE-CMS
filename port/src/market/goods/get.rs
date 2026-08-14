// port/src/market/goods/get.rs
// ⏩️ 端口 - MARKET - 商品 - 获取
// 2026/8/5 00:00 Created.

////////

use cola_data::market::info::goods::goods::GoodsInfo;

////////

/// # [GET PORTS] -  获取
/// * `desc`: `获取视频`
#[async_trait::async_trait]
pub trait GoodsGetPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 我的商品
    async fn get_my_list(
        &self,
        user_id: i64,            // 用户 ID
        keyword: Option<String>, // 关键词
        limit: i64,              // 数量
        offset: i64,             // 页码
        status_code: i16,        // 状态码
    ) -> anyhow::Result<(Vec<GoodsInfo>)>;

    ////////

    /// # 2. [PORT] - 店铺的商品
    async fn get_list_by_shop_id(
        &self,
        shop_id: i64,            // 商店 ID
        keyword: Option<String>, // 关键词
        limit: i64,              // 数量
        offset: i64,             // 页码
        status_code: i16,        // 状态码
    ) -> anyhow::Result<(Vec<GoodsInfo>)>;

    ////////

    /// # 3. [PORT] - 商场的商品
    async fn get_list_by_mall_id(
        &self,
        mall_id: i64,            // 商场 ID
        keyword: Option<String>, // 关键词
        limit: i64,              // 数量
        offset: i64,             // 页码
        status_code: i16,        // 状态码
    ) -> anyhow::Result<(Vec<GoodsInfo>)>;

    ////////

    /// # 4. [PORT] - 按IDs批量获取
    async fn single_get_list_by_id(
        &self,
        goods_ids: i64, // 商品 ID
    ) -> anyhow::Result<(Vec<GoodsInfo>)>;

    ////////

    /// # 5. [PORT] - 按IDs批量获取
    async fn batch_get_list_by_ids(
        &self,
        goods_ids: &[i64], // 商品 IDs
    ) -> anyhow::Result<(Vec<GoodsInfo>)>;

    ////////
}

//////// END
