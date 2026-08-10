// data/src/cola_video/prot/collect/manage.rs
// 数据 - ▶ 可乐视频 - port - 收藏 - 管理
// 2026/8/5 00:04 Created.

////////

use crate::cola_video::command::comment::CommentCommand;
use crate::cola_video::info::collect::VideoCollectInfo;
use crate::cola_video::info::comment::VideoCommentInfo;

////////

/// # [MANAGE PORTS] - 管理
/// * `desc`: `▶ 可乐视频 - 收藏管理端口`
#[async_trait::async_trait]
pub trait CollectManagePort: Send + Sync {
    //

    ////////

    /// # 1. [SERVICE] - 管理员 - 视频的
    /// * `condition`: `⚠️ 管理员` - 根据视频ID - 查看收藏记录
    async fn get_collect_record_by_video_id(
        &self,
        uid: i64,      // 操作者 ID
        video_id: i64, // 视频 ID
        limit: i64,    // 数量
        offset: i64,   // 页码
    ) -> anyhow::Result<(Vec<VideoCollectInfo>)>;

    ////////

    /// # 2. [SERVICE] - 管理员 - 用户的
    /// * `condition`: `⚠️ 管理员` - 根据用户ID - 查看收藏记录
    async fn get_collect_record_by_user_id(
        &self,
        uid: i64,     // 操作者 ID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<VideoCollectInfo>)>;

    ////////

    /// # [PORT] - 编辑
    async fn edit_comment_record(
        &self,
        comment_id: i64,
        cmd: CommentCommand,
    ) -> anyhow::Result<(VideoCommentInfo)>;

    ////////

    /// # [PORT] - 删除
    async fn del_comment_record(&self, comment_id: i64) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 批量删除
    async fn del_comments_record(&self, comment_ids: Vec<i64>) -> anyhow::Result<()>;
}

//////// END
