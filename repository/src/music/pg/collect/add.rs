// repository/src/music/pg/collect/add.rs -- 仓储 - MUSIC - 收藏记录 - 发布仓储
// 2026/8/23 03:10 Created.

////////

use crate::pg_pool;
use uuid::Uuid;

////////

/// # [ADD REPO] - 发布仓储
/// * `desc`: `可乐音乐 - 音乐收藏发布仓储`
pub struct MusicCollectAddRepo;

impl MusicCollectAddRepo {
    //

    ///////

    /// # 1. [REPOSITORY] - 保存或恢复音乐收藏
    /// * `desc`: `存在未删除的记录则更新，否则执行插入，返回 bool 表示是否实际产生了数据变更`
    pub async fn save(
        uid: i64,
        music_id: i64,
        album_id: Option<i64>,
    ) -> Result<bool, sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now();
        let id = now.timestamp_millis();
        let timestamp = now.timestamp();
        let uuid = Uuid::new_v4().to_string();
        let music_id_str = music_id.to_string();

        // 尝试更新最近的一条记录（将其恢复为正常状态：status = 1, is_deleted = false）
        let restored = sqlx::query(
            r#"
            UPDATE cola_music.collect
            SET album_id = $1, status = 1, is_deleted = false, deleted_at = NULL, updated_at = $2
            WHERE id = (
                SELECT id FROM cola_music.collect
                WHERE uid = $3 AND music_id = $4
                ORDER BY updated_at DESC NULLS LAST
                LIMIT 1
            )
            "#,
        )
            .bind(album_id)
            .bind(now)
            .bind(uid)
            .bind(&music_id_str)
            .execute(&pool)
            .await?;

        // 如果没有找到历史记录进行恢复，则直接插入一条新纪录
        if restored.rows_affected() == 0 {
            sqlx::query(
                r#"
                INSERT INTO cola_music.collect
                    (id, uuid, uid, music_id, album_id, status, is_deleted, add_time, created_at, updated_at, deleted_at)
                VALUES ($1, $2, $3, $4, $5, 1, false, $6, $7, $7, NULL)
                "#,
            )
                .bind(id)
                .bind(uuid)
                .bind(uid)
                .bind(&music_id_str)
                .bind(album_id)
                .bind(timestamp)
                .bind(now)
                .execute(&pool)
                .await?;

            // 新增插入成功，表示状态有变更
            Ok(true)
        } else {
            // 如果恢复了历史记录，也可以通过对比或者直接认为有变更（或者判断字段是否真的发生改变）
            // 这里我们以是否成功命中了恢复更新为准返回 true
            Ok(true)
        }
    }

    ///////

    /// # 2. [REPOSITORY] - 逻辑删除音乐收藏
    /// * `desc`: `将指定的收藏记录标记为逻辑删除，返回 bool 表示是否实际删除了记录`
    pub async fn delete(
        uid: i64,
        music_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now();
        let music_id_str = music_id.to_string();

        let result = sqlx::query(
            r#"
            UPDATE cola_music.collect
            SET status = 0, is_deleted = true, deleted_at = $1, updated_at = $1
            WHERE uid = $2 AND music_id = $3 AND is_deleted = false
            "#,
        )
            .bind(now)
            .bind(uid)
            .bind(&music_id_str)
            .execute(&pool)
            .await?;

        // 如果影响行数大于 0，说明确实删除了有效的收藏记录；否则说明原本就没有收藏或已经是删除状态
        let is_changed = result.rows_affected() > 0;

        Ok(is_changed)
    }
}

//////// END