// repo_adapter/src/market/collec/get.rs
// 🔌 适配器 - MARKET - 商品收藏 - 获取
// 2026/8/9 20:42 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::collect::get::VideoCollectGetPort;
use port::market::collect::get::GoodsCollectGetPort;
////////

/// # [GET ADAPTER] - 获取
/// * `desc`: `商品收藏查询适配器`
#[derive(Debug, Default, Clone)]
pub struct GoodsCollectGetAdapter;

#[async_trait]
impl GoodsCollectGetPort for GoodsCollectGetAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 用户的
    /// * `desc`: `根据用户ID` - `批量获取视频IDs`
    async fn get_video_ids_by_user_id(
        &self,
        uid: i64,
        user_id: i64,  // 用户 ID
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<i64>)> {
        todo!()
    }
}

//////// END
