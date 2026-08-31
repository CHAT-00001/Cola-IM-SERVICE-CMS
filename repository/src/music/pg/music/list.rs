// repository/src/music/pg/music/list.rs -- 仓储 - MUSIC - pg - 音乐 - 列表仓储
// 2026/8/4 00:01 Created.

////////

use crate::pg_pool;
use cola_data::music::entity::music::{MUSIC_COLUMNS, MusicEntity};
use sqlx;

////////

/// # [LIST REPOSITORY] - 音乐 列表仓储
/// * `desc`: `可乐音乐 - 音乐列表 仓储`
pub struct MusicListRepo;

impl MusicListRepo {
    //

    ////////

    /// # 1. [REPO] - 最新的
    pub async fn find_new_list(limit: i64, offset: i64) -> Result<Vec<MusicEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_music.music WHERE status = 1 ORDER BY add_time DESC LIMIT $1 OFFSET $2",
            MUSIC_COLUMNS
        );
        sqlx::query_as::<_, MusicEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 2. [REPO] - 热门
    pub async fn find_hot_list(limit: i64, offset: i64) -> Result<Vec<MusicEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_music.music WHERE status = 1 ORDER BY use_nums DESC, likes DESC LIMIT $1 OFFSET $2",
            MUSIC_COLUMNS
        );
        sqlx::query_as::<_, MusicEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }
}

//////// END