// repository/src/cola_gis/pg/perm_update.rs  -- 仓储 - GIS - pg - 权限更新
// 2026/7/6 14:18

////////

use anyhow::{anyhow, Result};
use crate::pg_pool;

////////

// # [GIS REPOSITORY] - 地理信息服务 权限更新
pub struct GisPermUpdateRepo;

impl GisPermUpdateRepo {
    // 💡

    ////////

    /// # [REPOSITORY] - 更新
    async fn update_perm(gis_id: i64, new_perm: i16, column: &str) -> Result<()> {
        let pool = pg_pool();
        let sql = format!(r#"UPDATE cola_gis.cola_gis SET {} = $1, updated_at = NOW() WHERE id = $2"#, column);
        let result = sqlx::query(&sql).bind(new_perm).bind(gis_id).execute(&pool).await?;
        if result.rows_affected() == 0 { return Err(anyhow!("鏈壘鍒?GIS ID: {}", gis_id)); }
        Ok(())
    }
    pub async fn update_gis_visibility_perm(gis_id: i64, new_perm: i16) -> Result<()> { Self::update_perm(gis_id, new_perm, "visibility_perm").await }
    pub async fn update_gis_comment_perm(gis_id: i64, new_perm: i16) -> Result<()> { Self::update_perm(gis_id, new_perm, "comment_perm").await }
    pub async fn update_gis_danmaku_perm(gis_id: i64, new_perm: i16) -> Result<()> { Self::update_perm(gis_id, new_perm, "danmaku_perm").await }
}

//////// END

