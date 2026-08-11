// port/src/market/goods/list.rs
// ⏩️ 端口 - MARKET -  商品 - 列表
// 2026/8/5 00:23 Created.

////////

use cola_data::cola_market::info::goods::goods::GoodsInfo;
use rust_decimal::Decimal;

////////

/// # [LIST PORTS] -  列表
/// * `desc`: `MARKET - 商品前台列表接口`
#[async_trait::async_trait]
pub trait GoodsListPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 新的
    async fn get_new_list(
        &self,
        uid: i64,    // UID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<GoodsInfo>)>;

    ////////

    /// # 2. [PORT] - 热门
    async fn get_hot_list(
        &self,
        uid: i64,    // UID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<GoodsInfo>)>;

    ////////

    /// # 3. [PORT] - 推荐
    async fn get_recommend_list(
        &self,
        uid: i64,    // UID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<GoodsInfo>)>;

    ////////

    /// # 4. [PORT] - 分类
    async fn get_category_list(
        &self,
        uid: i64,                       // UID
        one_category_id: Option<i64>,   // 一级分类ID
        two_category_id: Option<i64>,   // 二级分类ID
        three_category_id: Option<i64>, // 三级分类ID
        limit: i64,                     // 数量
        offset: i64,                    // 页码
    ) -> anyhow::Result<(Vec<GoodsInfo>)>;

    ////////

    /// # 5. [PORT] - 频道
    async fn get_channel_list(
        &self,
        uid: i64,        // UID
        channel_id: i64, // 通道 ID
        limit: i64,      // 数量
        offset: i64,     // 页码
    ) -> anyhow::Result<(Vec<GoodsInfo>)>;

    ////////

    /// # 6. [PORT] - 同城
    async fn get_city_list(
        &self,
        uid: i64,     // UID
        city_id: i64, // 城市 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<GoodsInfo>)>;

    ////////

    /// # 7. [PORT] - 附近
    async fn get_nearby_list(
        &self,
        lat: f64,    // 纬度
        lng: f64,    // 经度
        range: f64,  // 范围
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<GoodsInfo>)>;

    ////////

    /// # 8. [PORT] - 搜索
    async fn get_search_list(
        &self,
        uid: i64,                    // UID
        keyword: Option<String>,     // 关键词
        low_price: Option<Decimal>,  // 最低价格
        high_price: Option<Decimal>, // 最高价格
        new_first: Option<bool>,     // 最新发布优先
        range: Option<i32>,          // 半径范围
        city: Option<i32>,           // 发货地城市 ID
        limit: i64,                  // 数量
        offset: i64,                 // 页码
    ) -> anyhow::Result<(Vec<GoodsInfo>)>;
}

//////// END
