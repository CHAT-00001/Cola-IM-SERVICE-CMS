// repo_adapter/src/video/recommend/list.rs
// 🔌 插头 - 可乐视频 - 推荐 - 列表
// 2026/8/6 18:59 Created.

////////

use async_trait::async_trait;
use cola_data::cola_video::info::recommend::VideoRecommendInfo;
use port::cola_video::recommend::list::VideoRecommendListPort;

////////

/// # [LIST ADAPTER] - recommend list
/// * `desc`: `▶ 视频 - 推荐记录管理服务`
#[derive(Debug, Default, Clone)]
pub struct RecommendListAdapter;

#[async_trait]
impl VideoRecommendListPort for RecommendListAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 用户的
    /// * `desc`: `根据用户ID` - `获取购买记录信息`
    async fn get_recommend_infos_user_id(
        &self,
        uid: i64,
        user_id: i64, // 用户 ID
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<VideoRecommendInfo>)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 视频的
    /// * `desc`: `根据视频ID` - `获取购买记录信息`
    async fn get_recommend_infos_video_id(
        &self,
        uid: i64,
        video_id: i64, // 视频 ID
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<VideoRecommendInfo>)> {
        todo!()
    }
}

//////// END
