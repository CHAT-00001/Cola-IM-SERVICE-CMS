// network/src/qiniu/client.rs  -- 七牛云客户端
// 2026-02-07 18:44:01

use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use sqlx::{FromRow, PgPool};
use anyhow::Result;

/// CDN 服务
pub struct QiniuConfig<'a> {
    redis: &'a mut MultiplexedConnection,
    pg_pool: &'a PgPool,
    ttl_sec: usize,
}
#[derive(Debug, FromRow)]
struct CdnRecord {
    domain: String,
}
impl<'a> QiniuConfig<'a> {
    /// 创建服务
    pub fn new(redis: &'a mut MultiplexedConnection, pg_pool: &'a PgPool, ttl_sec: usize) -> Self {
        Self { redis, pg_pool, ttl_sec }
    }

    /// 获取 CDN 域名
    pub async fn get_cdn(&mut self, name: &str) -> Result<String> {
        let key = format!("cdn:{}", name);

        // 1️⃣ 尝试 Redis
        let domain_opt: Option<String> = self.redis.get(&key).await.ok();
        if let Some(domain) = domain_opt {
            return Ok(domain);
        }

        // 2️⃣ Redis 没有，从 DB 查
        let domain = self.fetch_cdn_from_db(name).await?;

        // 3️⃣ 写入 Redis，带 TTL
        let _: () = self.redis.set_ex(&key, &domain, self.ttl_sec as u64).await?;

        Ok(domain)
    }

    /// 数据库查询 CDN 域名
    async fn fetch_cdn_from_db(&self, name: &str) -> Result<String> {
        // 关键：使用 query_as 而非 query!，关闭编译时SQL检查
        let rec: CdnRecord = sqlx::query_as(
            "SELECT domain FROM cdn_config WHERE name = $1 AND is_active = true LIMIT 1"
        )
            .bind(name)
            .fetch_one(self.pg_pool)
            .await?;

        Ok(rec.domain)
    }

    /// 管理员更新后刷新缓存
    pub async fn refresh_cdn(&mut self, name: &str) -> Result<()> {
        let domain = self.fetch_cdn_from_db(name).await?;
        let key = format!("cdn:{}", name);
        let ttl: u64 = self.ttl_sec.try_into().unwrap();
        let _: () = self.redis.set_ex(&key, &domain, ttl).await?;
        Ok(())
    }
}