// repository/src/gis/redis/visited.rs -- 浠撳偍 - GIS - Redis - visited
// 2026/7/6

use redis::AsyncCommands;
use app_config::DbService;

#[derive(Clone)]
pub struct VisitedCache {
    db: DbService,
}

impl VisitedCache {
    pub fn new(db: DbService) -> Self {
        Self { db }
    }

    fn key(user_id: i64) -> String {
        format!("gis:visited:{}", user_id)
    }

    pub async fn add_visited(&self, user_id: i64, gis_id: i64, ts: i64) -> anyhow::Result<()> {
        let mut conn = self.db.redis_conn.clone();
        let key = Self::key(user_id);
        let _: () = conn.zadd(key, gis_id, ts as f64).await?;
        Ok(())
    }

    pub async fn get_user_gis_ids(&self, user_id: i64, offset: i64, limit: i64) -> anyhow::Result<Vec<i64>> {
        let mut conn = self.db.redis_conn.clone();
        let key = Self::key(user_id);
        let ids: Vec<i64> = conn.zrevrange(key, offset as isize, (offset + limit - 1) as isize).await?;
        Ok(ids)
    }

    pub async fn remove(&self, user_id: i64, gis_id: i64) -> anyhow::Result<()> {
        let mut conn = self.db.redis_conn.clone();
        let key = Self::key(user_id);
        let _: () = conn.zrem(key, gis_id).await?;
        Ok(())
    }
}

