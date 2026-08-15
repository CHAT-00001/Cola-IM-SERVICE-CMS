// repository/src/video/pg/file/add.rs
// 仓储 - VIDEO - pg - file - 发布
// 2026/6/8 16:55

////////

use crate::pg_pool;
use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::entity::comment::{VIDEO_COMMENT_COLUMNS, VideoCommentEntity};
use sqlx::{self, Postgres, QueryBuilder};

////////

/// # [ADD REPOSITORY] - 评论 发布
/// * `desc`: `用户发布新评论`
pub struct VideoCommentAddRepo;

impl VideoCommentAddRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 保存视频评论
    /// * `user_id` 用户 ID
    pub async fn save_comment(
        user_id: i64,        // 用户 ID（服务端从 Token 解析，不属于前端直接提交的纯 Command）
        visibility: i16,     // 可见性（服务端风控计算得出）
        cmd: CommentCommand, // 评论命令（包含 video_id, parent_id, content 等）
    ) -> Result<VideoCommentEntity, sqlx::Error> {
        let pool = pg_pool();

        let query = format!(
            "INSERT INTO cola_video.comments (user_id, video_id, parent_id, content, visibility, status) \
             VALUES ($1, $2, $3, $4, $5, 1) \
             RETURNING {}",
            VIDEO_COMMENT_COLUMNS
        );

        sqlx::query_as::<_, VideoCommentEntity>(&query)
            .bind(user_id)
            .bind(cmd.video_id) // 从 Command 中获取视频 ID
            .bind(cmd.parent_id) // 从 Command 中获取父级评论 ID
            .bind(cmd.content) // 简介字段安全入库
            .bind(visibility) // 风控计算后的可见性状态
            .fetch_one(&pool)
            .await
    }

    ////////
    // 修改评论
}

//////// END
