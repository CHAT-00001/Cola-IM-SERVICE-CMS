// repository/src/video/pg/like/clean.rs
// 仓储 - VIDEO - pg - like - clean 点赞/不喜欢
// 2026/8/2 14:49 Created.

////////

use crate::pg_pool;
use sqlx::{self, Postgres};

////////

/// [CLEAN REPOSITORY] - 视频 点赞/不喜欢 清除 仓储
pub struct VideoLikeCleanRepo;

impl VideoLikeCleanRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 定时任务：物理删除过期的点赞失效记录
    /// * 删除 `cola_video.like` 表中 `is_deleted = true` 且 `deleted_at` 距今超过 180 天的记录
    pub async fn pg_purge_expired_likes() -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        let query = "
            DELETE FROM cola_video.like
            WHERE is_deleted = true
              AND deleted_at < (NOW() - INTERVAL '180 days')
        ";

        let result = sqlx::query(query)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 2. [REPOSITORY] - 定时任务：物理删除过期的不喜欢失效记录
    /// * 删除 `cola_video.dislike` 表中 `is_deleted = true` 且 `deleted_at` 距今超过 180 天的记录
    pub async fn pg_purge_expired_dislikes() -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        let query = "
            DELETE FROM cola_video.dislike
            WHERE is_deleted = true
              AND deleted_at < (NOW() - INTERVAL '180 days')
        ";

        let result = sqlx::query(query)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }
}

//////// END