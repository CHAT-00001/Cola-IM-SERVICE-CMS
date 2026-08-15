// repository/src/new/pg/file/like.rs
// 仓储 - VIDEO - pg - file - 点赞/不喜欢
// 2026/6/8 16:55 Created.

////////

use crate::pg_pool;
use sqlx::{self, Postgres, QueryBuilder};

////////


/// # [LIKE REPOSITORY] - 评论 点赞/不喜欢
pub struct CommentLikeRepo;

impl CommentLikeRepo {


    /// # 1. [REPOSITORY] - 更新评论点赞（幂等）
    pub async fn update_comment_like_by_id(
        uid: i64,
        comment_id: i64,
        is_liked: bool,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        if is_liked {
            sqlx::query(
                r#"
            INSERT INTO video_comments_like (uid, comment_id, created_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT (uid, comment_id)
            DO NOTHING
            "#,
            )
                .bind(uid)
                .bind(comment_id)
                .execute(&pool)
                .await?;
        } else {
            sqlx::query(
                r#"
            DELETE FROM video_comments_like
            WHERE uid = $1 AND comment_id = $2
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
            INSERT INTO cola_video.comments_dislike (uid, comment_id, created_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT (uid, comment_id)
            DO NOTHING
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
            WHERE uid = $1 AND comment_id = $2
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
