// repo_adapter/src/new/add -- 适配器 - VIDEO - Add
// 2026-06-12 14:12

////////

use async_trait::async_trait;
use cola_data::video::port::comment::CommentRepo;
use cola_data::video::command::comment::CommentCommand;
use cola_data::video::info::comment::VideoCommentInfo;
use repository::video::service::comment::CommentService;

////////

pub struct CommentPortAdapter;

#[async_trait]
impl CommentRepo for CommentPortAdapter {

    /// # 1. [PORT] - 保存评论记录 + 更新评论数量
    async fn save_comment_record(
        &self,
        uid: i64,
        video_id: i64,
        is_liked: bool,
    ) -> anyhow::Result<VideoCommentInfo> {
        // is_liked is unused; we need a placeholder cmd
        let cmd = CommentCommand::default();
        CommentService::save_comment_and_update_count(uid, video_id, cmd, 1).await
    }

    /// # 2. [PORT] - 编辑评论(预设)
    async fn edit_comment_record(
        &self,
        comment_id: i64,
        cmd: CommentCommand,
    ) -> anyhow::Result<VideoCommentInfo> {
        // TODO: actual edit; for now call delete + re-add
        CommentService::delete_comment_and_update_count(0, comment_id).await?;
        Err(anyhow::anyhow!("edit_comment_record not implemented"))
    }

    /// # 3. [PORT] - 删除评论 + 更新评论数量
    async fn del_comment_record(
        &self,
        comment_id: i64,
    ) -> anyhow::Result<()> {
        CommentService::delete_comment_and_update_count(0, comment_id).await?;
        Ok(())
    }

    async fn del_comments_record(
        &self,
        comment_ids: Vec<i64>,
    ) -> anyhow::Result<()> {
        for id in comment_ids {
            CommentService::delete_comment_and_update_count(0, id).await?;
        }
        Ok(())
    }
}

//////// END