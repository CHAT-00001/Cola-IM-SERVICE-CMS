// repo_adapter/src/gis/comment.rs
// 2026-07-07

use async_trait::async_trait;
use cola_data::gis::port::comment::CommentRepo;
use cola_data::gis::command::comment::PoiCommentCommand;
use cola_data::gis::info::comment::PoiCommentInfo;
use repo::gis::service::poi_comment::PoiCommentService;

pub struct CommentPortAdapter;

#[async_trait]
impl CommentRepo for CommentPortAdapter {

    /// # 1. [PORT] - 保存评论记录 + 更新评论数量
    async fn save_comment_record(
        &self,
        uid: i64,
        poi_id: i64,
        cmd: PoiCommentCommand,
    ) -> anyhow::Result<PoiCommentInfo> {
        PoiCommentService::save_comment_and_update_count(uid, poi_id, cmd, 1).await
    }

    /// # 2. [PORT] - 编辑评论
    async fn edit_comment_record(
        &self,
        _comment_id: i64,
        _cmd: PoiCommentCommand,
    ) -> anyhow::Result<PoiCommentInfo> {
        Err(anyhow::anyhow!("edit_comment_record not implemented"))
    }

    /// # 3. [PORT] - 删除评论 + 更新评论数量
    async fn del_comment_record(
        &self,
        comment_id: i64,
    ) -> anyhow::Result<()> {
        PoiCommentService::delete_comment_and_update_count(0, comment_id).await?;
        Ok(())
    }

    /// # 4. [PORT] - 批量删除评论
    async fn del_comments_record(
        &self,
        comment_ids: Vec<i64>,
    ) -> anyhow::Result<()> {
        for id in comment_ids {
            PoiCommentService::delete_comment_and_update_count(0, id).await?;
        }
        Ok(())
    }
}