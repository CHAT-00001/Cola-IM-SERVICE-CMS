// repo_adapter/src/cola_video/recommend/add.rs
// 🔌 适配器 - ▶ 视频 - 推荐 - 发布
// 2026/8/6 18:59 Created.

////////



use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::info::recommend::VideoRecommendInfo;
use port::cola_video::recommend::add::VideoRecommendAddPort;

////////

/// # [ADD ADAPTER] - recommend add
/// * `desc`: `▶ 视频 - 推荐记录发布适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoRecommendAddAdapter;

#[async_trait]
impl VideoRecommendAddPort for VideoRecommendAddAdapter {
    //

    ////////

    /// #  1. [ADAPTER] - 发布推荐
    async fn send_recommend(
        &self,
        uid: i64,
        video_id: i64,
        cmd: CommentCommand,
    ) -> Result<(VideoRecommendInfo)> {
        todo!()
    }

    ////////

    /// #  2. [ADAPTER] - 发布推荐
    async fn edit_old_record(
        &self,
        uid: i64,
        recommend_id: i64,
        cmd: CommentCommand,
    ) -> Result<(VideoRecommendInfo)> {
        todo!()
    }
}

//////// END
