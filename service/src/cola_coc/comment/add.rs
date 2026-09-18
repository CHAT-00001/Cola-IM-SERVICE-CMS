// servicey/src/cola_coc/comment/add.rs -- 服务 - COC - COMMENT - ADD
// 2026/8/12 05:39 Created.

////////

use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::info::comment::CommentInfo; // 👈 这里引入 Info 模型
use repository::video::pg::comment::add::VideoCommentAddRepo;
use repository::video::pg::comment::check::VideoCommentCheckRepo;
use tracing::{error, info};

////////

/// # [ADD SERVICE] - 发布
pub struct VideoCommentAddService;

impl VideoCommentAddService {
    pub async fn create_comment(
        user_id: i64,
        visibility: i16,
        cmd: CommentCommand,
    ) -> Result<CommentInfo, anyhow::Error> {
        // 1. 如果有父级评论 ID，校验父级评论是否存在
        if let Some(parent_id) = cmd.parent_id {
            info!("Service: 校验父级评论是否存在, parent_id: {}", parent_id);

            let exists = VideoCommentCheckRepo::exists_active(parent_id).await?;
            if !exists {
                error!(
                    "Service: 父级评论不存在或已被删除, parent_id: {}",
                    parent_id
                );
                return Err(anyhow::anyhow!(
                    "Parent file does not exist or has been deleted"
                ));
            }
        }

        // 2. 校验通过，执行保存 (返回的是 VideoCommentEntity)
        let comment_entity = VideoCommentAddRepo::save_comment(user_id, visibility, cmd).await?;

        info!("Service: 评论发布成功, comment_id: {}", comment_entity.id);

        // 3. 使用 data 层写好的 from_entity 转换为 CommentInfo
        let comment_info = CommentInfo::from_entity(comment_entity);

        Ok(comment_info)
    }
}

//////// END
