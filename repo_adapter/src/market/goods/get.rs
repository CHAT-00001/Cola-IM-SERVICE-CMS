// repo_adapter/src/market/goods/get.rs
// 🔌 适配器 - MARKET - GOODS - 获取详情/状态
// 2026/8/6 10:23 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::market::info::goods::goods::GoodsInfo;
use port::market::goods::get::GoodsGetPort;

////////

/// # [GET ADAPTER] - 商品 获取
/// `desc`: `MARKET - 商品适配器`
pub struct GoodsGetAdapter;

#[async_trait]
impl GoodsGetPort for GoodsGetAdapter {
    async fn get_my_list(
        &self,
        user_id: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
        status_code: i16,
    ) -> Result<(Vec<GoodsInfo>)> {
        todo!()
    }

    async fn get_list_by_shop_id(
        &self,
        shop_id: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
        status_code: i16,
    ) -> Result<(Vec<GoodsInfo>)> {
        todo!()
    }

    async fn get_list_by_mall_id(
        &self,
        mall_id: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
        status_code: i16,
    ) -> Result<(Vec<GoodsInfo>)> {
        todo!()
    }

    async fn single_get_list_by_id(&self, goods_ids: i64) -> Result<(Vec<GoodsInfo>)> {
        todo!()
    }

    async fn batch_get_list_by_ids(&self, goods_ids: &[i64]) -> Result<(Vec<GoodsInfo>)> {
        todo!()
    }
}

//////// END
