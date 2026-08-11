// repository/src/gis/service/permission/check.rs
// 服务 - 可乐GIS - 权限 - 模块
// 2026/8/11 11:07 Created.

////////

use anyhow::Result;
use crate::pg_pool;

////////

/// # [GIS PERMISSION REPO] - 权限检查仓储
pub struct GisPermissionCheckRepo;

impl GisPermissionCheckRepo {
    /// 获取实体的可见性权限
    pub async fn get_visibility_perm(id: i64) -> Result<i16, sqlx::Error> {
        let pool = pg_pool();
        let sql = r#"SELECT visibility_perm FROM cola_gis.cola_gis WHERE id = $1"#;
        let perm = sqlx::query_scalar::<_, i16>(sql)
            .bind(id)
            .fetch_one(&pool)
            .await?;
        Ok(perm)
    }
}

//////// END


