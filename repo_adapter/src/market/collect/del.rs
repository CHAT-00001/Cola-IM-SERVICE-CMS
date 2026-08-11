// repo_adapter/src/market/collect/del.rs
// 🔌 适配器 - MARKET - 商品收藏 - 删除
// 2026/8/9 20:37 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::collect::del::VideoCollectDelPort;
use port::market::collect::del::GoodsCollectDelPort;
////////

/// # [DELETE ADAPTER] - 删除
/// * `desc`: `商品收藏适配器`
#[derive(Debug, Default, Clone)]
pub struct GoodsCollectDelAdapter;

#[async_trait]
impl GoodsCollectDelPort for GoodsCollectDelAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 单个
    /// * `desc`: `根据单个ID` - `单个删除记录`
    async fn single_soft_del_record(&self, uid: i64, video_id: i64, id: i64) -> Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 批量
    /// * `desc`: `根据批量IDs` - `批量删除记录`
    async fn batch_soft_del_record(&self, uid: i64, video_id: i64, ids: Vec<i64>) -> Result<(u64)> {
        todo!()
    }
}

//////// END
