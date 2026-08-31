// repo_adapter/src/video/hotlist/manage_port.rs
// 🔌 适配器 - ▶ 视频 - 推荐 - 管理
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::recommend::VideoRecommendInfo;
use port::cola_video::recommend::manage::VideoRecommendManagePort;

////////

/// # [ADAPTER] - hotlist manage
/// * `DESC`: `▶ 视频 - 视频推荐记录管理`
#[derive(Debug, Default, Clone)]
pub struct VideoRrecommendManageAdapter;

#[async_trait]
impl VideoRecommendManagePort for VideoRrecommendManageAdapter {
    //

    ////////

    /// # [ADAPTER] - 管理员列表
    async fn admin_get_recommends_infos(
        &self,
        uid: i64,                // UID
        user_id: Option<i64>,    // 用户 ID
        video_id: Option<i64>,   // 视频 ID
        start_time: Option<i64>, // 开始时间
        end_time: Option<i64>,   // 结束时间
        status_code: i16,        // 状态码
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> Result<(Vec<VideoRecommendInfo>, u64)> {
        todo!()
    }
}

//////// END
