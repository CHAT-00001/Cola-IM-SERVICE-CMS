// repo lib.rs
pub mod auth;
pub mod dynamic;
pub mod gis;
pub mod live;
pub mod market;
pub mod three;
pub mod user;
pub mod video;

// repo/src/lib.rs

// 统一为整个 repo 层提供一个获取 PG 连接池的便捷函数
pub fn pg_pool() -> sqlx::PgPool {
    app_config::GLOBAL_DB
        .get()
        .expect("❌ GLOBAL_DB 未初始化，请在程序启动时先调用 init_global_db")
        .pg_pool
        .clone()
}
