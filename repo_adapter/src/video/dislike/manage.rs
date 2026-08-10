// repo_adapter/src/video/dislike/manage.rs
// 🔌 适配器 - 视频 - 不喜欢 - 管理 服务
// 2026/8/9 22:07 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::cola_video::dislike::manage::VideoDislikeManagePort;

////////

/// # [MANAGE ADAPTER] - dislike manage
#[derive(Debug, Default, Clone)]
pub struct VideoDislikeManageAdapter;

// 构造实现
#[async_trait]
impl VideoDislikeManagePort for VideoDislikeManageAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 管理员列表
    async fn admin_get_dislikes_infos(
        &self,
        uid: i64,
        user_id: Option<i64>,
        video_id: Option<i64>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        status_code: i16,
        limit: i64,
        offset: i64,
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }
    // TODO:
}

//////// END
