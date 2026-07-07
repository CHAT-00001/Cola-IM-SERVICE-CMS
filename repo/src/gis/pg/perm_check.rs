// repo/src/gis/pg/perm_check.rs  -- 浠撳偍 - GIS - PG - 鏉冮檺妫€鏌?
// 2026/7/6

use crate::pg_pool;
use sqlx;

pub struct GisPermCheckRepo;

impl GisPermCheckRepo {
    pub async fn check_gis_visibility_perm(gis_id: i64) -> Result<i16, sqlx::Error> {
        let pool = pg_pool();
        sqlx::query_scalar(r#"SELECT visibility_perm FROM cola_gis.gis WHERE id = $1 LIMIT 1"#)
            .bind(gis_id).fetch_one(&pool).await
    }
    pub async fn check_gis_comment_perm(gis_id: i64) -> Result<i16, sqlx::Error> {
        let pool = pg_pool();
        sqlx::query_scalar(r#"SELECT comment_perm FROM cola_gis.gis WHERE id = $1 LIMIT 1"#)
            .bind(gis_id).fetch_one(&pool).await
    }
}


