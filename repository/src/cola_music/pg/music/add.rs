// repository/src/cola_music/pg/cola_music/add.rs
// 仓储 - MUSIC - pg - 音乐 发布
// 2026/8/3 20:35 Created.

////////

use crate::pg_pool;
use cola_data::cola_music::command::music::new::MusicCommand;
use cola_data::cola_music::entity::music::{MUSIC_COLUMNS, MusicEntity};
use sqlx;

////////

/// [ADD REPOSITORY] - 发布
pub struct MusicAddRepo;

impl MusicAddRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 保存
    pub async fn pg_save_music_by_uid(
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

    ////////

    /// # 2. [REPOSITORY] - 编辑
    pub async fn pg_update_music_by_id() {}
}

//////// END
