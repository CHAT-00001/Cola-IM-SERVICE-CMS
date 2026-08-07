// /basic.rs
// 
// 2026/8/4 00:01 Created.

////////


use crate::pg_pool;
use cola_data::cola_music::command::music::new::MusicCommand;
use cola_data::cola_music::entity::music::{MusicEntity, MUSIC_COLUMNS};
use sqlx;

////////


pub struct MusicRepo;

impl MusicRepo {
    pub async fn save_music_by_uid(
        uid: i64,
        cmd: MusicCommand,
        visibility: i16,
    ) -> Result<MusicEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "INSERT INTO cola_music.cola_music (author, title, description, cover_url, href, duration, status) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING {}",
            MUSIC_COLUMNS
        );
        sqlx::query_as::<_, MusicEntity>(&query)
            .bind(uid)
            .bind(cmd.name)
            .bind(cmd.description)
            .bind(cmd.cover_url)
            .bind(cmd.sync_id)
            .bind(cmd.duration)
            .bind(1i16)
            .fetch_one(&pool)
            .await
    }

    pub async fn find_new_list(limit: i64, offset: i64) -> Result<Vec<MusicEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_music.cola_music WHERE status = 1 ORDER BY add_time DESC LIMIT $1 OFFSET $2",
            MUSIC_COLUMNS
        );
        sqlx::query_as::<_, MusicEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    pub async fn find_hot_list(limit: i64, offset: i64) -> Result<Vec<MusicEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_music.cola_music WHERE status = 1 ORDER BY use_nums DESC, likes DESC LIMIT $1 OFFSET $2",
            MUSIC_COLUMNS
        );
        sqlx::query_as::<_, MusicEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }
}
