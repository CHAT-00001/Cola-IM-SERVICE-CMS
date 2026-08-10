// repo_adapter/src/cola_user/follow/mod.rs
// 🔌 插头 - 可乐用户 - 关注 - 模块
// 2026/8/6 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::port::follow::add::FollowAddPort;
use cola_data::cola_user::port::follow::check::FollowCheckPort;
use cola_data::cola_user::port::follow::del::FollowDelPort;
use cola_data::cola_user::port::follow::get::FollowGetPort;
use cola_data::cola_user::port::follow::list::FollowListPort;
use cola_data::cola_user::port::follow::manage::FollowManagePort;
use port::cola_user::follow::add::UserFollowAddPort;
////////

pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat;
pub(crate) mod get;
// 统计
