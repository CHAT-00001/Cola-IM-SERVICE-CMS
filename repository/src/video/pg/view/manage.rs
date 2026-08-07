// repository/src/video/pg/view/manage.rs
// 仓储 - VIDEO - pg - view - manage 管理
// 2026/8/2 13:19 Created.

////////

use crate::pg_pool;
use cola_data::video::entity::view::{VIDEO_VIEW_COLUMNS, VideoViewEntity};
use sqlx::{self, Postgres};

////////

/// # [MANAGE REPOSITORY] - 管理 仓储
pub struct VideoViewManageRepo;

impl VideoViewManageRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 用户软删除浏览记录 - 单条删除 (uid + video_id)
    pub async fn pg_soft_delete_single(uid: i64, video_id: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let query = "
            UPDATE cola_video.view_history
            SET is_tombstone = 1, is_deleted = 1
            WHERE uid = $1 AND video_id = $2
        ";

        let result = sqlx::query(query)
            .bind(uid)
            .bind(video_id)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 2. [REPOSITORY] - 用户软删除浏览记录 - 按时间批量删除 (早于某个时间戳的所有记录)
    pub async fn pg_soft_delete_by_time_batch(
        uid: i64,
        before_time: i64,
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let query = "
            UPDATE cola_video.view_history
            SET is_deleted = 1
            WHERE uid = $1 AND addtime <= $2 AND is_deleted = 0
        ";

        let result = sqlx::query(query)
            .bind(uid)
            .bind(before_time)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 5. [REPOSITORY] - 用户软删除浏览记录 - 全部删除
    pub async fn pg_soft_delete_all_by_uid(uid: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let query = "
            UPDATE cola_video.view_history
            SET is_deleted = 1
            WHERE uid = $1 AND is_deleted = 0
        ";

        let result = sqlx::query(query).bind(uid).execute(&pool).await?;

        Ok(result.rows_affected())
    }
}

//////// END
