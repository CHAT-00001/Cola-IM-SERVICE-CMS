// repository/src/cola_gis/redis/cola_gis.rs  -- 浠撳偍 - GIS - Redis - cola_gis
// 2026/7/6

use redis::AsyncCommands;
use app_config::DbService;

#[derive(Clone)]
pub struct GisCache {
    db: DbService,
}

impl GisCache {
    pub fn new(db: DbService) -> Self {
        Self { db }
    }

    pub fn key(gis_id: i64) -> String {
        format!("cola_gis:info:{}", gis_id)
    }

    pub async fn get_gis(&self, gis_id: i64) -> anyhow::Result<Option<String>> {
        let mut conn = self.db.redis_conn.clone();
        let key = Self::key(gis_id);
        let val: Option<String> = conn.get(&key).await?;
        Ok(val)
    }

    pub async fn set_gis(&self, gis_id: i64, value: &str, ttl_secs: usize) -> anyhow::Result<()> {
        let mut conn = self.db.redis_conn.clone();
        let key = Self::key(gis_id);
        let _: () = conn.set_ex(key, value, ttl_secs as u64).await?;
        Ok(())
    }

    pub async fn del_gis(&self, gis_id: i64) -> anyhow::Result<()> {
        let mut conn = self.db.redis_conn.clone();
        let key = Self::key(gis_id);
        let _: () = conn.del(key).await?;
        Ok(())
    }
}

