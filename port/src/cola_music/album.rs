// port/src/music/album.rs
// ⏩️ 端口 - 可乐音乐 - 专辑 - mod
// 2026/8/22 23:01 Created.

////////
use crate::cola_music::album::add::MusicAlbumAddPort;
use crate::cola_music::album::check::MusicAlbumCheckPort;
use crate::cola_music::album::delete::MusicAlbumDeletePort;
use crate::cola_music::album::get::MusicAlbumGetPort;
use crate::cola_music::album::list::MusicAlbumListPort;
use crate::cola_music::album::manage::MusicAlbumManagePort;
use crate::cola_music::album::stat::MusicAlbumStatPort;
use std::sync::Arc;

////////

pub mod add;
pub mod check;
pub mod delete;
pub mod get;
pub mod list;
pub mod manage;
pub mod stat;

////////

/// # [MUSIC ALBUM PORTS] - 音乐专辑端口集合
/// * `desc`: `用户创建并管理可公开展示的音乐专辑`
#[derive(Clone)]
pub struct MusicAlbumPort {
    pub add: Arc<dyn MusicAlbumAddPort + Send + Sync + 'static>,       // 发布
    pub check: Arc<dyn MusicAlbumCheckPort + Send + Sync + 'static>,   // 检查
    pub delete: Arc<dyn MusicAlbumDeletePort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn MusicAlbumGetPort + Send + Sync + 'static>,       // 获取
    pub list: Arc<dyn MusicAlbumListPort + Send + Sync + 'static>,     // 列表
    pub manage: Arc<dyn MusicAlbumManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn MusicAlbumStatPort + Send + Sync + 'static>,     // 统计
}

//////// END
