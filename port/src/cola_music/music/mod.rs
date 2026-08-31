// port/src/cola_music/music/mod.rs
// ⏩️ 端口 - MUSIC - 音乐 - mod
// 2026/8/22 23:22 Created.

////////

use crate::cola_music::music::add::MusicAddPort;
use crate::cola_music::music::check::MusicCheckPort;
use crate::cola_music::music::del::MusicDelPort;
use crate::cola_music::music::get::MusicGetPort;
use crate::cola_music::music::list::MusicListPort;
use crate::cola_music::music::manage::MusicManagePort;
use std::sync::Arc;

////////

pub mod add; // 发布与更新
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理

////////

/// # [COLA MUSIC PORT] - 音乐
/// * `desc`: `🎶 可乐音乐 - Cola Music Service Ports`
#[derive(Clone)]
pub struct MusicContentPort {
    pub add: Arc<dyn MusicAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn MusicCheckPort + Send + Sync + 'static>,
    pub del: Arc<dyn MusicDelPort + Send + Sync + 'static>,
    pub get: Arc<dyn MusicGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn MusicListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn MusicManagePort + Send + Sync + 'static>,
}

//////// END
