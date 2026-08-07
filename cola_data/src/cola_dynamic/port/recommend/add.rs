// cola_video/port/recommend/add.rs
// 视频 - port - 推荐 - 列表
// 2026/8/5 00:01 Created.

////////

use crate::cola_video::command::comment::CommentCommand;
use crate::cola_video::info::comment::VideoCommentInfo;

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `推荐发布端口`
#[async_trait::async_trait]
pub trait AddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 新建
    async fn save_new_record(
        &self,
        uid: i64,            // UID
        video_id: i64,       // 视频ID
        cmd: CommentCommand, // 命令
    ) -> anyhow::Result<(VideoCommentInfo)>;

    ////////

    /// # 2. [PORT] - 编辑
    async fn edit_old_record(
        &self,
        uid: i64,            // UID
        recommend_id: i64,   // 推荐ID
        cmd: CommentCommand, // 命令
    ) -> anyhow::Result<(VideoCommentInfo)>;
}

//////// END
