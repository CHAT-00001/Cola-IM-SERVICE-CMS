// repo/src/gis/pg/ids.rs  -- 浠撳偍 - GIS - PG - IDs
// 2026/7/6

use crate::pg_pool;
use sqlx;

pub struct GisIdsRepo;

impl GisIdsRepo {
    pub async fn find_gis_visite_ids(user_id: i64, limit: i64, offset: i64) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();
        let query = "SELECT id FROM cola_gis.gis_visite WHERE user_id = $1 AND status = 1 ORDER BY add_time DESC LIMIT $2 OFFSET $3";
        sqlx::query_scalar::<_, i64>(query).bind(user_id).bind(limit).bind(offset)
            .fetch_all(&pool).await
    }

    pub async fn find_gis_like_ids(user_id: i64, limit: i64, offset: i64) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();
        let query = "SELECT id FROM cola_gis.gis_like WHERE user_id = $1 AND status = 1 ORDER BY add_time DESC LIMIT $2 OFFSET $3";
        sqlx::query_scalar::<_, i64>(query).bind(user_id).bind(limit).bind(offset)
            .fetch_all(&pool).await
    }

    pub async fn find_gis_collect_ids(user_id: i64, limit: i64, offset: i64) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();
        let query = "SELECT id FROM cola_gis.gis_collect WHERE user_id = $1 AND status = 1 ORDER BY add_time DESC LIMIT $2 OFFSET $3";
        sqlx::query_scalar::<_, i64>(query).bind(user_id).bind(limit).bind(offset)
            .fetch_all(&pool).await
    }
}


