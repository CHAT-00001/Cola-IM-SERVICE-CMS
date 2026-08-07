// repository/src/cola_gis/service/check.rs
// 服务 - 可乐GIS - 权限 - 模块
// 2026/7/6

////////

use anyhow::Result;
use repository::pg_pool;


////////


/// # [PERMISSIONS CHECK SERVICE] - 权限检查服务
pub struct VideoPermissionsCheckService;


// 构造实现
impl VideoPermissionsCheckService {
    pub async fn check_video_publish_perm(uid: i64) -> Result<()> {
        Ok(())
    }

    pub async fn check_video_visibility_perm(uid: i64, delta: i32) -> Result<()> {
        let pool = pg_pool();
        let sql = r#"SELECT visibility_perm FROM cola_gis.cola_gis WHERE id = $1"#;
        sqlx::query_scalar::<_, i16>(sql).bind(uid).fetch_one(&pool).await?;
        Ok(())
    }
}

//////// END

