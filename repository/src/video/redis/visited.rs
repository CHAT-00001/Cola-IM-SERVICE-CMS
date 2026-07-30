// repository/src/video/redis/visited.rs
// 2026/6/8 23:03

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

    // =========================
    // 1. 记录用户看过的视频
    // =========================
    pub async fn add_visited(
        &self,
        user_id: i64,
        video_id: i64,
        ts: i64,
    ) -> anyhow::Result<()> {

        let mut conn = self.db.redis_conn.clone();
        let key = Self::key(user_id);

        // ⭐ 关键修复：score 必须显式转 f64（Redis ZSET 标准）
        let _: () = conn
            .zadd(key, video_id, ts as f64)
            .await?;

        Ok(())
    }

    // =========================
    // 2. 获取看过的视频 ids（分页）
    // =========================
    pub async fn get_user_video_ids(
        &self,
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<i64>> {

        let mut conn = self.db.redis_conn.clone();
        let key = Self::key(user_id);

        let start = offset;
        let end = offset + limit - 1;

        // ✔ 标准：按时间倒序
        let ids: Vec<i64> = conn
            .zrevrange(key, start as isize, end as isize)
            .await?;

        Ok(ids)
    }

    // =========================
    // 3. 删除某个视频记录
    // =========================
    pub async fn remove(
        &self,
        user_id: i64,
        video_id: i64,
    ) -> anyhow::Result<()> {

        let mut conn = self.db.redis_conn.clone();
        let key = Self::key(user_id);

        let _: () = conn.zrem(key, video_id).await?;

        Ok(())
    }

    // =========================
    // 4. key 统一管理
    // =========================
    fn key(user_id: i64) -> String {
        format!("visited:user:{}", user_id)
    }
}