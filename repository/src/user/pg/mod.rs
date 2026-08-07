// repository/src/user/pg/mod.rs
// 仓储中心 - 用户 - pg - mod
// 2026/5/23 04:59 Created.

////////

pub mod ban; // 封禁
pub mod black; // 黑名单
pub mod black_repo;
pub mod category; // 分类
pub mod follow; // 关注
pub mod follow_repo;
pub mod friend; // 朋友
pub mod home_repo;
pub mod profile; // 资料
pub mod role; // 角色
pub mod state_repo;
pub mod user; // 用户(主内容)
mod view;
pub mod view_repo;
pub mod vip; // 贵宾 // 浏览
