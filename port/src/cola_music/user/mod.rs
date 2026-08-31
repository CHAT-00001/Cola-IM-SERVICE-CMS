// port/src/cola_music/user/mod.rs -- 端口 - MUSIC - 用户 - mod
// 2026/8/22 23:22 Created.

////////

use crate::cola_music::user::add::MusicUserAddPort;
use crate::cola_music::user::check::MusicUserCheckPort;
use crate::cola_music::user::del::MusicUserDelPort;
use crate::cola_music::user::get::MusicUserGetPort;
use crate::cola_music::user::list::MusicUserListPort;
use crate::cola_music::user::manage::MusicUserManagePort;
use crate::cola_music::user::stat::MusicUserStatPort;
use std::sync::Arc;

////////

pub mod add; // 发布与更新
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [COLA MUSIC USER PORTS] - 音乐用户
/// * `desc`: `🎶 可乐音乐 - Cola Music User Service Ports`
#[derive(Clone)]
pub struct MusicUserPort {
    pub add: Arc<dyn MusicUserAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn MusicUserCheckPort + Send + Sync + 'static>,
    pub del: Arc<dyn MusicUserDelPort + Send + Sync + 'static>,
    pub get: Arc<dyn MusicUserGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn MusicUserListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn MusicUserManagePort + Send + Sync + 'static>,
    pub stat: Arc<dyn MusicUserStatPort + Send + Sync + 'static>,
}

//////// END
