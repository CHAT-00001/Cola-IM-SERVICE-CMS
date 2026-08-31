// repository/src/cola_gis/pg/like.rs  -- 浠撳偍 - GIS - pg - 鐐硅禐
// 2026/7/6

use crate::pg_pool;
use sqlx;

pub struct LikeRepo;

impl LikeRepo {
    pub async fn pg_save_gis_like(
        uid: i64,
        gis_id: i64,
        is_liked: bool,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let ts = chrono::Utc::now().timestamp();
        let query = r#"
            INSERT INTO cola_gis.gis_like (uid, gis_id, is_liked, addtime, updatetime)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (uid, gis_id) DO UPDATE SET is_liked = EXCLUDED.is_liked, update_time = EXCLUDED.update_time"#;
        sqlx::query(query)
            .bind(uid)
            .bind(gis_id)
            .bind(is_liked)
            .bind(ts)
            .bind(ts)
            .execute(&pool)
            .await?;
        Ok(())
    }

    pub async fn pg_save_gis_unlike(
        uid: i64,
        gis_id: i64,
        is_unliked: bool,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let ts = chrono::Utc::now().timestamp();
        let query = r#"
            INSERT INTO cola_gis.gis_unlike (uid, gis_id, is_unliked, add_time, update_time)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (uid, gis_id) DO UPDATE SET is_unliked = EXCLUDED.is_unliked, update_time = EXCLUDED.update_time"#;
        sqlx::query(query)
            .bind(uid)
            .bind(gis_id)
            .bind(is_unliked)
            .bind(ts)
            .bind(ts)
            .execute(&pool)
            .await?;
        Ok(())
    }

    pub async fn find_like_record_by_user_id(
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();
        let query = "SELECT gis_id FROM cola_gis.gis_like WHERE user_id = $1 AND status = 1 ORDER BY add_time DESC LIMIT $2 OFFSET $3";
        sqlx::query_scalar::<_, i64>(query)
            .bind(uid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    pub async fn find_unlike_record_by_user_id(
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();
        let query = "SELECT gis_id FROM cola_gis.gis_unlike WHERE user_id = $1 AND status = 1 ORDER BY add_time DESC LIMIT $2 OFFSET $3";
        sqlx::query_scalar::<_, i64>(query)
            .bind(uid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }
}
