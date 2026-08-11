// repo_adapter/src/market/cart/get.rs
// 🔌 插头 - MARKET - CART - 获取
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::video::VideoInfo;
use port::market::cart::get::CartGetPort;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `购物车获取适配器`
pub struct CartGetAdapter;

// 构造实现
#[async_trait]
impl CartGetPort for CartGetAdapter {
    async fn get_my_list(
        &self,
        uid: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
        is_liked: bool,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_he_list(
        &self,
        uid: i64,
        user_id: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }
}

//////// END
