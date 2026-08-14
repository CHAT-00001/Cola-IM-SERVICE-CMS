// repo_adapter/src/market/view/list.rs
// 🔌 适配器 - MARKET - 商品浏览 - 列表
// 2026/8/7 05:56 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::market::info::goods::view::GoodsViewInfo;
use port::market::view::list::GoodsViewListPort;

////////

/// # [LIST ADAPTER] - 浏览列表
/// * `desc`: `商品浏览列表服务`
pub struct GoodsViewListAdapter;

// 构造实现
#[async_trait]
impl GoodsViewListPort for GoodsViewListAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 用户的主动浏览记录
    async fn get_view_infos_by_user_id(
        &self,
        _user_id: i64, // 用户 ID
        _limit: i64,
        _offset: i64,
    ) -> Result<(Vec<GoodsViewInfo>)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 商品的被动浏览记录
    async fn get_view_infos_by_video_id(
        &self,
        _video_id: i64, // 商品 ID
        _limit: i64,
        _offset: i64,
    ) -> Result<(Vec<GoodsViewInfo>)> {
        todo!()
    }
}

//////// END
