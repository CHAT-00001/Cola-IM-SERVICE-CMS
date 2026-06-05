// app/src/db_service.rs  -- 构建所有数据库连接

use sqlx::PgPool;
use redis::{Client as RedisClient, AsyncCommands, RedisError};
use mongodb::Client as MongoClient;
use crate::config::{AppConfig, Pg, Redis, Mongodb};
use tracing::{info, error};
use serde::{Serialize, de::DeserializeOwned};
use std::io;
use redis::aio::MultiplexedConnection;

#[derive(Clone)]
pub struct DbService {
    pub pg_pool: PgPool,
    pub mongo_client: MongoClient,
    pub redis_conn: MultiplexedConnection,
}

impl DbService {
    pub async fn new(config: &AppConfig) -> Option<Self> {
        // PostgreSQL
        let pg_pool = match crate::pg::pg_init(&config.pg).await {
            Ok(pool) => pool,
            Err(e) => {
                error!("Failed to connect PostgreSQL: {:?}", e);
                return None;
            }
        };

        // MongoDB
        let mongo_client = match crate::mongodb::mongodb_init(&config.mongodb).await {
            Ok(client) => client,
            Err(e) => {
                error!("Failed to connect MongoDB: {:?}", e);
                return None;
            }
        };

        let redis_client = crate::redis::redis_init(&config.redis).ok()?;
        let redis_conn = redis_client.get_multiplexed_async_connection().await
            .map_err(|e| { error!("Redis Connection Error: {}", e); e }).ok()?;

        info!("🚀 All database services connected successfully");

        info!("All database connections initialized successfully");

        Some(Self {
            pg_pool,
            // redis_client,
            mongo_client,
            redis_conn,
        })
    }

    /// 从 Redis 获取 JSON 并反序列化
    pub async fn get_redis_json<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, RedisError> {
        let mut conn = self.redis_conn.clone();
        let json_str: Option<String> = conn.get(key).await?;

        match json_str {
            Some(s) => {
                serde_json::from_str(&s).map_err(|e| {
                    // 绕过具体枚举名，直接通过 io::Error 中转
                    // 这是 Rust 处理不同库之间错误转换的“万能钥匙”
                    RedisError::from(std::io::Error::new(std::io::ErrorKind::Other, e))
                }).map(Some)
            },
            None => Ok(None),
        }
    }

    /// 写 JSON 到 Redis 并设置 TTL
    pub async fn set_redis_json<T: Serialize>(&self, key: &str, value: &T, ttl_secs: u64) -> Result<(), RedisError> {
        let mut conn = self.redis_conn.clone();

        let json = serde_json::to_string(value).map_err(|e| {
            RedisError::from(std::io::Error::new(std::io::ErrorKind::Other, e))
        })?;

        // ✅ 修正：只需要提供一个泛型参数 ()
        redis::cmd("SETEX")
            .arg(key)
            .arg(ttl_secs)
            .arg(json)
            .query_async::<()>(&mut conn)
            .await?;

        Ok(())
    }
}
