// repository/src/video/pg/comment/add.rs -- VIDEO - PG - 评论 - 发布仓储
// 2026/6/8 16:55

////////

use crate::pg_pool;
use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::entity::comment::{VIDEO_COMMENT_COLUMNS, VideoCommentEntity};
use cola_data::common::kits::snow::next_id;
use sqlx::{self, Postgres, QueryBuilder};
use tracing::error;

////////

/// # [ADD REPOSITORY] - 视频评论发布仓储
/// * `desc`: `VIDEO - Video Comment Add Repository.`
pub struct VideoCommentAddRepo;

impl VideoCommentAddRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 保存视频评论
    /// * `user_id` 用户 ID
    pub async fn save_comment(
        uid: i64,            // 当前操作用户 ID（服务端注入）
        visibility: i16,     // 可见性（服务端风控计算得出）
        cmd: CommentCommand, // 评论命令（包含 video_id, parent_id, content 等）
    ) -> Result<VideoCommentEntity, sqlx::Error> {
        let pool = pg_pool();
        let comment_id = next_id(1);
        let video_id = cmd.video_id;
        let parent_id = cmd.parent_id;
        let client_id_present = cmd._id.is_some();

        // 1. 获取当前时间戳和时间对象
        let now = chrono::Utc::now();
        let now_timestamp = now.timestamp();

        let query = format!(
            r#"
            INSERT INTO cola_video.comments (
                comment_id, _id, user_id, video_id, parent_id, message_type,
                content, media_ids, photos_url, video_url, voice_url,
                likes, dislikes, collects, reply, visibility, status,
                add_time, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, 0, 0, 0, 0, $12, 1, $13, $14, $15)
            RETURNING {}
            "#,
            VIDEO_COMMENT_COLUMNS
        );

        let result = sqlx::query_as::<_, VideoCommentEntity>(&query)
            .bind(comment_id)
            .bind(cmd._id) // 客户端 UUID v4
            .bind(uid)
            .bind(video_id)
            .bind(parent_id)
            .bind(cmd.message_type)
            .bind(cmd.content) // 简介字段安全入库
            .bind(cmd.media_ids)
            .bind(cmd.photos_url)
            .bind(cmd.video_url)
            .bind(cmd.voice_url)
            .bind(visibility) // 风控计算后的可见性状态
            .bind(now_timestamp) // 对应 $13: add_time (i64)
            .bind(now) // 对应 $14: created_at (DateTime<Utc>)
            .bind(now) // 对应 $15: updated_at (DateTime<Utc>)
            .fetch_one(&pool)
            .await;

        if let Err(database_error) = &result {
            error!(
                "[🤐 REPOSITORY] - ❌️ 保存视频评论失败: error={database_error:?}, uid={uid}, video_id={video_id}, parent_id={parent_id:?}, client_id_present={client_id_present}, visibility={visibility}, sql={query}"
            );
        }

        result
    }

    ////////

    /// # 2. [REPOSITORY] - 保存视频评论
    /// * `user_id` 用户 ID
    pub async fn update_comment(
        user_id: i64,        // 用户 ID
        comment_id: i64,     // 评论 ID
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
}

//////// END
