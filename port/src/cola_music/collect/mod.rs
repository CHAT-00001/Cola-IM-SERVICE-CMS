// cola_port/src/music/collect/mod.rs -- 端口 - MUSIC - 收藏 - mod
// 2026/8/24 11:40 Created.

////////

use crate::cola_music::collect::add::MusicCollectAddPort;
use crate::cola_music::collect::check::MusicCollectCheckPort;
use crate::cola_music::collect::delete::MusicCollectDeletePort;
use crate::cola_music::collect::get::MusicCollectGetPort;
use crate::cola_music::collect::list::MusicCollectListPort;
use crate::cola_music::collect::manage::MusicCollectManagePort;
use crate::cola_music::collect::stat::MusicCollectStatPort;
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

/// # [MUSIC COLLECT PORTS] - 音乐 收藏 端口集合
/// * `desc`: `音乐与用户收藏专辑之间的关系端口`
#[derive(Clone)]
pub struct MusicCollectPort {
    pub add: Arc<dyn MusicCollectAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn MusicCollectCheckPort + Send + Sync + 'static>, // 检查
    pub delete: Arc<dyn MusicCollectDeletePort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn MusicCollectGetPort + Send + Sync + 'static>, // 删除
    pub list: Arc<dyn MusicCollectListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn MusicCollectManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn MusicCollectStatPort + Send + Sync + 'static>, // 统计
}

//////// END
