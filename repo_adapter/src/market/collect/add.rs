// repo_adapter/src/market/collect/add.rs
// 🔌 适配器 - MARKET - 商品收藏 - 发布
// 2026/8/9 20:35 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::collect::add::GoodsCollectAddPort;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `商品收藏发布适配器`
pub struct GoodsCollectAddAdapter;

// 构造实现
#[async_trait]
impl GoodsCollectAddPort for GoodsCollectAddAdapter {
    //

    ////////

    /// # 1. [SERVICE] - 收藏
    /// * `desc`: `用户收藏视频`
    async fn save_collect_record(
        &self,
        _uid: i64,      // 操作者 ID
        _video_id: i64, // 视频 ID
    ) -> Result<()> {
        Ok(())
    }

    ////////

    /// # 1. [SERVICE] - 收藏
    /// * `desc`: `用户收藏视频`
    async fn edit_collect_record(
        &self,
        uid: i64,         // 操作者 ID
        video_id: i64,    // 视频 ID
        is_unliked: bool, // 是否收藏
    ) -> Result<()> {
        todo!()
    }

    ////////

    /// # 2. [SERVICE] - 收藏
    /// * `desc`: `用户收藏视频`
    async fn del_collect_record(
        &self,
        uid: i64,         // 操作者 ID
        video_id: i64,    // 视频 ID
        is_unliked: bool, // 是否收藏
    ) -> Result<()> {
        todo!()
    }

    ////////

    /// # 3. [SERVICE] - 收藏
    /// * `desc`: `用户收藏视频`
    async fn get_collect_ids_by_user_id(
        &self,
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> Result<(Vec<i64>)> {
        todo!()
    }
}
//////// END
