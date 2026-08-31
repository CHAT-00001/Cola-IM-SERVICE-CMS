// port/src/video/port/hotlist/add.rs
// ⏩️ 端口 -  ▶ 视频 -  推荐 - 发布
// 2026/8/5 00:01 Created.

////////

use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::info::recommend::VideoRecommendInfo;

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `推荐发布端口`
#[async_trait::async_trait]
pub trait VideoRecommendAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 新建
    async fn send_recommend(
        &self,
        uid: i64,            // UID
        video_id: i64,       // 视频 ID
        cmd: CommentCommand, // 命令
    ) -> anyhow::Result<(VideoRecommendInfo)>;

    ////////

    /// # 2. [PORT] - 编辑
    async fn edit_old_record(
        &self,
        uid: i64,            // UID
        recommend_id: i64,   // 推荐ID
        cmd: CommentCommand, // 命令
    ) -> anyhow::Result<(VideoRecommendInfo)>;
}

//////// END
