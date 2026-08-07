// repository/src/video/pg/like/active
// 仓储 - VIDEO - pg - like - add 点赞/不喜欢
// 2026/8/2 14:49 Created.

////////

use crate::pg_pool;
use sqlx::{self, Postgres};

////////

/// [ADD REPOSITORY] - 视频 点赞/不喜欢 添加 仓储
pub struct VideoLikeAddRepo;

impl VideoLikeAddRepo {
    ////////

    /// # 1. [REPOSITORY] - 保存或更新点赞记录 (通过 status 识别: 1有效 0失效)
    pub async fn pg_save_video_like(
        uid: i64,
        video_id: i64,
        status: i16,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now();
        let timestamp = now.timestamp();
        let datetime = now.naive_utc();

        let query = if status == 1 {
            r#"
                INSERT INTO cola_video.like (uid, video_id, status, is_deleted, deleted_at, add_time, created_at, updated_at)
                VALUES ($1, $2, 1, false, NULL, $3, $4, $4)
                ON CONFLICT (uid, video_id)
                DO UPDATE SET
                    status = 1,
                    is_deleted = false,
                    deleted_at = NULL,
                    updated_at = EXCLUDED.updated_at
            "#
        } else {
            r#"
                INSERT INTO cola_video.like (uid, video_id, status, is_deleted, deleted_at, add_time, created_at, updated_at)
                VALUES ($1, $2, 0, true, $4, $3, $4, $4)
                ON CONFLICT (uid, video_id)
                DO UPDATE SET
                    status = 0,
                    is_deleted = true,
                    deleted_at = EXCLUDED.deleted_at,
                    updated_at = EXCLUDED.updated_at
            "#
        };

        sqlx::query(query)
            .bind(uid)
            .bind(video_id)
            .bind(timestamp)
            .bind(datetime)
            .execute(&pool)
            .await?;

        Ok(())
    }

    ////////

    /// # 2. [REPOSITORY] - 保存或更新不喜欢记录 (通过 status 识别: 1有效 0失效)
    pub async fn pg_save_video_unlike(
        uid: i64,
        video_id: i64,
        status: i16,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now();
        let timestamp = now.timestamp();
        let datetime = now.naive_utc();

        let query = if status == 1 {
            r#"
                INSERT INTO cola_video.dislike (uid, video_id, status, is_deleted, deleted_at, add_time, created_at, updated_at)
                VALUES ($1, $2, 1, false, NULL, $3, $4, $4)
                ON CONFLICT (uid, video_id)
                DO UPDATE SET
                    status = 1,
                    is_deleted = false,
                    deleted_at = NULL,
                    updated_at = EXCLUDED.updated_at
            "#
        } else {
            r#"
                INSERT INTO cola_video.dislike (uid, video_id, status, is_deleted, deleted_at, add_time, created_at, updated_at)
                VALUES ($1, $2, 0, true, $4, $3, $4, $4)
                ON CONFLICT (uid, video_id)
                DO UPDATE SET
                    status = 0,
                    is_deleted = true,
                    deleted_at = EXCLUDED.deleted_at,
                    updated_at = EXCLUDED.updated_at
            "#
        };

        sqlx::query(query)
            .bind(uid)
            .bind(video_id)
            .bind(timestamp)
            .bind(datetime)
            .execute(&pool)
            .await?;

        Ok(())
    }
}

//////// END