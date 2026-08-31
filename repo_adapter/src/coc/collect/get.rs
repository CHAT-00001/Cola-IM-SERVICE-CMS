// repo_adapter/src/video/collect/get.rs
// 🔌 适配器 - ▶ 可乐视频 -  收藏 -  获取
// 2026/8/9 20:42 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::collect::get::VideoCollectGetPort;

////////

/// # [GET ADAPTER] - 获取
/// * `desc`: `▶ 可乐视频 - 收藏记录查询适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoCollectGetAdapter;

#[async_trait]
impl VideoCollectGetPort for VideoCollectGetAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 用户的
    /// * `desc`: `根据用户ID` - `批量获取视频IDs`
    async fn get_video_ids_by_user_id(
        &self,
        uid: i64,
        user_id: i64, // 用户 ID
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<i64>)> {
        todo!()
    }
}

//////// END
