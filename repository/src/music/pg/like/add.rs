// repository/src/music/pg/like/add.rs -- 仓储 - MUSIC - PG - 点赞 - 发布仓储
// 2026/8/23 03:10 Created.

////////

use crate::pg_pool;
use uuid::Uuid;

////////

/// # [ADD REPOSITORY] - 音乐点赞记录发布仓储
/// * `desc`: `COLA MUSIC - Like Add Repository.`
pub struct MusicLikeAddRepo;

impl MusicLikeAddRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 设置音乐点赞
    /// * `desc`: `新增或恢复用户与音乐的有效最喜欢关系`
    pub async fn save_like(uid: i64, music_id: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now();
        let id = now.timestamp_millis();
        let timestamp = now.timestamp();
        let uuid = Uuid::new_v4().to_string();

        // 使用双引号安全包裹关键字表名 "like"
        let restored = sqlx::query(
            r#"
            UPDATE cola_music."like"
            SET status = 1, is_deleted = false, deleted_at = NULL, upd_time = $1, updated_at = $2
            WHERE id = (
                SELECT id FROM cola_music."like"
                WHERE uid = $3 AND music_id = $4
                ORDER BY updated_at DESC NULLS LAST
                LIMIT 1
            )
            "#,
        )
            .bind(timestamp)
            .bind(now)
            .bind(uid)
            .bind(music_id)
            .execute(&pool)
            .await?;

        if restored.rows_affected() > 0 {
            return Ok(());
        }

        sqlx::query(
            r#"
            INSERT INTO cola_music."like"
                (id, uuid, uid, music_id, status, is_deleted, add_time, upd_time, created_at, updated_at, deleted_at)
            VALUES ($1, $2, $3, $4, 1, false, $5, $5, $6, $6, NULL)
            "#,
        )
            .bind(id)
            .bind(uuid)
            .bind(uid)
            .bind(music_id)
            .bind(timestamp)
            .bind(now)
            .execute(&pool)
            .await?;
        Ok(())
    }

    ////////

    /// # 2. [REPOSITORY] - 取消音乐点赞
    /// * `desc`: `逻辑删除有效关系，保留历史记录供审计`
    pub async fn un_like(uid: i64, music_id: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now();

        sqlx::query(
            r#"
            UPDATE cola_music."like"
            SET status = 0, is_deleted = true, deleted_at = $1,
                upd_time = $2, updated_at = $1
            WHERE uid = $3 AND music_id = $4 AND is_deleted = false
            "#,
        )
            .bind(now)
            .bind(now.timestamp())
            .bind(uid)
            .bind(music_id)
            .execute(&pool)
            .await?;
        Ok(())
    }

    ////////

    /// # 3. [REPOSITORY] - 更新/插入点赞记录
    /// * `desc`: `根据 status 值支持正反幂操作：status > 0 执行点赞/恢复，status <= 0 执行取消点赞`
    pub async fn upsert_like(uid: i64, music_id: i64, status: i16) -> Result<(), sqlx::Error> {
        if status > 0 {
            // 正向：执行点赞或恢复
            Self::save_like(uid, music_id).await
        } else {
            // 反向：执行取消点赞
            Self::un_like(uid, music_id).await
        }
    }
}

//////// END