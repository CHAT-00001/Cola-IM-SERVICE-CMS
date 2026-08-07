// market/port/goods_feed.rs
// 市场 - port - 商品 - feed
// 2026/6/18 14:37

////////

use rust_decimal::Decimal;
use crate::market::info::goods::goods::GoodsInfo;

////////

/// # [SERVICE PORT] - 商品 收藏
#[async_trait::async_trait]
pub trait GoodsFeedPort: Send + Sync {
    ////////

    /// # 1. [PORT] - 推荐
    async fn feed_recommend(
        &self,
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<GoodsInfo>)>;

    ////////

    /// # 2. [PORT] - 分类
    /// `desc` 多级分类查找
    async fn feed_category(
        &self,
        one_class_id: Option<i16>,   // 一级分类ID
        two_class_id: Option<i16>,   // 二级分类ID
        three_class_id: Option<i16>, // 三级分类ID
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<GoodsInfo>)>;

    ////////

    /// # 3. [PORT] - 附近
    /// * `desc` GEO
    async fn change_nearby(
        &self,
        lat: f64,
        lng: f64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # 4. [PORT] - 同城
    async fn delete_city(
        &self,
        city_id: i64, // 城市ID
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<GoodsInfo>)>;

    ////////

    /// # 5. [PORT] - 搜索
    async fn feed_search(
        &self,
        keyword: Option<String>,     // 关键词
        category_id: Option<i16>,    // 分类ID
        low_price: Option<Decimal>,  // 最低价格
        high_price: Option<Decimal>, // 最高价格
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<GoodsInfo>)>;
}


//////// END