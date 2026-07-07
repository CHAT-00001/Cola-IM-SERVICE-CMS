// cola_data/src/gis/port/comment.rs  -- 数据中心 - GIS - port - 兴趣点评论
// 2026/7/7

//////

use crate::gis::command::comment::PoiCommentCommand;
use crate::gis::info::comment::PoiCommentInfo;

//////

/// # [SERVICE PORT] - 评论
#[async_trait::async_trait]
pub trait CommentRepo: Send + Sync {

    ////////

    /// # [PORT] - 保存评论记录
    async fn save_comment_record(
        &self,
        uid: i64,
        poi_id: i64,
        cmd: PoiCommentCommand,
    ) -> anyhow::Result<PoiCommentInfo>;

    ////////

    /// # [PORT] - 编辑评论
    async fn edit_comment_record(
        &self,
        comment_id: i64,
        cmd: PoiCommentCommand,
    ) -> anyhow::Result<PoiCommentInfo>;

    ////////

    /// # [PORT] - 删除评论
    async fn del_comment_record(
        &self,
        comment_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 批量删除评论
    async fn del_comments_record(
        &self,
        comment_ids: Vec<i64>,
    ) -> anyhow::Result<()>;
}