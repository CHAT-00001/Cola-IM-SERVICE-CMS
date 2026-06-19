// repo_adapter/src/market/goods_feed.rs  -- 适配器 - 商品 FEED
// 2026/6/18

//////

use async_trait::async_trait;
use rust_decimal::Decimal;
use cola_data::market::info::goods::GoodsInfo;
use cola_data::market::port::goods_feed::GoodsFeedPort;
use repo::market::pg::goods::GoodsRepo;

//////

/// # [ADAPTER] - 商品 FEED 端口适配器
pub struct GoodsFeedAdapter;

#[async_trait]
impl GoodsFeedPort for GoodsFeedAdapter {

    async fn feed_recommend(&self, _uid: i64, offset: i64, limit: i64) -> anyhow::Result<Vec<GoodsInfo>> {
        let entities = GoodsRepo::find_recommend(offset, limit).await?;
        Ok(entities.into_iter().map(GoodsInfo::from).collect())
    }

    async fn feed_category(
        &self,
        one_class_id: Option<i16>,
        two_class_id: Option<i16>,
        three_class_id: Option<i16>,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<GoodsInfo>> {
        let entities = GoodsRepo::find_by_category(one_class_id, two_class_id, three_class_id, offset, limit).await?;
        Ok(entities.into_iter().map(GoodsInfo::from).collect())
    }

    async fn change_nearby(&self, _lat: f64, _lng: f64, _offset: i64, _limit: i64) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("not implemented"))
    }

    async fn delete_city(&self, _city_id: i64, _offset: i64, _limit: i64) -> anyhow::Result<Vec<GoodsInfo>> {
        Err(anyhow::anyhow!("not implemented"))
    }

    async fn feed_search(
        &self,
        keyword: Option<String>,
        category_id: Option<i16>,
        _low_price: Option<Decimal>,
        _high_price: Option<Decimal>,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<GoodsInfo>> {
        let kw = keyword.unwrap_or_default();
        let entities = GoodsRepo::search(&kw, category_id, offset, limit).await?;
        Ok(entities.into_iter().map(GoodsInfo::from).collect())
    }
}
