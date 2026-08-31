// repository/src/music/pg/favorites/del.rs -- 仓储 - MUSIC - PG - 点赞 - 删除仓储
// 2026/8/23 03:10 Created.

////////

use crate::pg_pool;

////////

/// # [DELETE REPOSITORY] - 删除仓储
/// * `desc`: `MUSIC - 点赞记录 删除仓储`
pub struct MusicLikeDelRepo;

impl MusicLikeDelRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 硬删除过期最喜欢记录
    /// * `desc`: `time_range 按秒计算，清理已软删除历史`
    pub async fn hard_delete_expired(time_range: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let cutoff = chrono::Utc::now() - chrono::Duration::seconds(time_range.max(0));
        let result = sqlx::query(
            "DELETE FROM cola_music.like WHERE is_deleted = true AND deleted_at IS NOT NULL AND deleted_at < $1",
        )
        .bind(cutoff)
        .execute(&pool)
        .await?;
        Ok(result.rows_affected())
    }
}

//////// END