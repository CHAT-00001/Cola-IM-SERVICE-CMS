// user/port/follow/mod.rs
// ⏩️ 端口 - 🗣 用户 - 关注 - 模块
// 2026/8/5 21:56 Created.

////////

use crate::cola_user::follow::add::UserFollowAddPort;
use crate::cola_user::follow::check::UserFollowCheckPort;
use crate::cola_user::follow::del::UserFollowDelPort;
use crate::cola_user::follow::get::UserFollowGetPort;
use crate::cola_user::follow::list::UserFollowListPort;
use crate::cola_user::follow::manage::UserFollowManagePort;
use crate::cola_user::follow::stat::UserFollowStatPort;
use async_trait::async_trait;
use std::sync::Arc;

////////

pub mod add; // 添加
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [FOLLOW PORTS]
/// * `desc`: `可乐用户 - 用户关注端口`
#[derive(Clone)]
pub struct UserFollowPort {
    pub add: Arc<dyn UserFollowAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn UserFollowCheckPort + Send + Sync + 'static>,
    pub delete: Arc<dyn UserFollowDelPort + Send + Sync + 'static>,
    pub get: Arc<dyn UserFollowGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn UserFollowListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn UserFollowManagePort + Send + Sync + 'static>,
    pub stat: Arc<dyn UserFollowStatPort + Send + Sync + 'static>,
}

//////// END
