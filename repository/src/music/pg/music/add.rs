// repository/src/music/pg/music/add.rs -- 仓储 - MUSIC - pg - 音乐 - 内容 - 发布仓储
// 2026/8/3 20:35 Created.

////////

use crate::pg_pool;
use cola_data::music::command::music::new::{MusicCreateCommand, MusicUpdateCommand};
use cola_data::music::entity::music::{MUSIC_COLUMNS, MusicEntity};
use sqlx;

////////

/// [ADD REPOSITORY] - 发布
pub struct MusicAddRepo;

impl MusicAddRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 保存音乐
    pub async fn save_music_by_uid(
        uid: i64,
        cmd: MusicCreateCommand, // 音乐创建命令
        visibility: i16,         // 可见度
    ) -> Result<MusicEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "INSERT INTO cola_music.music (author, title, description, cover_url, href, duration, status) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING {}",
            MUSIC_COLUMNS
        );
        sqlx::query_as::<_, MusicEntity>(&query)
            .bind(uid)
            .bind(cmd.name)
            .bind(cmd.description)
            .bind(cmd.cover_url)
            .bind(cmd.sync_id)
            .bind(cmd.duration)
            .bind(visibility)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 编辑音乐
    pub async fn update_music_by_id(
        uid: i64,
        music_id: i64,           // 👈 必须加上音乐 ID，用于确定更新哪一条
        cmd: MusicUpdateCommand, // 音乐更新命令
        visibility: i16,         // 可见度
    ) -> Result<MusicEntity, sqlx::Error> {
        let pool = pg_pool();

        // 修正为标准的 SQL UPDATE 语法，并加上 WHERE 条件
        let query = format!(
            "UPDATE cola_music.music
             SET title = $1, description = $2, cover_url = $3, href = $4, duration = $5, status = $6, updated_at = NOW()
             WHERE id = $7 AND author = $8
             RETURNING {}",
            MUSIC_COLUMNS
        );

        sqlx::query_as::<_, MusicEntity>(&query)
            .bind(cmd.name)
            .bind(cmd.description)
            .bind(cmd.cover_url)
            .bind(cmd.sync_id)
            .bind(cmd.duration)
            .bind(visibility)
            .bind(music_id) // $7 对应 music_id
            .bind(uid)      // $8 对应 author（即 uid），确保只能修改自己的音乐
            .fetch_one(&pool)
            .await
    }
}

//////// END
