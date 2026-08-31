// repository/src/music/pg/like/stat.rs -- 仓储 - MUSIC - PG - 点赞 - 统计仓储
// 2026/8/23 03:10 Created.

////////

use crate::pg_pool;

////////

/// # [STAT REPOSITORY] - 统计仓储
pub struct MusicLikeStatRepo;

impl MusicLikeStatRepo {
    /// # 1. [REPOSITORY] - 统计用户有效最喜欢数量
    /// * `desc`: `按用户统计有效关系`
    pub async fn count_valid_by_user_id(user_id: i64) -> Result<u64, sqlx::Error> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM cola_music.like WHERE uid = $1 AND status = 1 AND is_deleted = false",
        )
        .bind(user_id)
        .fetch_one(&pg_pool())
        .await?;
        Ok(count as u64)
    }

    /// # 2. [REPOSITORY] - 统计音乐被最喜欢数量
    /// * `desc`: `按音乐统计有效用户关系`
    pub async fn count_valid_by_music_id(music_id: i64) -> Result<u64, sqlx::Error> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM cola_music.like WHERE music_id = $1 AND status = 1 AND is_deleted = false",
        )
        .bind(music_id)
        .fetch_one(&pg_pool())
        .await?;
        Ok(count as u64)
    }
}

//////// END