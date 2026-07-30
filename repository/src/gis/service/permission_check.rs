// repository/src/gis/service/permission_check.rs -- 鏈嶅姟灞?- GIS 鏉冮檺妫€鏌?
// 2026/7/6

use anyhow::Result;
use crate::pg_pool;

pub struct VideoPermissionsCheckService;

impl VideoPermissionsCheckService {
    pub async fn check_video_publish_perm(uid: i64) -> Result<()> {
        Ok(())
    }

    pub async fn check_video_visibility_perm(uid: i64, delta: i32) -> Result<()> {
        let pool = pg_pool();
        let sql = r#"SELECT visibility_perm FROM cola_gis.gis WHERE id = $1"#;
        sqlx::query_scalar::<_, i16>(sql).bind(uid).fetch_one(&pool).await?;
        Ok(())
    }
}

