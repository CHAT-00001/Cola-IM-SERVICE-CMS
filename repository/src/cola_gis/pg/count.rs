// repository/src/cola_gis/pg/count.rs  -- 仓储 - GIS - PG - 璁℃暟
// 2026/7/6 14:01

////////

use crate::pg_pool;
use sqlx;

////////

pub struct CountRepo;

impl CountRepo {
    pub async fn pg_update_gis_views(gis_id: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let query = r#"INSERT INTO cola_gis.gis_count (gis_id, views) VALUES ($1, 1) ON CONFLICT (gis_id) DO UPDATE SET views = cola_gis.gis_count.views + 1"#;
        sqlx::query(query).bind(gis_id).execute(&pool).await?;
        Ok(())
    }

    pub async fn pg_update_gis_comments(gis_id: i64, increment: i16) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let query = r#"INSERT INTO cola_gis.gis_count (gis_id, comments) VALUES ($1, GREATEST(0, $2::INT)) ON CONFLICT (gis_id) DO UPDATE SET comments = GREATEST(0, cola_gis.gis_count.comments + $2::INT)"#;
        sqlx::query(query).bind(gis_id).bind(increment as i32).execute(&pool).await?;
        Ok(())
    }

    pub async fn pg_update_gis_likes(gis_id: i64, increment: i16) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let query = r#"INSERT INTO cola_gis.gis_count (gis_id, likes) VALUES ($1, GREATEST(0, $2::INT)) ON CONFLICT (gis_id) DO UPDATE SET likes = GREATEST(0, cola_gis.gis_count.likes + $2::INT)"#;
        sqlx::query(query).bind(gis_id).bind(increment as i32).execute(&pool).await?;
        Ok(())
    }

    pub async fn pg_update_gis_collects(gis_id: i64, increment: i16) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let query = r#"INSERT INTO cola_gis.gis_count (gis_id, collects) VALUES ($1, GREATEST(0, $2::INT)) ON CONFLICT (gis_id) DO UPDATE SET collects = GREATEST(0, cola_gis.gis_count.collects + $2::INT)"#;
        sqlx::query(query).bind(gis_id).bind(increment as i32).execute(&pool).await?;
        Ok(())
    }

    pub async fn pg_update_gis_shares(gis_id: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let query = r#"INSERT INTO cola_gis.gis_count (gis_id, shares) VALUES ($1, 1) ON CONFLICT (gis_id) DO UPDATE SET shares = cola_gis.gis_count.shares + 1"#;
        sqlx::query(query).bind(gis_id).execute(&pool).await?;
        Ok(())
    }
}

//////// END


