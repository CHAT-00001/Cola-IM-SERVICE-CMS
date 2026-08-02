// app/src/db.rs -- 应用配置 - 数据库连接池
// 2025-12-16 10:50

////////

use crate::config::AppConfig;
use mongodb::Client as MongoClient;
use mongodb::mongodb_init;
use pg::pg_init;
use redis::Client as RedisClient;
use redis::redis_init;
use sqlx::PgPool;
use tracing::error;

////////
pub mod mongodb;
pub mod pg;
pub mod redis;

////////

/// # [CONN] - 封装后的数据库连接结构体
pub struct DbConnection {
    pub pg_pool: PgPool,           // PG
    pub redis_client: RedisClient, // REDIS
    pub mongo_client: MongoClient, // MONGODB
}

/// # [INIT] - 初始化所有数据库连接
/// * 返回 Result<DbConnection, ()>，初始化失败打印错误并返回 Err
pub async fn db_connection(config: &AppConfig) -> Result<DbConnection, ()> {
    // --- PostgreSQL ---
    let pg_pool = match pg_init(&config.pg).await {
        Ok(pool) => pool,
        Err(e) => {
            error!("PostgreSQL initialization failed: {:?}", e);
            return Err(());
        }
    };

    // --- Redis ---
    let redis_client = match redis_init(&config.redis) {
        Ok(client) => client,
        Err(e) => {
            error!("Redis initialization failed: {:?}", e);
            return Err(());
        }
    };

    // --- MongoDB ---
    let mongo_client = match mongodb_init(&config.mongodb).await {
        Ok(client) => client,
        Err(e) => {
            error!("MongoDB initialization failed: {:?}", e);
            return Err(());
        }
    };

    Ok(DbConnection {
        pg_pool,
        redis_client,
        mongo_client,
    })
}

//////// END
