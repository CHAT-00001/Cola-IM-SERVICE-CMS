// repository/src/lib.rs -- 仓储 - lib
// 2026-04-05 14:02

////////
pub mod auth; // 验证中心
pub mod dynamic; // 动态
pub mod fs; // 文件存储(公共)
pub mod gis; // 地理信息服务
pub mod im; // 即时通讯
pub mod live; // 直播
pub mod market; // 市场
pub mod music; // 音乐
pub mod three; // 三方服务
pub mod user; // 用户中心
pub mod video; // 短视频
pub mod wallet; // 钱包

// 统一为整个 repository 层提供一个获取 PG 连接池的便捷函数
pub fn pg_pool() -> sqlx::PgPool {
    app_config::GLOBAL_DB
        .get()
        .expect("❌ GLOBAL_DB 未初始化，请在程序启动时先调用 init_global_db")
        .pg_pool
        .clone()
}
