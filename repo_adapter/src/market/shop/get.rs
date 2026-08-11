// repo_adapter/src/market/shop/get.rs
// 🔌 插头 - MARKET - 商店 - 获取IDs
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_market::info::shop::shop_apply::ShopInfo;
use port::market::shop::get::ShopGetPort;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `🔌 视频发布插头`
pub struct ShopGetAdapter;

// 构造实现
#[async_trait]
impl ShopGetPort for ShopGetAdapter {
    async fn get_my_list(
        &self,
        uid: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
        is_liked: bool,
    ) -> Result<(Vec<ShopInfo>)> {
        todo!()
    }

    async fn get_he_list(
        &self,
        uid: i64,
        user_id: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<ShopInfo>)> {
        todo!()
    }
}

//////// END
