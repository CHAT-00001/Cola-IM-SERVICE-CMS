// repository/src/video/pg/view/stat.rs
// 仓储 - VIDEO - pg - 浏览 - 统计数量
// 2026/8/9 23:49 Created.

////////

use crate::pg_pool;
use sqlx::{self, Postgres};

////////

/// [STAT REPOSITORY] - 视频浏览统计 repository
/// * `desc`: `统计数量`
pub struct VideoViewStatRepo;

impl VideoViewStatRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 用户浏览的数量
    pub async fn stat_count_by_user_id(user_id: i64, // 用户 ID
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let query = "
            SELECT COUNT(*)
            FROM cola_video.view
            WHERE uid = $1 AND status = 1 AND (is_deleted = false OR is_deleted IS NULL)
        ";

        let count: i64 = sqlx::query_scalar(query)
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = ?e,
                    user_id = user_id,
                    "[🤐 REPO] - ❌️ VideoViewStatRepo::stat_count_by_user_id query failed"
                );
                e
            })?;

        Ok(count as u64)
    }

    ////////

    /// # 2. [REPOSITORY] - 视频被浏览的数量
    pub async fn stat_count_by_video_id(video_id: i64, // 视频 ID
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let query = "
            SELECT COUNT(*)
            FROM cola_video.disview
            WHERE video_id = $1 AND status = 1 AND (is_deleted = false OR is_deleted IS NULL)
        ";

        let count: i64 = sqlx::query_scalar(query)
            .bind(video_id)
            .fetch_one(&pool)
            .await
            .map_err(|e| {
                tracing::error!(
                    error = ?e,
                    video_id = video_id,
                    "[🤐 REPO] - ❌️ VideoViewStatRepo::stat_count_by_video_id query failed"
                );
                e
            })?;

        Ok(count as u64)
    }
}

//////// END
