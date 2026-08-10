// repo_adapter/src/video/collect/add.rs
// 🔌 插头 - ▶ 可乐视频 - 收藏 - 发布
// 2026/8/9 20:35 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::collect::add::VideoCollectAddPort;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `▶可乐视频 - 收藏发布服务`
pub struct VideoCollectAddAdapter;

// 构造实现
#[async_trait]
impl VideoCollectAddPort for VideoCollectAddAdapter {
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
