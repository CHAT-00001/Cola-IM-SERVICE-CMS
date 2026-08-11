// repo_adapter/src/cola_video/view/del.rs
// 🔌 插头 - VIDEO - 浏览 - 删除服务
// 2026/8/6 19:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::view::del::GoodsViewDelPort;

////////

/// # [DELETE ADAPTER] - 删除
/// * `desc`: `视频浏览删除适配器`
pub struct GoodsViewDeleteAdapter;

// 构造实现
#[async_trait]
impl GoodsViewDelPort for GoodsViewDeleteAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 单个删除
    async fn single_delete(&self, view_id: i64) -> Result<(u16)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 批量删除
    async fn batch_delete(&self, view_ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }

    ////////

    /// # 3. [ADAPTER] - 用户删除时
    async fn delete_view_by_user_id(&self, user_id: i64) -> Result<(u64)> {
        todo!()
    }

    ////////

    /// # 4. [ADAPTER] - 视频删除时
    async fn delete_view_by_video_id(&self, video_id: i64) -> Result<(u64)> {
        todo!()
    }
}

//////// END
