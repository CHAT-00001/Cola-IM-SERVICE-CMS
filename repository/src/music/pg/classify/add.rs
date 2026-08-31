// repository/src/music/pg/classify/add.rs -- 仓储 - MUSIC - PG - 分类 - 发布仓储
// 2026/8/3 20:40 Created.

////////

use crate::pg_pool;
use cola_data::music::command::music::new::{MusicCreateCommand, MusicUpdateCommand};
use cola_data::music::entity::music::{MUSIC_COLUMNS, MusicEntity};
use sqlx;

////////

/// # [ADD REPOSITORY] - 发布仓储
/// * `desc`: `可乐音乐 - 专辑创建/修改 仓储`
pub struct MusicClassifyAddRepo;

impl MusicClassifyAddRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 保存
    pub async fn save_music_by_uid(
        uid: i64,
        cmd: MusicCreateCommand, // 音乐分类创建命令
        visibility: i16,         // 可见度
    ) -> Result<MusicEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "INSERT INTO cola_cola_music.music (author, title, description, cover_url, href, duration, status) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING {}",
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
    pub async fn update_music_by_id(
        uid: i64,
        cmd: MusicUpdateCommand, // 音乐更新命令
        visibility: i16,         // 可见度
    ) -> Result<MusicEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "UPDATE INTO cola_cola_music.music (author, title, description, cover_url, href, duration, status) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING {}",
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
}

//////// END
