// repo_adapter/src/market/goods/list.rs
// 🔌 适配器 - MARKET - GOODS - 列表
// 2026/8/6 21:50 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::market::info::goods::goods::GoodsInfo;
use port::market::goods::list::GoodsListPort;
use rust_decimal::Decimal;

////////
/// # [LIST ADAPTER] - 商品 端口适配器
/// `desc`: `MARKET - 商品适配器`
pub struct GoodsListAdapter;

#[async_trait]
impl GoodsListPort for GoodsListAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 最新
    async fn get_new_list(&self, uid: i64, limit: i64, offset: i64) -> Result<(Vec<GoodsInfo>)> {
        todo!()
    }

    async fn get_hot_list(&self, uid: i64, limit: i64, offset: i64) -> Result<(Vec<GoodsInfo>)> {
        todo!()
    }

    async fn get_recommend_list(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<GoodsInfo>)> {
        todo!()
    }

    async fn get_category_list(
        &self,
        uid: i64,
        one_category_id: Option<i64>,
        two_category_id: Option<i64>,
        three_category_id: Option<i64>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<GoodsInfo>)> {
        todo!()
    }

    async fn get_channel_list(
        &self,
        uid: i64,
        channel_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<GoodsInfo>)> {
        todo!()
    }

    async fn get_city_list(
        &self,
        uid: i64,
        city_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<GoodsInfo>)> {
        todo!()
    }

    async fn get_nearby_list(
        &self,
        lat: f64,
        lng: f64,
        range: f64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<GoodsInfo>)> {
        todo!()
    }

    async fn get_search_list(
        &self,
        uid: i64,
        keyword: Option<String>,
        low_price: Option<Decimal>,
        high_price: Option<Decimal>,
        new_first: Option<bool>,
        range: Option<i32>,
        city: Option<i32>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<GoodsInfo>)> {
        todo!()
    }
}

//////// END
