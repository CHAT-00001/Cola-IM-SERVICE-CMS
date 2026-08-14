// repository/src/lib.rs
// 仓储 - lib
// 2026-04-05 14:02

extern crate core;

////////
pub mod cola_asset; // 可乐资产
pub mod auth; // 可乐验证中心
pub mod cola_dynamic; // 快乐动态
pub mod cola_finance; // 可乐财务
pub mod cola_fs; // 可怜文件存储(公共)
pub mod cola_gis; // 可乐地理信息服务
pub mod cola_im; // 可乐即时通讯
pub mod cola_live; // 可乐直播
pub mod cola_market; // 可乐市场
pub mod cola_music; // 可乐音乐
pub mod cola_three; // 可乐三方服务
pub mod user; // 可乐用户中心
pub mod cola_video; // 可乐短视频

////////

// 统一为整个 repository 层提供一个获取 PG 连接池的便捷函数
pub fn pg_pool() -> sqlx::PgPool {
    app_config::GLOBAL_DB
        .get()
        .expect("❌ GLOBAL_DB 未初始化，请在程序启动时先调用 init_global_db")
        .pg_pool
        .clone()
}

//////// END
