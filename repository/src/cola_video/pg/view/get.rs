// repository/src/cola_video/pg/get.rs
// 仓储 - VIDEO - pg - view - get 获取
// 2026/5/20 10:20

////////

use crate::pg_pool;
use cola_data::cola_video::entity::view::{VideoViewEntity, VIDEO_VIEW_COLUMNS};
use sqlx::{self, Postgres};

////////

/// # [REPOSITORY] - 获取 仓储
pub struct VideoViewGetRepo;

impl VideoViewGetRepo {
    //

    ////////

    /// # 2. [REPOSITORY] - 提取用户浏览记录的 video_ids (带分页)
    /// * 上层动态查询历史视频时使用
    pub async fn pg_find_video_ids_by_uid(
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();
        let query = "
            SELECT video_id
            FROM cola_video.view_history
            WHERE uid = $1 AND is_deleted = 0 AND status = 1
            ORDER BY addtime DESC
            LIMIT $2 OFFSET $3
        ";

        sqlx::query_scalar::<_, i64>(query)
            .bind(uid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - 用户软删除浏览记录 - 单条删除 (uid + video_id)
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

    /// # 4. [REPOSITORY] - 用户软删除浏览记录 - 按时间批量删除 (早于某个时间戳的所有记录)
    pub async fn pg_soft_delete_by_time_batch(uid: i64, before_time: i64) -> Result<u64, sqlx::Error> {
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

        let result = sqlx::query(query)
            .bind(uid)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 6. [REPOSITORY] - 定时任务 - 物理删除过期的历史记录
    /// * 删除早于指定时间戳（如过期阈值）的记录
    pub async fn pg_purge_expired_history(expire_before_time: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let query = "
            DELETE FROM cola_video.view_history
            WHERE addtime < $1
        ";

        let result = sqlx::query(query)
            .bind(expire_before_time)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }
}

//////// END