// repository/src/cola_video/pg/file/del.rs
// 🛢 仓储 - ▶ 可乐视频 - pg - 评论 - 软删除
// 2026/8/9 00:50 Created.

////////

use crate::pg_pool;
use cola_data::cola_video::entity::comment::VIDEO_COMMENT_COLUMNS;
use sqlx::{self, Postgres};

////////

/// # [MANAGE REPOSITORY] - 软删除
/// * `desc`: `▶ 可乐视频 - 👤 评论软删除仓储`
pub struct VideoCommentDelRepo;

// 构造实现
impl VideoCommentDelRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 视频的
    /// * `uid`: 用户 ID
    /// * `condition`: `▶ 视频被删除时 - 🔄 同步删除TA的评论记录`
    pub async fn soft_delete_comments_by_video_id(
        video_id: i64, // 视频 ID
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now();
        let datetime = now.naive_utc();

        let query = format!(
            r#"
            UPDATE cola_video.file
            SET is_deleted = true,
                deleted_at = $1,
                updated_at = $1
            WHERE video_id = $2 AND is_deleted = false
            RETURNING {}
            "#,
            VIDEO_COMMENT_COLUMNS
        );

        let result = sqlx::query(&query)
            .bind(datetime)
            .bind(video_id)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 2. [REPOSITORY] - 用户的
    /// * `uid`: 用户 ID
    /// * `condition`: `🗣 用户被删除时 - 🔄 同步删除TA的评论记录`
    pub async fn soft_delete_comments_by_user_id(
        user_id: i64, // 用户 ID
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now();
        let datetime = now.naive_utc();

        let query = format!(
            r#"
            UPDATE cola_video.file
            SET is_deleted = true,
                deleted_at = $1,
                updated_at = $1
            WHERE uid = $2 AND is_deleted = false
            RETURNING {}
            "#,
            VIDEO_COMMENT_COLUMNS
        );

        let result = sqlx::query(&query)
            .bind(datetime)
            .bind(user_id)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 3. [REPOSITORY] - 单条软删除
    /// * `desc`: `单条软删除评论记录`
    pub async fn soft_delete_comments_by_id(comment_id: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now();
        let datetime = now.naive_utc();

        let query = format!(
            r#"
            UPDATE cola_video.file
            SET is_deleted = true,
                deleted_at = $1,
                updated_at = $1
            WHERE id = $2 AND is_deleted = false
            RETURNING {}
            "#,
            VIDEO_COMMENT_COLUMNS
        );

        let result = sqlx::query(&query)
            .bind(datetime)
            .bind(comment_id)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 4. [REPOSITORY] - 批量软删除
    /// * `desc`: `批量软删除评论记录`
    pub async fn batch_soft_delete_comment_by_ids(
        comment_ids: &[i64], // 评论 IDs
    ) -> Result<u64, sqlx::Error> {
        if comment_ids.is_empty() {
            return Ok(0);
        }

        let pool = pg_pool();
        let now = chrono::Utc::now();
        let datetime = now.naive_utc();

        let query = format!(
            r#"
            UPDATE cola_video.file
            SET is_deleted = true,
                deleted_at = $1,
                updated_at = $1
            WHERE id = ANY($2) AND is_deleted = false
            RETURNING {}
            "#,
            VIDEO_COMMENT_COLUMNS
        );

        let result = sqlx::query(&query)
            .bind(datetime)
            .bind(comment_ids)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }
}

//////// END
