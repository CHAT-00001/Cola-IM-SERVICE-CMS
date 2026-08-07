// repository/src/cola_gis/pg/perm_check.rs  -- 仓储 - GIS - pg - 权限检查
// 2026/7/6 14:17

////////

use crate::pg_pool;
use sqlx;

////////

pub struct GisPermCheckRepo;

impl GisPermCheckRepo {
    pub async fn check_gis_visibility_perm(gis_id: i64) -> Result<i16, sqlx::Error> {
        let pool = pg_pool();
        sqlx::query_scalar(r#"SELECT visibility_perm FROM cola_gis.cola_gis WHERE id = $1 LIMIT 1"#)
            .bind(gis_id).fetch_one(&pool).await
    }
    pub async fn check_gis_comment_perm(gis_id: i64) -> Result<i16, sqlx::Error> {
        let pool = pg_pool();
        sqlx::query_scalar(r#"SELECT comment_perm FROM cola_gis.cola_gis WHERE id = $1 LIMIT 1"#)
            .bind(gis_id).fetch_one(&pool).await
    }
}

//////// END

