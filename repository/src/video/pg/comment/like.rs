// repository/src/video/pg/comment/like.rs -- VIDEO - PG - 评论 - 点赞仓储
// 2026/6/8 16:55 Created.

////////

use crate::pg_pool;
use cola_data::common::kits::snow::next_id;
use sqlx::{self, Postgres, QueryBuilder};

////////

/// # [LIKE REPOSITORY] - 视频评论点赞仓储
/// * `DESC`: `VIDEO - Comment Like Repository.`
pub struct CommentLikeRepo;

impl CommentLikeRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 更新评论点赞（幂等）
    pub async fn update_comment_like_by_id(
        uid: i64,
        comment_id: i64,
        is_liked: bool,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        if is_liked {
            let like_id = next_id(1);
            sqlx::query(
                r#"
            INSERT INTO cola_video.comments_like
                (id, user_id, comment_id, video_id, touid, add_time, created_at, updated_at)
            SELECT
                $1,
                $2,
                $3,
                comments.video_id,
                comments.user_id,
                EXTRACT(EPOCH FROM NOW())::BIGINT,
                NOW(),
                NOW()
            FROM cola_video.comments AS comments
            WHERE comments.id = $3
            ON CONFLICT (user_id, comment_id)
            DO UPDATE SET
                video_id = EXCLUDED.video_id,
                touid = EXCLUDED.touid,
                add_time = EXCLUDED.add_time,
                created_at = EXCLUDED.created_at,
                updated_at = EXCLUDED.updated_at
            "#,
            )
            .bind(like_id)
            .bind(uid)
            .bind(comment_id)
            .execute(&pool)
            .await?;
        } else {
            sqlx::query(
                r#"
            DELETE FROM cola_video.comments_like
            WHERE user_id = $1 AND comment_id = $2
            "#,
            )
            .bind(uid)
            .bind(comment_id)
            .execute(&pool)
            .await?;
        }

        Ok(())
    }

    ////////

    /// # 2. [REPOSITORY] - 查询评论点赞状态
    /// * `desc`: `根据用户和评论查询有效点赞关系`
    pub async fn check_comment_like_state(uid: i64, comment_id: i64) -> Result<bool, sqlx::Error> {
        let pool = pg_pool();
        let exists: Option<i64> = sqlx::query_scalar(
            r#"
            SELECT 1
            FROM cola_video.comments_like
            WHERE user_id = $1 AND comment_id = $2
            LIMIT 1
            "#,
        )
        .bind(uid)
        .bind(comment_id)
        .fetch_optional(&pool)
        .await?;
        Ok(exists.is_some())
    }

    ////////

    /// # 2. [REPOSITORY] - 更新不喜欢
    pub async fn update_comment_dislike_by_id(
        uid: Option<i64>,
        comment_id: i64,
        is_unliked: bool,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        if is_unliked {
            sqlx::query(
                r#"
            INSERT INTO cola_video.comments_dislike (user_id, comment_id, created_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT DO NOTHING
            "#,
            )
            .bind(uid)
            .bind(comment_id)
            .execute(&pool)
            .await?;
        } else {
            sqlx::query(
                r#"
            DELETE FROM cola_video.comments_dislike
            WHERE user_id = $1 AND comment_id = $2
            "#,
            )
            .bind(uid)
            .bind(comment_id)
            .execute(&pool)
            .await?;
        }

        Ok(())
    }
}

//////// END
